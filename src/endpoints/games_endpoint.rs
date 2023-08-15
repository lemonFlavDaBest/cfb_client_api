// games_endpoint.rs

use reqwest::{Error, Response};
use serde::Deserialize;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const GAMES_ENDPOINT: &str = "games";

//define the response struct
#[derive(Deserialize, Debug)]
pub struct GamesResponse {
    id: u32,
    season: u32,
    week: u32,
    season_type: String,
    start_date: String, // Change this to DateTime<Utc> or your preferred date-time type
    start_time_tbd: bool,
    completed: bool,
    neutral_site: bool,
    conference_game: bool,
    attendance: Option<u32>,
    venue_id: u32,
    venue: String,
    home_id: u32,
    home_team: String,
    home_conference: String,
    home_division: String,
    home_points: u32,
    home_line_scores: Vec<u32>,
    home_post_win_prob: Option<f64>,
    home_pregame_elo: Option<u32>,
    home_postgame_elo: Option<u32>,
    away_id: u32,
    away_team: String,
    away_conference: String,
    away_division: String,
    away_points: u32,
    away_line_scores: Vec<u32>,
    away_post_win_prob: Option<f64>,
    away_pregame_elo: Option<u32>,
    away_postgame_elo: Option<u32>,
    excitement_index: Option<f64>,
    highlights: Option<String>,
    notes: Option<String>,
}

#[derive(Debug)]
pub struct GamesParams<'a> {
    year: &'a str,
    week: Option<&'a str>,
    seasonType: Option<&'a str>,
    team: Option<&'a str>,
    home: Option<&'a str>,
    away: Option<&'a str>,
    conference: Option<&'a str>,
    division: Option<&'a str>, //Division classification filter (fbs/fcs/ii/iii)
    id: Option<&'a str>, // id filter for querying a single game
}

impl GamesParams<'_> {
    fn new() -> Self {
        Default::default()
    }

    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("year", self.year)];
        // Add other fields if they exist in self
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(season_type) = self.seasonType {
            params.push(("seasonType", season_type));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(home) = self.home {
            params.push(("home", home));
        }
        if let Some(away) = self.away {
            params.push(("away", away));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(division) = self.division {
            params.push(("division", division));
        }
        if let Some(id) = self.id {
            params.push(("id", id));
        }  
        params
    }
}

impl Default for GamesParams<'_> {
    fn default() -> Self {
        GamesParams { 
            year: "2023", 
            week: None, 
            seasonType: Some("regular"),
            team: None,
            home: None,
            away: None,
            conference: None,
            division: None,
            id: None,
        } 
    }
}

//create function to get games that take the api client as a parameter and an optional parameters struct
pub async fn get_games_with_params(api_client: &ApiClient, year: &str, params: Option<GamesParams<'_>>) -> Result<Vec<GamesResponse>, Error> {
    let mut games_params = params.unwrap_or_else(GamesParams::new);
    games_params.year = year;

    let endpoint = GAMES_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, games_params.as_query_params()).await?;
    println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<GamesResponse> = response.json().await?;
    println!("json_response: {:?}", json_response);

    Ok(json_response)
}