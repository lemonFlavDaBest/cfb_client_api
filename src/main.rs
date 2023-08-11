use reqwest::header;
use dotenv::dotenv;
use std::collections::HashMap;

dotenv().ok();

let api_key = std::env::var!("CFB_API_KEY").expect("MAILCOACH_API_TOKEN must be set.");
let mut headers = header::HeaderMap::new();
let mut auth_value = header::HeaderValue::from_static({});
auth_value.set_sensitive(true);
headers.insert(header::AUTHORIZATION, "Bearer {}", auth_value);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}

