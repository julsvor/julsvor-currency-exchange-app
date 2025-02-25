// use tokio;
// use axum::{
//     http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
// };
// use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, string};
// use phf::phf_map;
// // SERVER //

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
    
    
    
    
//     async fn root() -> &'static str {
//         "Hello, world"
//     }
    
//     async fn currency_list_route() -> impl IntoResponse {
        
    
    
//         (StatusCode::OK, Json(CURRENCIES))
//     }
    
//     async fn convert_route() -> impl IntoResponse {
    
//         let conversion = Conversion {
//             exchange_rate: 1.2345,
//             from: "AED".to_owned(),
//             to: "AED".to_owned(),
//         };
    
//         (StatusCode::OK, Json(conversion))
//     }
    
//     #[derive(Serialize)]
//     struct Currency {
//         name: String,
//         iso_code: String,
//     }
    
//     #[derive(Serialize)]
//     struct Conversion {
//         exchange_rate: f32,
//         from: String,
//         to: String,
//     }
    


// async fn main() {
// #[tokio::main]
// tokio::spawn(async {
//     let app = Router::new()
//     .route("/", get(root))
//     .route("/currencies", get(currency_list_route))
//     .route("/convert", post(convert_route));

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
//         .await
//         .expect("Failed to bind server");

//     tracing::debug!("Listening on {}", listener.local_addr().unwrap());

//     if let Err(e) = axum::serve(listener, app).await {
//         tracing::error!("Server error: {}", e);
//     }
// });
// }


// #[tokio::main]
pub async fn server_start() {

let connection = sqlite::open(":memory:").unwrap();

let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
";
connection.execute(query).expect("Error sqlite");


let query2 = "
    SELECT Name FROM users;
";


let result = connection.iterate(query2,  |res.|{

    println!("First row: {}", res);
    return false;
});



}