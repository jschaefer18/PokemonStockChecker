// Import required crates and modules for HTTP routing, JSON handling, async runtime, and time formatting.
use axum::{extract::Query, http::StatusCode, response::Json, routing::get, Router};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};
use chrono::{Local, TimeZone, Utc};
use anyhow::Result;
use tokio;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer};

// Define query string structure expected from frontend (e.g., ?ticker=AAPL)
#[derive(Deserialize)]
struct QueryParams {
    ticker: String,
}

// Structure to match Finnhub API JSON response
#[derive(Deserialize, Debug, Serialize)]
#[allow(dead_code)]
struct QuoteResponse {
    c: Option<f64>,  // current price
    d: Option<f64>,  // dollar change
    dp: Option<f64>, // percent change
    h: Option<f64>,  // high
    l: Option<f64>,  // low
    o: Option<f64>,  // open
    pc: Option<f64>, // previous close
    t: Option<u64>,  // timestamp
}

// Entry point of the async app. Sets up router and starts the Axum server.
#[tokio::main]
async fn main() -> Result<()> {
    // Define a route and enable permissive CORS so frontend can make requests.
    let app = Router::new()
        .route("/quote", get(get_quote))
        .layer(CorsLayer::permissive());

    // Start server at localhost:3000
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 Server running at http://127.0.0.1:3000/quote?ticker=AAPL");

    // Serve the app with Axum
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

// Handler for GET /quote?ticker=XYZ — fetches and returns stock data
async fn get_quote(Query(params): Query<QueryParams>) -> Result<Json<HashMap<&'static str, String>>, (StatusCode, String)> {
    // Clean and uppercase the ticker
    let ticker = params.ticker.trim().to_uppercase();

    // Use API key from env or fallback key
    let api_key = env::var("FINNHUB_API_KEY").unwrap_or("d0j572hr01ql09hq965gd0j572hr01ql09hq9660".into());
    let url = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", ticker, api_key);

    // Send GET request to Finnhub API
    let client = Client::new();
    let resp = client.get(&url).send().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let text = resp.text().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Parse JSON response into QuoteResponse struct
    let data: QuoteResponse = serde_json::from_str(&text).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // Build a response map to send back to the frontend
    let mut result = HashMap::new();
    result.insert("ticker", ticker);
    result.insert("current_price", format!("{:.2}", data.c.unwrap_or(-1.0)));
    result.insert("change", format!("{:.2}", data.d.unwrap_or(0.0)));
    result.insert("percent_change", format!("{:.2}", data.dp.unwrap_or(0.0)));
    result.insert("high", format!("{:.2}", data.h.unwrap_or(0.0)));
    result.insert("low", format!("{:.2}", data.l.unwrap_or(0.0)));

    // Convert timestamp to readable local time
    if let Some(ts) = data.t {
        if let chrono::LocalResult::Single(utc) = Utc.timestamp_opt(ts as i64, 0) {
            let local_time = utc.with_timezone(&Local);
            result.insert("market_time", local_time.format("%B %d, %Y at %I:%M %p").to_string());
        }
    }

    // Add local request time
    let adjusted_time = Local::now();
    result.insert("request_time", adjusted_time.format("%B %d, %Y at %I:%M %p").to_string());

    // Return the map as JSON
    Ok(Json(result))
}
