use tauri_plugin_http::reqwest;
use serde_json::Value;
use dotenv::dotenv;

async fn get_auth() -> String {
   dotenv().ok();
   let fastforex_api_token = std::env::var("API_KEY").expect("No API key set");
   return fastforex_api_token;
}


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

#[tauri::command]
async fn get_exchange_rate(currency_from: &str, currency_to: &str) -> Result<String, String> {

    let client = reqwest::Client::new();

    let auth: String = get_auth().await;

    let response = client.get(format!("https://api.fastforex.io/convert?from={}&to={}&amount=1&precision=2&api_key={}", currency_from, currency_to, auth))
        .header("Content-type", "application/x-www-form-urlencoded")
        .header("Authorization", "Basic ".to_owned() + &auth)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let body: Value = response.json().await.map_err(|e| e.to_string())?;

    Ok(body.to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_exchange_rate, get_currencies])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
