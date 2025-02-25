use std::{collections::HashMap, sync::Mutex};
use lazy_static::lazy_static;
use sqlite::State;
use tokio;
use axum::{
    http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, string};
// use phf::phf_map;

// static mut CURRENCIES: phf::Map<&'static str, &'static str> = phf_map!{
//     "United Arab Emirates Dirham" => "AED",
//     "Afghan Afghani" => "AFN",
//     "Albanian Lek" => "ALL",
//     "Armenian Dram" => "AMD",
//     "Dutch Guilders" => "ANG",
//     "Angolan Kwanza" => "AOA",
//     "Argentine Peso" => "ARS",
//     "Australian Dollar" => "AUD",
//     "Aruban Florin" => "AWG",
//     "Azerbaijani Manat" => "AZN",
//     "Bosnia-Herzegovina Convertible Mark" => "BAM",
//     "Barbadian Dollar" => "BBD",
//     "Bangladeshi Taka" => "BDT",
//     "Bulgarian Lev" => "BGN",
//     "Bahraini Dinar" => "BHD",
//     "Burundian Franc" => "BIF",
//     "Bermudian Dollar" => "BMD",
//     "Bruneian Dollar" => "BND",
//     "Bolivian Boliviano" => "BOB",
//     "Brazilian Real" => "BRL",
//     "Bahamian Dollar" => "BSD",
//     "Bhutanese Ngultrum" => "BTN",
//     "Botswanan Pula" => "BWP",
//     "Belizean Dollar" => "BZD",
//     "Canadian Dollar" => "CAD",
//     "Congolese Franc" => "CDF",
//     "Swiss Franc" => "CHF",
//     "Chilean Unit of Account UF" => "CLF",
//     "Chilean Peso" => "CLP",
//     "Chinese Yuan Offshore" => "CNH",
//     "Chinese Yuan" => "CNY",
//     "Colombian Peso" => "COP",
//     "Cuban Peso" => "CUP",
//     "Cape Verdean Escudo" => "CVE",
//     "Czech Republic Koruna" => "CZK",
//     "Djiboutian Franc" => "DJF",
//     "Danish Krone" => "DKK",
//     "Dominican Peso" => "DOP",
//     "Algerian Dinar" => "DZD",
//     "Egyptian Pound" => "EGP",
//     "Eritrean Nakfa" => "ERN",
//     "Ethiopian Birr" => "ETB",
//     "Euro" => "EUR",
//     "Fijian Dollar" => "FJD",
//     "Falkland Islands Pound" => "FKP",
//     "British Pound Sterling" => "GBP",
//     "Georgian Lari" => "GEL",
//     "Ghanaian Cedi" => "GHS",
//     "Gibraltar Pound" => "GIP",
//     "Gambian Dalasi" => "GMD",
//     "Guinean Franc" => "GNF",
//     "Guatemalan Quetzal" => "GTQ",
//     "Guyanaese Dollar" => "GYD",
//     "Hong Kong Dollar" => "HKD",
//     "Honduran Lempira" => "HNL",
//     "Croatian Kuna" => "HRK",
//     "Haitian Gourde" => "HTG",
//     "Hungarian Forint" => "HUF",
//     "Indonesian Rupiah" => "IDR",
//     "Israeli New Sheqel" => "ILS",
//     "Indian Rupee" => "INR",
//     "Iraqi Dinar" => "IQD",
//     "Iranian Rial" => "IRR",
//     "Icelandic Krona" => "ISK",
//     "Jamaican Dollar" => "JMD",
//     "Jordanian Dinar" => "JOD",
//     "Japanese Yen" => "JPY",
//     "Kenyan Shilling" => "KES",
//     "Kyrgystani Som" => "KGS",
//     "Cambodian Riel" => "KHR",
//     "Comorian Franc" => "KMF",
//     "North Korean Won" => "KPW",
//     "South Korean Won" => "KRW",
//     "Kuwaiti Dinar" => "KWD",
//     "Caymanian Dollar" => "KYD",
//     "Kazakhstani Tenge" => "KZT",
//     "Laotian Kip" => "LAK",
//     "Lebanese Pound" => "LBP",
//     "Sri Lankan Rupee" => "LKR",
//     "Liberian Dollar" => "LRD",
//     "Basotho Maloti" => "LSL",
//     "Libyan Dinar" => "LYD",
//     "Moroccan Dirham" => "MAD",
//     "Moldovan Leu" => "MDL",
//     "Malagasy Ariary" => "MGA",
//     "Macedonian Denar" => "MKD",
//     "Myanma Kyat" => "MMK",
//     "Mongolian Tugrik" => "MNT",
//     "Macanese Pataca" => "MOP",
//     "Mauritanian Ouguiya" => "MRU",
//     "Mauritian Rupee" => "MUR",
//     "Maldivian Rufiyaa" => "MVR",
//     "Malawian Kwacha" => "MWK",
//     "Mexican Peso" => "MXN",
//     "Malaysian Ringgit" => "MYR",
//     "Mozambican Metical" => "MZN",
//     "Namibian Dollar" => "NAD",
//     "Nigerian Naira" => "NGN",
//     "Norwegian Krone" => "NOK",
//     "Nepalese Rupee" => "NPR",
//     "New Zealand Dollar" => "NZD",
//     "Omani Rial" => "OMR",
//     "Panamanian Balboa" => "PAB",
//     "Peruvian Nuevo Sol" => "PEN",
//     "Papua New Guinean Kina" => "PGK",
//     "Philippine Peso" => "PHP",
//     "Pakistani Rupee" => "PKR",
//     "Polish Zloty" => "PLN",
//     "Paraguayan Guarani" => "PYG",
//     "Qatari Rial" => "QAR",
//     "Romanian Leu" => "RON",
//     "Serbian Dinar" => "RSD",
//     "Russian Ruble" => "RUB",
//     "Rwandan Franc" => "RWF",
//     "Saudi Arabian Riyal" => "SAR",
//     "Seychellois Rupee" => "SCR",
//     "Sudanese Pound" => "SDG",
//     "Swedish Krona" => "SEK",
//     "Singapore Dollar" => "SGD",
//     "Saint Helena Pound" => "SHP",
//     "Sierra Leonean Leone" => "SLL",
//     "Somali Shilling" => "SOS",
//     "Surinamese Dollar" => "SRD",
//     "Syrian Pound" => "SYP",
//     "Swazi Emalangeni" => "SZL",
//     "Thai Baht" => "THB",
//     "Tajikistani Somoni" => "TJS",
//     "Turkmenistani Manat" => "TMT",
//     "Tunisian Dinar" => "TND",
//     "Tongan Pa'anga" => "TOP",
//     "Turkish Lira" => "TRY",
//     "Trinidad and Tobago Dollar" => "TTD",
//     "Taiwan New Dollar" => "TWD",
//     "Tanzanian Shilling" => "TZS",
//     "Ukrainian Hryvnia" => "UAH",
//     "Ugandan Shilling" => "UGX",
//     "United States Dollar" => "USD",
//     "Uruguayan Peso" => "UYU",
//     "Uzbekistan Som" => "UZS",
//     "Vietnamese Dong" => "VND",
//     "Ni-Vanuatu Vatu" => "VUV",
//     "Samoan Tala" => "WST",
//     "CFA Franc BEAC" => "XAF",
//     "East Caribbean Dollar" => "XCD",
//     "Special Drawing Rights" => "XDR",
//     "CFA Franc BCEAO" => "XOF",
//     "CFP Franc" => "XPF",
//     "Yemeni Rial" => "YER",
//     "South African Rand" => "ZAR",
//     "Zambian Kwacha" => "ZMW",
//     };
    

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


async fn currency_list_route() -> impl IntoResponse {


    
    (StatusCode::OK, Json(get_currencies()))
}


async fn convert_route(Json(payload): Json<ConversionRequest>) -> impl IntoResponse {

    let currency_from_iso = &payload.currency_from;
    let currency_to_iso = &payload.currency_to;

    (StatusCode::OK, Json(get_conversion(currency_from_iso, currency_to_iso)))
}


fn get_currencies() -> Vec<Currency>{

    let conn = CONNECTION.lock().unwrap();

    let query2 = "SELECT * FROM currencies;";

    let mut statement = conn.prepare(query2).unwrap();
    
    let mut currencies: Vec<Currency> = Vec::new();

    while let Ok(State::Row) = statement.next() {
        let iso = statement.read::<String, _>("iso").unwrap();
        let name = statement.read::<String, _>("name").unwrap();
        let value = statement.read::<f64, _>("value").unwrap();

        let currency = Currency {
            iso,
            name,
            value,
        };
        currencies.push(currency);
    };

    currencies
}

fn get_conversion(currency_from_iso:&str, currency_to_iso:&str) -> HashMap<String, Currency>{

    println!("Getting conversions");

    let conn = CONNECTION.lock().unwrap();

    let query = "
    SELECT * FROM currencies WHERE iso=:cfi OR iso=:cti
    ORDER BY 
        CASE
            WHEN iso = :cfi THEN 1
            WHEN iso = :cti THEN 2
            ELSE 3
        END;
    ";


    let mut statement = conn.prepare(query).unwrap();
    statement.bind((":cfi", currency_from_iso)).unwrap();
    statement.bind((":cti", currency_to_iso)).unwrap();

    if currency_from_iso == currency_to_iso {

        let mut currencies: HashMap<String, Currency> = HashMap::new();

        if let Ok(State::Row) = statement.next() {
            let iso = statement.read::<String, _>("iso").unwrap();
            let name = statement.read::<String, _>("name").unwrap();
            let value = statement.read::<f64, _>("value").unwrap();
    
            let currency = Currency {
                iso,
                name,
                value,
            };
    
    
            currencies.insert("currency_from".to_owned(), currency.clone());
            currencies.insert("currency_to".to_owned(), currency.clone());
        }
        return currencies;
    }

    
    let mut currencies: HashMap<String, Currency> = HashMap::new();


    if let Ok(State::Row) = statement.next() {
        let iso = statement.read::<String, _>("iso").unwrap();
        let name = statement.read::<String, _>("name").unwrap();
        let value = statement.read::<f64, _>("value").unwrap();

        let currency = Currency {
            iso,
            name,
            value,
        };


        currencies.insert("currency_from".to_owned(), currency);
    }

    if let Ok(State::Row) = statement.next() {
        let iso = statement.read::<String, _>("iso").unwrap();
        let name = statement.read::<String, _>("name").unwrap();
        let value = statement.read::<f64, _>("value").unwrap();

        let currency = Currency {
            iso,
            name,
            value,
        };

        currencies.insert("currency_to".to_owned(), currency);
    }
    

    currencies
}

fn create_database() -> () {

    let conn = CONNECTION.lock().unwrap();

    let query = "
        CREATE TABLE IF NOT EXISTS currencies (iso CHAR(3), name TINYTEXT, value FLOAT);
        INSERT INTO currencies VALUES ('USD', 'United States Dollar', 1);
        INSERT INTO currencies VALUES ('HUF', 'Hungarian Forint', 0.00261022);
        INSERT INTO currencies VALUES ('SEK', 'Swedish Krona', 0.0938402); 
    ";
    
    let result = conn.execute(query);

    match result {
        Err(err) => println!("An error ): {}", err.to_string()),
        Ok(_) => println!("No error (:"),
    }

}


pub async fn server_start() {

    create_database();
    get_currencies();

    let app:Router<()> = Router::new()
    .route("/currencies", get(currency_list_route))
    .route("/convert", post(convert_route));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
