//lib.rs

//modules
mod api_client;
mod endpoints;

use reqwest::Error;

//exports
pub use api_client::ApiClient;
pub use endpoints::calendar_endpoint;
use endpoints::{calendar_endpoint::Week, games_endpoint::{Game, GamesParams, get_games_with_params}};

use crate::calendar_endpoint::get_calendar;


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