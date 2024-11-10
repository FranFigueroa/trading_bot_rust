use reqwest::Client;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
struct ticker {
    price: String,
}


async fn get_price(client: &Client, symbol: &str) -> Result<String, reqwset::Error>{
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let response = client.get(&url).send().await?;    
    let ticker: Ticker = response.json().await?;
    Ok(ticker.price) 
}

async fn trade_decision(current_price: f64) {
    let buy_threshold = 30000.0;
    let sell_threshold = 35000.0;

    if current_price < buy_threshold {
        println!("Comprar BTC a ${}", current_price);
        // Aquí podrías llamar a una función para realizar una orden de compra
    } else if current_price > sell_threshold {
        println!("Vender BTC a ${}", current_price);
        // Aquí podrías llamar a una función para realizar una orden de venta
    } else {
        println!("Esperando. Precio actual: ${}", current_price);
    }
}



use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not found in .env");
    let api_secret = env::var("API_SECRET").expect("API_SECRET not found in .env");

    let client = Client::new();
    let symbol = "BTCUSDT";

    loop {
        match get_price(&client, symbol).await {
            Ok(price) => {
                let price_f64: f64 = price.parse().expect("Error al convertir el precio a número");
                println!("Precio actual de BTC/USDT: ${}", price_f64);
                trade_decision(price_f64).await;
            }
            Err(e) => eprintln!("Error al obtener el precio: {}", e),
        }
        sleep(Duration::from_secs(60)).await; /
    }
}
