use rand::rngs::ThreadRng;
use rand::Rng;
use rand_distr::Normal;
use reqwest;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "3d6196706fb24bf1b30ce1e8444ae0b1"; // Replace with your API key
    let symbol = "MSFT"; // Stock symbol, e.g., AAPL for Apple Inc.
    let interval = "1day";
    let range = "90days"; // For example, past 30 days

    let url = format!(
        "https://api.twelvedata.com/time_series?symbol={}&interval={}&range={}&apikey={}",
        symbol, interval, range, api_key
    );
    let response = reqwest::get(&url).await?.text().await?;
    let json: Value = serde_json::from_str(&response)?;
    
    // Extracting price data
    let price_data = json["values"].as_array().ok_or("Data format error")?;

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
