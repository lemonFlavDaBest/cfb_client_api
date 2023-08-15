// games_endpoint.rs

use reqwest::{Error, Response};
use serde::Deserialize;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const GAMES_ENDPOINT: &str = "games";

//define the response struct
#[derive(Deserialize, Debug)]
pub struct GamesResponse {
    pub id: u32,
    pub season: u32,
    pub week: u32,
    pub season_type: String,
    pub start_date: String,
    pub start_time_tbd: bool,
    pub completed: bool,
    pub neutral_site: bool,
    pub conference_game: bool,
    pub attendance: u32,
    pub venue_id: u32,
    pub venue: String,
    pub home_id: u32,
    pub home_team: String,
    pub home_conference: String,
    pub home_division: String,
    pub home_points: u32,
    pub home_line_scores: Vec<u32>,
    pub home_post_win_prob: u32,
    pub home_pregame_elo: u32,
    pub home_postgame_elo: u32,
    pub away_id: u32,
    pub away_team: String,
    pub away_conference: String,
    pub away_division: String,
    pub away_points: u32,
    pub away_line_scores: Vec<u32>,
    pub away_post_win_prob: u32,
    pub away_pregame_elo: u32,
    pub away_postgame_elo: u32,
    pub excitement_index: u32,
    pub highlights: String,
    pub notes: String,
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

    // Deserialize the response into GamesResponse struct
    let json_response: Vec<GamesResponse> = response.json().await?;
    println!("json_response: {:?}", json_response);

    Ok(json_response)
}