use reqwest;
use serde_json::Value;
use std::error::Error;
extern crate rand; // Utiliser la bibliothèque rand pour la génération aléatoire

use rand::distributions::{Distribution, Uniform};
use std::vec::Vec;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "3d6196706fb24bf1b30ce1e8444ae0b1"; // Replace with your API key
    let symbol = "AAPL"; // Stock symbol, e.g., AAPL for Apple Inc.
    let interval = "1day";
    let range = "30days"; // For example, past 30 days

    let url = format!(
        "https://api.twelvedata.com/time_series?symbol={}&interval={}&range={}&apikey={}",
        symbol, interval, range, api_key
    );
    let response = reqwest::get(&url).await?.text().await?;
    let json: Value = serde_json::from_str(&response)?;

    let mut dates = Vec::new();
    let mut daily_prices = Vec::new();

    if let Some(data) = json["values"].as_array() {
        for entry in data {
            if let (Some(date), Some(open)) = (entry["datetime"].as_str(), entry["open"].as_str()) {
                dates.push(date);
                daily_prices.push(open.parse::<f32>()?);
            }
        }
    }
    Ok(())
}
