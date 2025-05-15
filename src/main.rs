use reqwest::Client;
use serde::Deserialize;
use std::env;
use anyhow::Result;
use tokio;
use std::io::{self, Write};
use chrono::{DateTime, Local, Utc};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct QuoteResponse {
    c: Option<f64>, // current price
    d: Option<f64>, // change
    dp: Option<f64>, // percent change
    h: Option<f64>, // high price of the day
    l: Option<f64>, // low price of the day
    o: Option<f64>, // open price of the day
    pc: Option<f64>, // previous close price
    t: Option<u64>,  // timestamp (Unix time)
}

#[tokio::main]
async fn main() -> Result<()> {
    print!("Enter stock ticker: ");
    io::stdout().flush()?;
    let mut ticker = String::new();
    io::stdin().read_line(&mut ticker)?;
    let ticker = ticker.trim().to_uppercase();

    let api_key = env::var("FINNHUB_API_KEY").unwrap_or("d0j572hr01ql09hq965gd0j572hr01ql09hq9660".into());
    let url = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", ticker, api_key);

    let client = Client::new();
    let resp = client.get(&url).send().await?;
    let text = resp.text().await?;

    let data: QuoteResponse = serde_json::from_str(&text)?;

    println!("\nTicker: {}", ticker);
    println!("Current Price: ${:.2}", data.c.unwrap_or(-1.0));
    println!("Change: ${:.2} ({:.2}%)", data.d.unwrap_or(0.0), data.dp.unwrap_or(0.0));
    println!("High: ${:.2} | Low: ${:.2}", data.h.unwrap_or(0.0), data.l.unwrap_or(0.0));

    if let Some(ts) = data.t {
        if let Some(utc) = chrono::NaiveDateTime::from_timestamp_opt(ts as i64, 0) {
            let datetime: DateTime<Utc> = DateTime::<Utc>::from_utc(utc, Utc);
            let local_time = datetime.with_timezone(&Local);
            println!("Market data as of: {}", local_time.format("%B %d, %Y at %I:%M %p"));
        }
    }

    let adjusted_time = chrono::Local::now() - chrono::Duration::hours(1);
    println!("Request made at: {}", adjusted_time.format("%B %d, %Y at %I:%M %p"));



    Ok(())
}