// Prevents additional console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::time::Instant;
use tokio::time::Duration;

/// Structure representing a single URL check result
#[derive(Debug, Serialize, Deserialize)]
struct UrlResult {
    url: String,
    status: String,
    reason: String,
    time_ms: u128,
    size_bytes: u64,
    timestamp: String,
    success: bool,
}

/// Structure for check request from frontend
#[derive(Debug, Deserialize)]
struct CheckRequest {
    urls: Vec<String>,
    timeout: u64,
    concurrency: usize,
}

/// Structure for check response
#[derive(Debug, Serialize)]
struct CheckResponse {
    results: Vec<UrlResult>,
    stats: Stats,
}

/// Statistics structure
#[derive(Debug, Serialize)]
struct Stats {
    total: usize,
    up: usize,
    down: usize,
    avg_time: u128,
    min_time: u128,
    max_time: u128,
    total_size: u64,
}

/// Main Tauri command: Check URLs
/// This function is called from the frontend JavaScript
#[tauri::command]
async fn check_urls(request: CheckRequest) -> Result<CheckResponse, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(request.timeout))
        .user_agent("url-checker-gui/0.1.0")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let mut results = Vec::new();
    let mut stats = Stats {
        total: 0,
        up: 0,
        down: 0,
        avg_time: 0,
        min_time: u128::MAX,
        max_time: 0,
        total_size: 0,
    };

    // Use semaphore to limit concurrency
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(request.concurrency));
    let mut handles = Vec::new();

    for url in request.urls {
        let client = client.clone();
        let permit = semaphore.clone();
        let handle = tokio::spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            check_single_url(client, url).await
        });
        handles.push(handle);
    }

    // Wait for all checks to complete
    for handle in handles {
        match handle.await {
            Ok(result) => {
                let result = result?;
                stats.total += 1;
                stats.total_size += result.size_bytes;
                
                if result.success {
                    stats.up += 1;
                    if result.time_ms < stats.min_time {
                        stats.min_time = result.time_ms;
                    }
                    if result.time_ms > stats.max_time {
                        stats.max_time = result.time_ms;
                    }
                } else {
                    stats.down += 1;
                }
                
                results.push(result);
            }
            Err(e) => {
                return Err(format!("Task error: {}", e));
            }
        }
    }

    // Calculate average time
    if stats.up > 0 {
        let total_time: u128 = results
            .iter()
            .filter(|r| r.success)
            .map(|r| r.time_ms)
            .sum();
        stats.avg_time = total_time / stats.up as u128;
    } else {
        stats.min_time = 0;
    }

    Ok(CheckResponse { results, stats })
}

/// Check a single URL
async fn check_single_url(
    client: reqwest::Client,
    url: String,
) -> Result<UrlResult, String> {
    let start = Instant::now();
    let resp = client.get(&url).send().await;
    let elapsed = start.elapsed().as_millis();

    match resp {
        Ok(r) => {
            let status_code = r.status().as_u16();
            let status = status_code.to_string();
            let reason = r.status().canonical_reason().unwrap_or("").to_string();
            let size_bytes = r.content_length().unwrap_or(0);
            let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
            let success = status_code >= 200 && status_code < 400;

            Ok(UrlResult {
                url,
                status,
                reason,
                time_ms: elapsed,
                size_bytes,
                timestamp,
                success,
            })
        }
        Err(e) => {
            let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
            Ok(UrlResult {
                url,
                status: "ERROR".to_string(),
                reason: format!("{}", e),
                time_ms: elapsed,
                size_bytes: 0,
                timestamp,
                success: false,
            })
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![check_urls])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

