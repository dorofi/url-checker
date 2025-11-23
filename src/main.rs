use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use csv::Writer;
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Serialize;
use tokio::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about = "An asynchronous URL checker in Rust", long_about = None)]
struct Args {
    /// File with a list of URLs (one per line)
    #[arg(short, long, default_value = "urls.txt")]
    input: String,

    /// Output CSV file
    #[arg(short, long, default_value = "report.csv")]
    output: String,

    /// Concurrency (how many requests at once)
    #[arg(short, long, default_value_t = 20)]
    concurrency: usize,

    /// Timeout in seconds for each request
    #[arg(short, long, default_value_t = 10)]
    timeout: u64,
}

#[derive(Debug, Serialize)]
struct ResultRow {
    url: String,
    status: String,
    reason: String,
    time_ms: u128,
    size_bytes: u64,
    timestamp: String,
}

struct Stats {
    total: usize,
    up: usize,
    down: usize,
    total_time: u128,
    min_time: u128,
    max_time: u128,
    total_size: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Print professional header
    print_header(&args);

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

    if urls.is_empty() {
        eprintln!("{} File {} is empty or contains no URLs. Exiting.", "✗".red(), &args.input);
        return Ok(());
    }

    println!("{} Found {} URL(s) to check\n", "ℹ".cyan(), urls.len().to_string().bold());

    let client = Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .user_agent("url-checker/0.2")
        .build()?;

    // Progress bar with better styling
    let pb = ProgressBar::new(urls.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );
    pb.set_message("Checking URLs...");

    // Asynchronously check all URLs
    let results = stream::iter(urls.into_iter().map(|url| {
        let client = client.clone();
        let pb = pb.clone();
        async move {
            let res = check_url(client, url).await;
            pb.inc(1);
            res
        }
    }))
    .buffer_unordered(args.concurrency)
    .collect::<Vec<_>>()
    .await;

    pb.finish_with_message("✓ Complete");

    // Process results and collect stats
    let file = File::create(&args.output)
        .with_context(|| format!("Could not create {} for writing", &args.output))?;
    let mut wtr = Writer::from_writer(file);

    let mut stats = Stats {
        total: 0,
        up: 0,
        down: 0,
        total_time: 0,
        min_time: u128::MAX,
        max_time: 0,
        total_size: 0,
    };

    // Print table header
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
                wtr.serialize(&row)?;
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
                wtr.serialize(&ResultRow {
                    url: url.clone(),
                    status: "ERROR".to_string(),
                    reason: err_msg.clone(),
                    time_ms: 0,
                    size_bytes: 0,
                    timestamp,
                })?;
                
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

    wtr.flush()?;
    
    // Print statistics
    print_statistics(&stats, &args.output);
    
    Ok(())
}

// read file into a list of strings
fn read_lines<P>(filename: P) -> Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(&filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().filter_map(|l| l.ok()).collect())
}

// check a single url
async fn check_url(client: Client, url: String) -> Result<ResultRow, (String, String)> {
    let start = Instant::now();
    let resp = client.get(&url).send().await;
    let elapsed = start.elapsed().as_millis();

    match resp {
        Ok(r) => {
            let status = r.status().as_u16().to_string();
            let reason = r.status().canonical_reason().unwrap_or("").to_string();
            
            // Try to get content length
            let size_bytes = r.content_length().unwrap_or(0);
            
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
        Err(e) => Err((url, format!("{}", e))),
    }
}

// Print professional header
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

// Print statistics
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

// Format file size
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
