mod api_client; 
mod endpoints; 

use dotenv::dotenv;
use api_client::ApiClient;
use reqwest::Response;
//use serde::Deserialize; 
use endpoints::calendar_endpoint::get_calendar;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = std::env::var("CFB_API_KEY").expect("CFB_API_KEY must be set.");
    let api_client = ApiClient::new(&api_key)?;

    let year = "2023";  
    
    let response = get_calendar(&api_client, year).await?;
    
    println!("{:#?}", response);
    Ok(())
    
    //Below this is our working test code.

    //let resp: Response = api_client.client
   //     .get("https://api.collegefootballdata.com/calendar?year=2022")
 //       .send()
//        .await?;

//   println!("{:#?}", resp);
 //  let response_text = resp.text().await?;
 //  println!("Response Text: {}", response_text);
    
 //  Ok(())
}

