// Standard library imports for file I/O and timing
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

// External crates for error handling, CLI parsing, colors, CSV, async, and HTTP
use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use csv::Writer;
use serde_json;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Serialize;
use tokio::time::Duration;

/// Command-line arguments structure
/// Uses clap for automatic argument parsing and help generation
#[derive(Parser, Debug)]
#[command(author, version, about = "An asynchronous URL checker in Rust", long_about = None)]
struct Args {
    /// Input file containing URLs to check (one URL per line)
    #[arg(short, long, default_value = "urls.txt")]
    input: String,

    /// Output CSV file path for saving results
    #[arg(short, long, default_value = "report.csv")]
    output: String,

    /// Export format: csv or json
    #[arg(short, long, default_value = "csv")]
    format: String,

    /// Number of concurrent HTTP requests to make simultaneously
    /// Higher values = faster checking but more resource usage
    #[arg(short, long, default_value_t = 20)]
    concurrency: usize,

    /// Request timeout in seconds for each URL check
    /// Requests taking longer than this will be marked as failed
    #[arg(short, long, default_value_t = 10)]
    timeout: u64,
}

/// Structure representing a single URL check result
/// Serialized to CSV format for reporting
#[derive(Debug, Serialize, Clone)]
struct ResultRow {
    url: String,              // The URL that was checked
    status: String,           // HTTP status code (e.g., "200", "404", "ERROR")
    reason: String,           // HTTP status reason phrase (e.g., "OK", "Not Found")
    time_ms: u128,            // Response time in milliseconds
    size_bytes: u64,          // Response body size in bytes (if available)
    timestamp: String,        // UTC timestamp when the check was performed
}

/// Statistics aggregated from all URL checks
/// Used for generating summary reports
struct Stats {
    total: usize,        // Total number of URLs checked
    up: usize,           // Number of successful checks (2xx/3xx status codes)
    down: usize,         // Number of failed checks (4xx/5xx/errors)
    total_time: u128,    // Sum of all response times (for calculating average)
    min_time: u128,      // Fastest response time encountered
    max_time: u128,      // Slowest response time encountered
    total_size: u64,     // Total bytes received across all requests
}

/// Main entry point for the URL checker application
/// Orchestrates the entire URL checking workflow:
/// 1. Parse command-line arguments
/// 2. Read URLs from input file
/// 3. Create HTTP client with configured timeout
/// 4. Check all URLs concurrently with progress tracking
/// 5. Display results in a formatted table
/// 6. Generate CSV report and statistics
#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments using clap
    let args = Args::parse();
    
    // Display professional header with configuration
    print_header(&args);

    // Read URLs from input file, filtering out empty lines
    let urls = read_lines(&args.input)
        .with_context(|| format!("Failed to read file {}", &args.input))?
        .into_iter()
        .filter_map(|l| {
            let s = l.trim().to_string();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        })
        .collect::<Vec<_>>();

    // Validate that we have URLs to check
    if urls.is_empty() {
        eprintln!("{} File {} is empty or contains no URLs. Exiting.", "✗".red(), &args.input);
        return Ok(());
    }

    println!("{} Found {} URL(s) to check\n", "ℹ".cyan(), urls.len().to_string().bold());

    // Build HTTP client with configured timeout and user agent
    // Using rustls instead of OpenSSL for better cross-platform compatibility
    let client = Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .user_agent("url-checker/0.2")
        .build()?;

    // Initialize progress bar with custom styling
    // Shows spinner, elapsed time, progress bar, percentage, and ETA
    let pb = ProgressBar::new(urls.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );
    pb.set_message("Checking URLs...");

    // Process all URLs concurrently using async streams
    // buffer_unordered allows up to 'concurrency' requests at once
    // Each URL check runs in parallel, updating the progress bar as it completes
    let results = stream::iter(urls.into_iter().map(|url| {
        let client = client.clone();
        let pb = pb.clone();
        async move {
            let res = check_url(client, url).await;
            pb.inc(1);  // Increment progress bar
            res
        }
    }))
    .buffer_unordered(args.concurrency)  // Limit concurrent requests
    .collect::<Vec<_>>()
    .await;

    pb.finish_with_message("✓ Complete");

    // Collect all results for export
    let mut all_results = Vec::new();

    // Initialize statistics tracking
    let mut stats = Stats {
        total: 0,
        up: 0,
        down: 0,
        total_time: 0,
        min_time: u128::MAX,  // Start with max value to find minimum
        max_time: 0,
        total_size: 0,
    };

    // Print formatted table header for results
    println!("\n{}", "─".repeat(100).bright_black());
    println!("{:<50} {:<8} {:<12} {:<10} {}", 
        "URL".bold(), 
        "STATUS".bold(), 
        "TIME (ms)".bold(), 
        "SIZE".bold(),
        "RESULT".bold()
    );
    println!("{}", "─".repeat(100).bright_black());

    for r in results {
        match r {
            Ok(row) => {
                all_results.push(row.clone());
                stats.total += 1;
                stats.total_time += row.time_ms;
                stats.total_size += row.size_bytes;
                
                if row.time_ms < stats.min_time {
                    stats.min_time = row.time_ms;
                }
                if row.time_ms > stats.max_time {
                    stats.max_time = row.time_ms;
                }

                let (status_color, status_icon, result_text) = if row.status.starts_with('2') {
                    (row.status.green().bold(), "✓".green(), "OK".green())
                } else if row.status.starts_with('3') {
                    (row.status.yellow().bold(), "↻".yellow(), "REDIRECT".yellow())
                } else if row.status.starts_with('4') {
                    (row.status.red().bold(), "✗".red(), "CLIENT ERROR".red())
                } else if row.status.starts_with('5') {
                    (row.status.red().bold(), "✗".red(), "SERVER ERROR".red())
                } else {
                    (row.status.normal(), "?".normal(), "UNKNOWN".normal())
                };

                let size_str = format_size(row.size_bytes);
                let url_display = if row.url.len() > 48 {
                    format!("{}...", &row.url[..45])
                } else {
                    row.url.clone()
                };

                println!("{:<50} {:<8} {:<12} {:<10} {} {}",
                    url_display,
                    status_color,
                    format!("{}", row.time_ms).bright_white(),
                    size_str.bright_white(),
                    status_icon,
                    result_text
                );

                if row.status.starts_with('2') || row.status.starts_with('3') {
                    stats.up += 1;
                } else {
                    stats.down += 1;
                }
            }
            Err((url, err_msg)) => {
                let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
                let error_row = ResultRow {
                    url: url.clone(),
                    status: "ERROR".to_string(),
                    reason: err_msg.clone(),
                    time_ms: 0,
                    size_bytes: 0,
                    timestamp,
                };
                all_results.push(error_row);
                
                let url_display = if url.len() > 48 {
                    format!("{}...", &url[..45])
                } else {
                    url.clone()
                };

                println!("{:<50} {:<8} {:<12} {:<10} {} {}",
                    url_display,
                    "ERROR".red().bold(),
                    "N/A".bright_black(),
                    "N/A".bright_black(),
                    "✗".red(),
                    "FAILED".red()
                );
                
                stats.total += 1;
                stats.down += 1;
            }
        }
    }

    // Export results based on format
    match args.format.to_lowercase().as_str() {
        "json" => {
            // Export to JSON format
            let json_data = serde_json::json!({
                "metadata": {
                    "total_urls": stats.total,
                    "successful": stats.up,
                    "failed": stats.down,
                    "avg_time_ms": if stats.up > 0 { stats.total_time / stats.up as u128 } else { 0 },
                    "min_time_ms": if stats.min_time != u128::MAX { stats.min_time } else { 0 },
                    "max_time_ms": stats.max_time,
                    "total_size_bytes": stats.total_size,
                    "generated_at": chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
                },
                "results": all_results
            });
            std::fs::write(&args.output, serde_json::to_string_pretty(&json_data)?)
                .with_context(|| format!("Could not write JSON to {}", &args.output))?;
        }
        _ => {
            // Default: Export to CSV format
            let file = File::create(&args.output)
                .with_context(|| format!("Could not create {} for writing", &args.output))?;
            let mut wtr = Writer::from_writer(file);
            for row in &all_results {
                wtr.serialize(row)?;
            }
            wtr.flush()?;
        }
    }
    
    // Print statistics
    print_statistics(&stats, &args.output);
    
    Ok(())
}

/// Reads a file line by line and returns a vector of non-empty strings
/// Filters out empty lines and trims whitespace
/// 
/// # Arguments
/// * `filename` - Path to the file to read
/// 
/// # Returns
/// * `Result<Vec<String>>` - Vector of URL strings or an error
fn read_lines<P>(filename: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().filter_map(|l| l.ok()).collect())
}

/// Checks a single URL by sending an HTTP GET request
/// Measures response time and extracts status information
/// 
/// # Arguments
/// * `client` - Reusable HTTP client instance
/// * `url` - URL string to check
/// 
/// # Returns
/// * `Ok(ResultRow)` - Success with check results
/// * `Err((String, String))` - Error with URL and error message
async fn check_url(client: Client, url: String) -> Result<ResultRow, (String, String)> {
    // Start timing the request
    let start = Instant::now();
    
    // Send the HTTP GET request asynchronously
    let resp = client.get(&url).send().await;
    
    // Calculate elapsed time in milliseconds
    let elapsed = start.elapsed().as_millis();

    match resp {
        Ok(r) => {
            // Extract HTTP status code and reason phrase
            let status = r.status().as_u16().to_string();
            let reason = r.status().canonical_reason().unwrap_or("").to_string();
            
            // Try to get content length from response headers
            // Some servers don't send Content-Length, so default to 0
            let size_bytes = r.content_length().unwrap_or(0);
            
            // Generate UTC timestamp for this check
            let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
            
            Ok(ResultRow {
                url,
                status,
                reason,
                time_ms: elapsed,
                size_bytes,
                timestamp,
            })
        }
        Err(e) => {
            // Return error with URL and error message for logging
            Err((url, format!("{}", e)))
        }
    }
}

/// Prints a professional header banner with configuration information
/// Displays input file, output file, concurrency, and timeout settings
/// 
/// # Arguments
/// * `args` - Command-line arguments containing configuration
fn print_header(args: &Args) {
    println!("\n{}", "═".repeat(100).bright_blue().bold());
    println!("{}", "  URL CHECKER - Professional Web Status Monitor".bright_cyan().bold());
    println!("{}", "═".repeat(100).bright_blue().bold());
    println!("{} Input file:  {}", "•".bright_cyan(), args.input.bright_white());
    println!("{} Output file: {}", "•".bright_cyan(), args.output.bright_white());
    println!("{} Concurrency: {}", "•".bright_cyan(), args.concurrency.to_string().bright_white());
    println!("{} Timeout:     {}s", "•".bright_cyan(), args.timeout.to_string().bright_white());
    println!("{}", "═".repeat(100).bright_blue().bold());
}

/// Prints comprehensive statistics after all URL checks are complete
/// Displays success rates, response times, data transfer, and output file location
/// 
/// # Arguments
/// * `stats` - Aggregated statistics from all URL checks
/// * `output_file` - Path to the CSV report file
fn print_statistics(stats: &Stats, output_file: &str) {
    println!("{}", "─".repeat(100).bright_black());
    println!("\n{}", "📊 STATISTICS".bright_cyan().bold());
    println!("{}", "─".repeat(100).bright_black());
    
    let success_rate = if stats.total > 0 {
        (stats.up as f64 / stats.total as f64) * 100.0
    } else {
        0.0
    };
    
    let avg_time = if stats.up > 0 {
        stats.total_time / stats.up as u128
    } else {
        0
    };

    println!("{} Total URLs checked:    {}", "  •".bright_cyan(), stats.total.to_string().bold().white());
    println!("{} Successful (2xx/3xx):  {}", "  •".bright_cyan(), format!("{} ({:.1}%)", stats.up, success_rate).green().bold());
    println!("{} Failed/Errors:         {}", "  •".bright_cyan(), format!("{} ({:.1}%)", stats.down, 100.0 - success_rate).red().bold());
    println!();
    
    if stats.min_time != u128::MAX {
        println!("{} Average response time: {}", "  •".bright_cyan(), format!("{} ms", avg_time).bright_white().bold());
        println!("{} Fastest response:     {}", "  •".bright_cyan(), format!("{} ms", stats.min_time).green().bold());
        println!("{} Slowest response:     {}", "  •".bright_cyan(), format!("{} ms", stats.max_time).red().bold());
    } else {
        println!("{} Average response time: {}", "  •".bright_cyan(), "N/A".bright_black());
        println!("{} Fastest response:     {}", "  •".bright_cyan(), "N/A".bright_black());
        println!("{} Slowest response:     {}", "  •".bright_cyan(), "N/A".bright_black());
    }
    println!("{} Total data received:  {}", "  •".bright_cyan(), format_size(stats.total_size).bright_white().bold());
    println!();
    println!("{} Report saved to:      {}", "  •".bright_cyan(), output_file.bright_white().bold());
    println!("{}", "─".repeat(100).bright_black());
    println!();
}

/// Formats a byte count into a human-readable string
/// Converts bytes to KB, MB, or GB as appropriate
/// 
/// # Arguments
/// * `bytes` - Number of bytes to format
/// 
/// # Returns
/// * `String` - Formatted size string (e.g., "1.5 MB", "512 B")
fn format_size(bytes: u64) -> String {
    if bytes == 0 {
        return "N/A".to_string();
    }
    
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}
