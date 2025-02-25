use tauri_plugin_http::reqwest;
use serde_json::Value;
use dotenv::dotenv;
use tokio;
pub mod server;

// CLIENT //

// API Auth token
async fn get_auth() -> String {
   dotenv().ok().expect("[ERROR] .env not found");
   std::env::var("API_KEY").ok().expect("[ERROR] API_KEY environment variable not found")
}

#[tauri::command]
async fn test_http() -> Result<String, String> {
    let client = reqwest::Client::new();

    let response = client.get("http://127.0.0.1:8080/currencies")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body: String = response.text().await.expect("");

    Ok(body.to_string())
}

// Get a list of currencies supported by the API
#[tauri::command]
async fn get_currencies() -> Result<String, String> {

    let client = reqwest::Client::new();

    let auth: String = get_auth().await;

    let response = client.get(format!("https://api.fastforex.io/currencies?api_key={}", &auth))
        .header("Content-type", "application/x-www-form-urlencoded")
        .header("Authorization", "Basic ".to_owned() + &auth)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body: Value = response.json().await.map_err(|e| e.to_string())?;

    Ok(body.to_string())
}


// Get the exchange rate of two currencies
#[tauri::command]
async fn get_exchange_rate(currency_from: &str, currency_to: &str) -> Result<String, String> {

    let client = reqwest::Client::new();

    let auth: String = get_auth().await;

    let response = client.get(format!("https://api.fastforex.io/convert?from={}&to={}&amount=1&precision=5&api_key={}", currency_from, currency_to, auth))
        .header("Content-type", "application/x-www-form-urlencoded")
        .header("Authorization", "Basic ".to_owned() + &auth)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body: Value = response.json().await.map_err(|e| e.to_string())?;

    Ok(body.to_string())
}

// Entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {


    tokio::spawn(server::server_start());

    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_exchange_rate, get_currencies, test_http])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
