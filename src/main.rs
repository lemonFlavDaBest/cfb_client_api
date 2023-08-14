mod api_client; 
mod endpoints; 

use dotenv::dotenv;
use api_client::ApiClient;
use reqwest::Response;
//use serde::Deserialize; 
//use endpoints::calendar_endpoint::get_calendar;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = std::env::var("CFB_API_KEY").expect("CFB_API_KEY must be set.");
    let api_client = ApiClient::new(&api_key)?;

    // let year = "2022";  Set the year for the query
    println!("Client: {:?}", api_client);
    

    Ok(())
}

