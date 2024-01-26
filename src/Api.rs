
use reqwest;
use serde_json::Value;
use std::error::Error;
extern crate rand; // Utiliser la bibliothèque rand pour la génération aléatoire

use rand::distributions::{Distribution, Uniform};
use std::vec::Vec;
use std::io;


#[tokio::Result]
fn GetResult(Model: &state) -> Result<(), Box<dyn Error>>
{
    let api_key = "3d6196706fb24bf1b30ce1e8444ae0b1"; // Replace with your API key
    let mut symbol = state.duration.clone();
    let interval = "1day";
    let choice = state.duration.clone();

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
                            process_json_data(&json, state)?;
                        }
                    }
                } else {
                    // Process the JSON data using a separate function
                    process_json_data(&json, state)?;
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
fn process_json_data(json: &Value, Model: &state) -> Result<(), Box<dyn Error>> {
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

    // Calculate daily returns
    let mut returns = Vec::new();
    for i in 0..daily_prices.len() - 1 {
        let daily_return = (daily_prices[i + 1] - daily_prices[i]) / daily_prices[i];
        returns.push(daily_return);
    }

    let num_simulations = 1000;
    let days_to_simulate = 90;
    let mut simulations: Vec<Vec<f64>> = Vec::with_capacity(num_simulations);

    let mut rng = rand::thread_rng();
    let between = Uniform::from(0..returns.len());

    // Perform Monte Carlo simulations
    for _ in 0..num_simulations {
        let mut simulated_prices = Vec::with_capacity(days_to_simulate);
        let mut last_price = *daily_prices.last().unwrap();

        for _ in 0..days_to_simulate {
            let random_index = between.sample(&mut rng);
            let random_return = returns[random_index];
            last_price *= 1.0 + random_return;
            simulated_prices.push(last_price.into());
        }

        simulations.push(simulated_prices);
    }

    // Gather final prices of each simulation
    let mut final_prices = Vec::new();
    for simulation in &simulations {
        if let Some(&last_price) = simulation.last() {
            final_prices.push(last_price);
        }
    }

    // Calculate the average of final prices
    if !final_prices.is_empty() {
        let sum: f64 = final_prices.iter().sum();
        let mean = sum / final_prices.len() as f64;

        println!("Moyenne des prix finaux sur toutes les simulations: {}", mean);
        state.set(Model {
            input_text: state.input_text.clone(),
            duration: state.duration.clone(),
            result_text: mean,
        });

    } else {
        println!("Aucune donnée disponible pour calculer la moyenne.");
    }

    Ok(())
}
