mod api_client; 
mod endpoints; 

use dotenv::dotenv;
use api_client::ApiClient;
//use reqwest::Response;
//use serde::Deserialize; 
//use polars::prelude::*;
use endpoints::calendar_endpoint::{get_calendar, Week};
use endpoints::games_endpoint::{get_games_with_params, Game, GamesParams};
use endpoints::plays_endpoint::{get_plays_with_params, get_all_plays_for_year_range, Play};
use endpoints::scoreboard_endpoint::{get_scoreboard, get_scoreboard_with_params, ScoreboardGame, ScoreboardParams};
use polars::prelude::*;
use reqwest::Error;
use std::fs::File;
use std::io::{Cursor, Write}; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = std::env::var("CFB_API_KEY").expect("CFB_API_KEY must be set.");
    let api_client = ApiClient::new(&api_key)?;

    //testing calendar range function here 
    {
    //let start_year = 2016;
    //let end_year = 2017;
    //let response = get_calendar_year_range(&api_client, start_year, end_year).await?;
    //println!("{:#?}", response);
    //print length of response
    //println!("length of response: {}", response.len());
    }

    //testing the games for years function here
    {
    //let start_year = 2016;
    //let end_year = 2017;
    
    //let response = get_all_games_for_years(&api_client, start_year, end_year).await?;
    
    //println!("{:#?}", response);
    //Ok(())
    }

    //testing the plays endpoint here 
    {
    //let year: &str = "2021";
    //let week: &str = "1";

    //let response: Vec<PlaysResponse> = get_plays_with_params(&api_client, year, week, None).await?;
    //println!("{:#?}", response);
    }

    //testing get_all_plays_for_year_range here
    //ready tot test tomorrows
    {
        //let start_year = 2016;
        //let end_year = 2027;
        //let plays_data = get_all_plays_for_year_range(&api_client, start_year, end_year, 1, 15).await?;
        //println!("{:#?}", response);
        // Create a DataFrame from the plays_data
        //let json_string = serde_json::to_string(&plays_data)?;

        // Specify the path and filename to save the JSON file
        //let filename = format!("plays_response_{}_{}.json", start_year, end_year);

        // Create or open the file for writing
        //let mut file = File::create(&filename)?;

        // Write the JSON string to the file
        //file.write_all(json_string.as_bytes())?;
    
        // Print a success message
        //println!("JSON response saved as: {}", filename);
    
    
    }

    //testing the scoreboard endpoint here
    {
        let scoreboard_response = get_scoreboard(&api_client).await?;
        println!("{:#?}", scoreboard_response);

        let scoreboard_params = ScoreboardParams {
            classification: Some("fbs"),
            conference: None,
        };
        let scoreboard_response = get_scoreboard_with_params(&api_client, Some(scoreboard_params)).await?;
        println!("{:#?}", scoreboard_response);
    }
    Ok(())

}

async fn _get_calendar_year(api_client: &ApiClient, year: &str) -> Result<Vec<Week>, Box<dyn std::error::Error>> {
    let response = get_calendar(api_client, year).await?;
    Ok(response)
}

async fn get_calendar_year_range(api_client: &ApiClient, mut start_year: i32, end_year: i32) -> Result<Vec<Week>, Box<dyn std::error::Error>> {
    let mut calendar_responses: Vec<Week> = Vec::new();;
    while start_year < end_year {
        let year_str = start_year.to_string();
        let response: Vec<Week> = get_calendar(&api_client, &year_str).await?;
        calendar_responses.extend(response);
        start_year += 1;
    }

    println!("{:#?}", calendar_responses);
    Ok(calendar_responses)
}

async fn get_games_full_season_fbs(api_client: &ApiClient, year: &str) -> Result<Vec<Game>, Error> {
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

async fn get_all_games_for_years(api_client: &ApiClient, mut start_year: i32, end_year: i32) -> Result<Vec<Game>, Error> {
    let mut all_games = Vec::new();

    while start_year < end_year {
        let year_str = start_year.to_string();
        let games_for_year = get_games_full_season_fbs(&api_client, &year_str).await?;
        all_games.extend(games_for_year);
        start_year += 1;
    }

    Ok(all_games)
}