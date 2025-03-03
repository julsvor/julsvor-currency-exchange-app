use std::{collections::HashMap, sync::Mutex};
use lazy_static::lazy_static;
use serde_json::json;
use sqlite::State;
use tokio;
use axum::{
    http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};


// Initialize static database connection with thread safety
lazy_static! {
    static ref CONNECTION: Mutex<sqlite::Connection> = Mutex::new(sqlite::open(":memory:").unwrap());
}


#[derive(Serialize, Deserialize, Clone)]
struct Currency {
    iso: String,
    name: String,
    value: f64,
}

#[derive(Deserialize)]
struct ConversionRequest {
    currency_from:String,
    currency_to:String,
}


// Respond to /currencies with a list of available currencies
async fn currency_list_route() -> impl IntoResponse {

    let result = get_currencies();

    let req_response: serde_json::Value;
    
    match result {
        Ok(res) => {
            req_response = json!({"code": "200","payload": res});
            (StatusCode::OK, Json(req_response))
        }

        Err(e) => {
            req_response = json!({"code":"500", "error": e.to_string()});
            (StatusCode::INTERNAL_SERVER_ERROR, Json(req_response))
        }
    }
}


// Respond to /convert with the two given currencies value, name and iso
async fn convert_route(Json(payload): Json<ConversionRequest>) -> impl IntoResponse {

    let result = get_conversion(&payload.currency_to, &payload.currency_from);

    let req_response: serde_json::Value;

    match result {
        Ok(res) => {
            req_response = json!({"code":"200", "payload":res});
            (StatusCode::OK, Json(req_response))
        }

        Err(e) => {
            req_response = json!({"code":"500", "error":e.to_string()});
            (StatusCode::INTERNAL_SERVER_ERROR, Json(req_response))
        }
    }
}


// Get the list of currencies from in-memory sqlite database
fn get_currencies() -> Result<Vec<Currency>, String> {

    let conn = CONNECTION.lock().map_err(|err| format!("Failed to lock connection: {}", err))?;

    let query = "SELECT * FROM currencies;";

    let mut statement = conn.prepare(query).map_err(|err| format!("SQL prepare failed: {}", err))?;

    let mut currencies: Vec<Currency> = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let iso = statement.read::<String, _>("iso").map_err(|err| format!("Failed to read String 'iso': {}", err))?;
        let name = statement.read::<String, _>("name").map_err(|err| format!("Failed to read String 'name': {}", err))?;
        let value = statement.read::<f64, _>("value").map_err(|err| format!("Failed to read f64'value': {}", err))?;

        let currency = Currency {iso, name, value};
        currencies.push(currency);
    };

    Ok(currencies)
}


// Get the two given currencies
fn get_conversion(currency_from_iso:&str, currency_to_iso:&str) -> Result<HashMap<String, Currency>, String>{

    let conn = CONNECTION.lock().map_err(|err| format!("Failed to lock connection: {}", err))?;

    let query = "
    SELECT * FROM currencies WHERE iso=:cfi
    UNION ALL
    SELECT * FROM currencies WHERE iso=:cti;
    ";

    let mut statement = conn.prepare(query).map_err(|err| format!("SQL prepare failed: {}", err))?;
    statement.bind((":cfi", currency_from_iso)).map_err(|err| format!("Failed to bind :cfi : {}", err))?;
    statement.bind((":cti", currency_to_iso)).map_err(|err| format!("Failed to bind :cti : {}", err))?;
    
    let mut currencies: HashMap<String, Currency> = HashMap::new();

    for order in ["currency_to", "currency_from"] {
        if let Ok(State::Row) = statement.next() {
        let iso = statement.read::<String, _>("iso").map_err(|err| format!("Failed to read String 'iso': {}", err))?;
        let name = statement.read::<String, _>("name").map_err(|err| format!("Failed to read String 'name': {}", err))?;
        let value = statement.read::<f64, _>("value").map_err(|err| format!("Failed to read f64'value': {}", err))?;
    
            let currency = Currency {iso, name, value};
    
            currencies.insert(order.to_owned(), currency);
        }

    }

    Ok(currencies)
}


// Create the in-memory sqlite database
fn create_database() -> () {

    let conn = CONNECTION.lock().unwrap();

    let query = "
        CREATE TABLE IF NOT EXISTS currencies (iso CHAR(3), name TINYTEXT, value FLOAT);

        -- Rates during 2025-03-03
        INSERT INTO currencies VALUES ('USD', 'United States Dollar', 1);
        INSERT INTO currencies VALUES ('UAH', 'Ukrainian Hryvnia', 0.024048094 );
        INSERT INTO currencies VALUES ('TRY', 'Turkish Lira', 0.027448936);
        INSERT INTO currencies VALUES ('BYN', 'Belarussian Ruble', 0.3058093 );
        INSERT INTO currencies VALUES ('BGN', 'Bulgarian Lev', 0.53629232 );
        INSERT INTO currencies VALUES ('CHF', 'Swiss Franc', 1.11532 ); 
        INSERT INTO currencies VALUES ('HUF', 'Hungarian Forint', 0.00261022);
        INSERT INTO currencies VALUES ('SEK', 'Swedish Krona', 0.0938402); 
        INSERT INTO currencies VALUES ('EUR', 'Euro', 1.0491207); 
        INSERT INTO currencies VALUES ('RUB', 'Russian Ruble', 0.011150311); 
        INSERT INTO currencies VALUES ('NOK', 'Norwegian Krone', 0.089388265);
        INSERT INTO currencies VALUES ('GBP', 'British Pound', 1.271049); 
        INSERT INTO currencies VALUES ('DKK', 'Danish Krone', 0.14064366); 
        INSERT INTO currencies VALUES ('CZK', 'Czech Koruna', 0.041923727);
        INSERT INTO currencies VALUES ('MKD', 'Macedonian Denar', 0.017005435);
        INSERT INTO currencies VALUES ('PLN', 'Polish Zloty', 0.25220751); 
        INSERT INTO currencies VALUES ('RON', 'Romanian Leu', 0.21079086); 
        INSERT INTO currencies VALUES ('RSD', 'Serbian Dinar', 0.0089556509); 
    ";
    
    let _result = conn.execute(query);

}


pub async fn server_start() {

    create_database();

    let app:Router<()> = Router::new()
    .route("/currencies", get(currency_list_route))
    .route("/convert", post(convert_route));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
