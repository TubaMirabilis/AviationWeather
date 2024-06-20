use reqwest::Client;
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide an ICAO code.");
        process::exit(1);
    }

    let icao = args[1].trim().to_uppercase();
    if icao.is_empty() {
        eprintln!("Invalid ICAO code.");
        process::exit(1);
    }

    let url = format!("https://aviationweather.gov/api/data/metar?ids={}&format=raw&taf=false", icao);
    let client = Client::new();

    match client.get(&url).send().await {
        Ok(response) => {
            match response.text().await {
                Ok(content) => {
                    println!("{}", content);
                }
                Err(_) => {
                    eprintln!("Failed to read the response content.");
                    process::exit(1);
                }
            }
        }
        Err(_) => {
            eprintln!("Failed to make the request.");
            process::exit(1);
        }
    }
}
