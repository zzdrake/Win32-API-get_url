use reqwest;
use serde_json::Value;
use std::io;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;
use tokio;

async fn check_debug_port(port: u16) -> bool {
    match reqwest::get(&format!("http://localhost:{}/json/version", port)).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn launch_browser_debug_mode(browser_path: &str, debug_port: u16) -> io::Result<Child> {
    let args = vec![
        format!("--remote-debugging-port={}", debug_port),
        "--user-data-dir=./chrome-debug-profile".to_string(),
    ];

    Command::new(browser_path).args(&args).spawn()
}

async fn get_browser_tabs(debug_port: u16) -> Result<Vec<Value>, reqwest::Error> {
    let response = reqwest::get(&format!("http://localhost:{}/json", debug_port)).await?;
    let tabs: Vec<Value> = response.json().await?;
    Ok(tabs)
}

async fn monitor_urls(debug_port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_tabs: Vec<String> = Vec::new();

    println!("Start listening URL change...");
    println!("Press Ctrl+C Exit...");

    loop {
        match get_browser_tabs(debug_port).await {
            Ok(tabs) => {
                let mut current_urls: Vec<String> = Vec::new();
                let mut has_changes = false;

                for tab in tabs.iter() {
                    if let Some(url) = tab["url"].as_str() {
                        current_urls.push(url.to_string());

                        if !last_tabs.contains(&url.to_string()) {
                            println!("New URL: {}", url);
                            has_changes = true;
                        }
                    }
                }

                for old_url in last_tabs.iter() {
                    if !current_urls.contains(old_url) {
                        println!("Closed URL: {}", old_url);
                        has_changes = true;
                    }
                }

                if has_changes {
                    println!("\nCurrent All URLs:");
                    for (i, url) in current_urls.iter().enumerate() {
                        println!("New label {}: {}", i + 1, url);
                    }
                    println!();
                }

                last_tabs = current_urls;
            }
            Err(e) => println!("Failed to fetch URL: {}", e),
        }

        thread::sleep(Duration::from_secs(1));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Tool of get URL of browser");
    println!("Supported browser: Chrome, Edge");

    let debug_port = 9222;

    if !check_debug_port(debug_port).await {
        println!("Conn't find debug mode of browser, Starting...");

        let chrome_paths = [
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        ];

        let mut launched = false;
        for path in chrome_paths.iter() {
            if std::path::Path::new(path).exists() {
                match launch_browser_debug_mode(path, debug_port) {
                    Ok(_) => {
                        println!("Already start Chrome debug mod");
                        launched = true;
                        break;
                    }
                    Err(e) => println!("Filed to open Chrome: {}", e),
                }
            }
        }

        if !launched {
            let edge_paths = [
                r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
                r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
            ];

            for path in edge_paths.iter() {
                if std::path::Path::new(path).exists() {
                    match launch_browser_debug_mode(path, debug_port) {
                        Ok(_) => {
                            println!("Already start debug mod of Edge");
                            launched = true;
                            break;
                        }
                        Err(e) => println!("Failed to open Edge: {}", e),
                    }
                }
            }
        }

        if !launched {
            println!(
                "Unable to start the browser, please manually start the browser in debug mode"
            );
            println!(
                "Startup Command: chrome.exe --remote-debugging-port=9222 --user-data-dir=./chrome-debug-profile"
            );
            return Ok(());
        }

        println!("Wait for the browser to start...");
        thread::sleep(Duration::from_secs(3));
    } else {
        println!("Detected that the browser is running in debug mode");
    }

    println!("\nFetching browser tabURL...");
    match get_browser_tabs(debug_port).await {
        Ok(tabs) => {
            if tabs.is_empty() {
                println!("No open tabs found");
            } else {
                for (i, tab) in tabs.iter().enumerate() {
                    if let Some(url) = tab["url"].as_str() {
                        println!("Tabs {}: {}", i + 1, url);
                    }
                }
            }
        }
        Err(e) => println!("Failed to get the tab page: {}", e),
    }

    monitor_urls(debug_port).await?;

    Ok(())
}
