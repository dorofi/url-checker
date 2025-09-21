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
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

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
        println!("File {} is empty or contains no URLs. Exiting.", &args.input);
        return Ok(());
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(args.timeout))
        .user_agent("url-checker/0.2")
        .build()?;

    // Progress bar
    let pb = ProgressBar::new(urls.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap(),
    );

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

    pb.finish_with_message("Check complete");

    // Write to CSV
    let mut wtr = Writer::from_path(&args.output)
        .with_context(|| format!("Could not open {} for writing", &args.output))?;
    wtr.write_record(&["url", "status", "reason", "time_ms"])?;

    let mut total = 0usize;
    let mut up = 0usize;
    let mut down = 0usize;

    for r in results {
        match r {
            Ok(row) => {
                wtr.serialize(&row)?;
                total += 1;
                if row.status.starts_with('2') || row.status.starts_with('3') {
                    println!("{} {} ({} ms)", row.url, row.status.green(), row.time_ms);
                    up += 1;
                } else {
                    println!("{} {} ({} ms)", row.url, row.status.red(), row.time_ms);
                    down += 1;
                }
            }
            Err((url, err_msg)) => {
                wtr.serialize(&ResultRow {
                    url: url.clone(),
                    status: "ERROR".to_string(),
                    reason: err_msg.clone(),
                    time_ms: 0,
                })?;
                println!("{} {}", url, "ERROR".red());
                total += 1;
                down += 1;
            }
        }
    }

    wtr.flush()?;
    println!(
        "\nDone. total: {}, {} up, {} down. Report: {}",
        total,
        up.to_string().green(),
        down.to_string().red(),
        args.output
    );
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
            Ok(ResultRow {
                url,
                status,
                reason,
                time_ms: elapsed,
            })
        }
        Err(e) => Err((url, format!("{}", e))),
    }
}
