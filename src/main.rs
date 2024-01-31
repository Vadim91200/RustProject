use rand::rngs::ThreadRng;
use rand::Rng;
use rand_distr::Normal;
use reqwest;
use std::error::Error;
use serde_json::Value;

use rand::distributions::{Distribution, Uniform};
use std::vec::Vec;
use std::io;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    //let symbol = "AAPL"; // Stock symbol, e.g., AAPL for Apple Inc.
    println!("Enter the stock symbol/ticker:");
    let mut symbol = String::new();
    io::stdin().read_line(&mut symbol)?;
    // Trim any leading or trailing whitespace
    symbol = symbol.trim().to_string();

    let interval = "1day";

    //let range = "30days"; // For example, past 30 days
    println!("Choose the time duration:");
    println!("1. 30 days");
    println!("2. 90 days");
    println!("3. 365 days");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    // Trim any leading or trailing whitespace
    choice = choice.trim().to_string();
    // Validate time duration choice
    let range = match choice.as_str() {
        "1" => "30days",
        "2" => "90days",
        "3" => "365days",
        _ => {
            eprintln!("Invalid choice. Please enter 1, 2, or 3.");
            return Ok(());
        }
    };

    let url = format!(
        "https://api.twelvedata.com/time_series?symbol={}&interval={}&range={}&apikey={}",
        symbol, interval, range, api_key
    );
    
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                let response_text = response.text().await?;
                let json: serde_json::Value = serde_json::from_str(&response_text)?;
    
                // Check if the JSON response indicates a symbol not found error
                if let Some(code) = json.get("code") {
                    if let Some(code_value) = code.as_u64() {
                        if code_value == 404 {
                            // Symbol not found
                            if let Some(message) = json.get("message") {
                                if let Some(message_str) = message.as_str() {
                                    eprintln!("{}", message_str);
                                } else {
                                    eprintln!("Symbol not found, but couldn't parse the error message");
                                }
                            } else {
                                eprintln!("Symbol not found, but the error response doesn't contain a message");
                            }
    
                            // Handle the case when the symbol is not found
                        } else {
                            // Process the JSON data using a separate function
                            process_json_data(&json)?;
                        }
                    }
                } else {
                    // Process the JSON data using a separate function
                    process_json_data(&json)?;
                }
            } else {
                eprintln!("Request failed with status code: {}", response.status());
                // Handle other non-success status codes if needed
            }
        }
        Err(err) => {
            eprintln!("Error making the request: {}", err);
        }
    }

    Ok(())
}


// Function to process JSON data
fn process_json_data(json: &Value) -> Result<(), Box<dyn Error>> {
    let mut dates = Vec::new();
    let mut daily_prices = Vec::new();

    if let Some(data) = json["values"].as_array() {
        for entry in data {
            if let (Some(date), Some(open)) = (entry["datetime"].as_str(), entry["close"].as_str()) {
                dates.push(date);
                daily_prices.push(open.parse::<f32>()?);
            }
        }
    }

    let mut daily_returns = Vec::new();
    for window in daily_prices.windows(2) {
        let log_return = (window[0] / window[1]).ln();
        daily_returns.push(log_return);
    }

    let mean_log_return = daily_returns.iter().sum::<f32>() / daily_returns.len() as f32;
    let variance: f32 = daily_returns.iter().map(|&r| (r - mean_log_return).powi(2)).sum::<f32>() / (daily_returns.len() - 1) as f32;
    let daily_volatility = variance.sqrt();

    let days = 30;
    let simulations = 1000;
    let mut rng = rand::thread_rng();

    let mut predictions = Vec::new();
    for _ in 0..simulations {
        let predicted_price = simulate_price(&mut rng, daily_prices[0], daily_volatility, days);
        predictions.push(predicted_price);
    }

    let average_price = predictions.iter().sum::<f32>() / simulations as f32;
    println!("Prix moyen prédit après {} jours : {}", days, average_price);
    Ok(())
}

fn simulate_price(rng: &mut ThreadRng, start_price: f32, volatility: f32, days: usize) -> f32 {
    let mut price = start_price;
    let drift = 0.0;

    for _ in 0..days {
        let change_pct = drift + volatility * rng.sample::<f32, _>(Normal::new(0.0, 1.0).unwrap());
        price *= 1.0 + change_pct;
    }

    price
}
