use rand::rngs::ThreadRng;
use rand::Rng;
use rand_distr::Normal;
use reqwest;
use serde_json::Value;

use rand::distributions::{Distribution, Uniform};
use std::vec::Vec;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let api_key = "3d6196706fb24bf1b30ce1e8444ae0b1"; // Replace with your API key


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

    let mut prices = Vec::new();
    for entry in price_data {
        let close_price = entry["close"].as_str().ok_or("Price format error")?.parse::<f64>()?;
        prices.push(close_price);
    }

    let mut daily_returns = Vec::new();
    for window in prices.windows(2) {
        let log_return = (window[0] / window[1]).ln();
        daily_returns.push(log_return);
    }

    let mean_log_return = daily_returns.iter().sum::<f64>() / daily_returns.len() as f64;
    let variance: f64 = daily_returns.iter().map(|&r| (r - mean_log_return).powi(2)).sum::<f64>() / (daily_returns.len() - 1) as f64;
    let daily_volatility = variance.sqrt();

    let days = 30;
    let simulations = 1000;
    let mut rng = rand::thread_rng();

    let mut predictions = Vec::new();
    for _ in 0..simulations {
        let predicted_price = simulate_price(&mut rng, prices[0], daily_volatility, days);
        predictions.push(predicted_price);
    }

    let average_price = predictions.iter().sum::<f64>() / simulations as f64;
    println!("Prix moyen prédit après {} jours : {}", days, average_price);
    Ok(())
}

fn simulate_price(rng: &mut ThreadRng, start_price: f64, volatility: f64, days: usize) -> f64 {
    let mut price = start_price;
    let drift = 0.0;

    for _ in 0..days {
        let change_pct = drift + volatility * rng.sample::<f64, _>(Normal::new(0.0, 1.0).unwrap());
        price *= 1.0 + change_pct;
    }

    price
}
