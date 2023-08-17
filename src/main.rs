mod api_client; 
mod endpoints; 

use dotenv::dotenv;
use api_client::ApiClient;
//use reqwest::Response;
//use serde::Deserialize; 
//use polars::prelude::*;
use endpoints::calendar_endpoint::{get_calendar, CalendarResponse};
use endpoints::games_endpoint::{get_games_with_params, GamesResponse, GamesParams};
use endpoints::plays_endpoint::{get_plays_with_params, PlaysResponse};
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = std::env::var("CFB_API_KEY").expect("CFB_API_KEY must be set.");
    let api_client = ApiClient::new(&api_key)?;

    //testing calendar range function here 
    {
    let start_year = 2010;
    let end_year = 2014;
    let response = get_calendar_year_range(&api_client, start_year, end_year).await?;
    println!("{:#?}", response);
    }

    //testing the games endpoint here. 
    {
    let year: &str = "2022";
    
    let response = get_games_full_season_fbs(&api_client, year).await?;
    
    println!("{:#?}", response);
    //Ok(())
    }

    //testing the plays endpoint here 
    {
    //let year: &str = "2021";
    //let week: &str = "1";

    //let response: Vec<PlaysResponse> = get_plays_with_params(&api_client, year, week, None).await?;
    //println!("{:#?}", response);
    }
    
    Ok(())

}

async fn get_calendar_year(api_client: &ApiClient, year: &str) -> Result<Vec<CalendarResponse>, Box<dyn std::error::Error>> {
    let response = get_calendar(api_client, year).await?;
    Ok(response)
}

async fn get_calendar_year_range(api_client: &ApiClient, mut start_year: i32, end_year: i32) -> Result<Vec<CalendarResponse>, Box<dyn std::error::Error>> {
    let mut calendar_responses: Vec<CalendarResponse> = Vec::new();;
    while start_year < end_year {
        let year_str = start_year.to_string();
        let response: Vec<CalendarResponse> = get_calendar(&api_client, &year_str).await?;
        calendar_responses.extend(response);
        start_year += 1;
    }

    println!("{:#?}", calendar_responses);
    Ok(calendar_responses)
}

async fn get_games_full_season_fbs(api_client: &ApiClient, year: &str) -> Result<Vec<GamesResponse>, Error> {
    let mut game_params = GamesParams::new();
    game_params.division = Some("fbs");

    // Get regular season games
    let mut response = get_games_with_params(&api_client, year, Some(game_params)).await?;
    
    // Update params for postseason
    let mut post_game_params = GamesParams::new();
    post_game_params.seasonType = Some("postseason");
    
    // Get postseason games
    let post_season_response = get_games_with_params(&api_client, year, Some(post_game_params)).await?;

    // Combine the responses into a single vector
    response.extend(post_season_response);

    Ok(response)
}

async fn get_all_games_for_years(api_client: &ApiClient) -> Result<Vec<GamesResponse>, Error> {
    let mut all_games = Vec::new();

    for year in 2016..=2023 {
        let year_str = year.to_string();
        let games_for_year = get_games_full_season_fbs(&api_client, &year_str).await?;
        all_games.extend(games_for_year);
    }

    Ok(all_games)
}