use std::collections::HashMap;

use lazy_static::lazy_static;
use tauri_plugin_http::reqwest;
use serde_json::Value;
use tokio;
pub mod server;

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}


// Get a list of currencies supported by the API
#[tauri::command]
async fn get_currencies() -> Result<String, String> {

    let response: reqwest::Response = CLIENT.get("http://127.0.0.1:8080/currencies")
        .send()
        .await
        .map_err(|e| format!("[ERROR] {}", e.to_string()))?;

    let body: Value = response.json().await.map_err(|op| format!("[ERROR] {}", op.to_string()))?;

    Ok(body.to_string())
}


// Get the exchange rate of two currencies
#[tauri::command]
async fn get_exchange_rate(currency_from_name: &str, currency_to_name: &str) -> Result<String, String> {

    let mut request: HashMap<&str, &str> = HashMap::new();
    request.insert("currency_from", currency_from_name);
    request.insert("currency_to", currency_to_name);

    let response: reqwest::Response = CLIENT.post("http://127.0.0.1:8080/convert")
        .json(&request)
        .header("Content-type", "application/json")
        .send()
        .await
        .map_err(|e| format!("[ERROR] {}", e.to_string()))?;

    let body: Value = response.json().await.map_err(|op| format!("[ERROR] {}", op.to_string()))?;

    // println!("True val {}", response.text().await.map_err(|op| op.to_string())?);

    Ok(body.to_string())
    // Ok("Yo".to_string())
}

// Entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {

    tokio::spawn(server::server_start());

    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_exchange_rate, get_currencies])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
