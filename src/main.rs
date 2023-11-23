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
    // Calculer les rendements quotidiens
    let mut returns = Vec::new();
    for i in 0..daily_prices.len() - 1 {
         let daily_return = (daily_prices[i + 1] - daily_prices[i]) / daily_prices[i];
         returns.push(daily_return);
     }
     let num_simulations = 1000; // Nombre de simulations
     let days_to_simulate = 90; // Nombre de jours à simuler (3 mois)
     let mut simulations: Vec<Vec<f64>> = Vec::with_capacity(num_simulations);
 
     let mut rng = rand::thread_rng();
     let between = Uniform::from(0..returns.len());
 
     // Effectuer des simulations de Monte Carlo
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
    // Rassembler les prix finaux de chaque simulation
    let mut final_prices = Vec::new();
    for simulation in &simulations {
        if let Some(&last_price) = simulation.last() {
            final_prices.push(last_price);
        }
    }

    // Calculer la moyenne des prix finaux
    if !final_prices.is_empty() {
        let sum: f64 = final_prices.iter().sum();
        let mean = sum / final_prices.len() as f64;

        println!("Moyenne des prix finaux sur toutes les simulations: {}", mean);
    } else {
        println!("Aucune donnée disponible pour calculer la moyenne.");
    }
    Ok(())
}
