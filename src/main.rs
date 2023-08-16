mod api_client; 
mod endpoints; 

use dotenv::dotenv;
use api_client::ApiClient;
//use reqwest::Response;
//use serde::Deserialize; 
use endpoints::calendar_endpoint::{get_calendar, CalendarResponse};
use endpoints::games_endpoint::{get_games_with_params, GamesResponse};
use endpoints::plays_endpoint::{get_plays_with_params, PlaysResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = std::env::var("CFB_API_KEY").expect("CFB_API_KEY must be set.");
    let api_client = ApiClient::new(&api_key)?;

    //testing the calendar endpoint here. 
    {
    let year = "2021";  
    
    let response: Vec<CalendarResponse> = get_calendar(&api_client, year).await?;
    
    println!("{:#?}", response);
    //Ok(())
    }
    
    //testing the games endpoint here. 
    {
    let year: &str = "2021";

    let response: Vec<GamesResponse> = get_games_with_params(&api_client, year, None).await?;

    println!("{:#?}", response);

    //Ok(())
    }

    //testing the plays endpoint here 
    
    let year: &str = "2021";
    let week: &str = "1";

    let response: Vec<PlaysResponse> = get_plays_with_params(&api_client, year, week, None).await?;
    println!("{:#?}", response);
    
    Ok(())

}

