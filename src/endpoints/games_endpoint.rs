// games_endpoint.rs

use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const GAMES_ENDPOINT: &str = "games";

//define the response struct
#[derive(Deserialize, Debug)]
pub struct GamesResponse {
    id: u32,
    season: Option<u32>,
    week: Option<u32>,
    season_type: Option<String>,
    start_date: Option<String>, // Change this to DateTime<Utc> or your preferred date-time type
    start_time_tbd: Option<bool>,
    completed: Option<bool>,
    neutral_site: Option<bool>,
    conference_game: Option<bool>,
    attendance: Option<u32>,
    venue_id: Option<u32>,
    venue: Option<String>,
    home_id: Option<u32>,
    home_team: Option<String>,
    home_conference: Option<String>,
    home_division: Option<String>,
    home_points: Option<u32>,
    home_line_scores: Option<Vec<Option<u32>>>,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    home_post_win_prob: Option<f64>, //f64::from_str(&val_str).unwrap()
    home_pregame_elo: Option<u32>,
    home_postgame_elo: Option<u32>,
    away_id: Option<u32>,
    away_team: Option<String>,
    away_conference: Option<String>,
    away_division: Option<String>,
    away_points: Option<u32>,
    away_line_scores: Option<Vec<Option<u32>>>,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    away_post_win_prob: Option<f64>, //f64::from_str(&val_str).unwrap()
    away_pregame_elo: Option<u32>,
    away_postgame_elo: Option<u32>,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    excitement_index: Option<f64>, //f64::from_str(&val_str).unwrap()
    highlights: Option<String>,
    notes: Option<String>,
}

#[derive(Debug)]
pub struct GamesParams<'a> {
    year: &'a str,
    week: Option<&'a str>,
    pub seasonType: Option<&'a str>,
    team: Option<&'a str>,
    pub home: Option<&'a str>,
    pub away: Option<&'a str>,
    pub conference: Option<&'a str>,
    pub division: Option<&'a str>, //Division classification filter (fbs/fcs/ii/iii)
    pub id: Option<&'a str>, // id filter for querying a single game
}

impl GamesParams<'_> {
    pub fn new() -> Self {
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

fn deserialize_f64_from_str<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;
    match value {
        Some(val_str) => {
            // Parse the string value into f64
            let parsed_val = f64::from_str(&val_str)
                .map_err(|_err| serde::de::Error::custom("Failed to parse f64 from string"))?;
            Ok(Some(parsed_val))
        }
        None => Ok(None),
    }
}

//create function to get games that take the api client as a parameter and an optional parameters struct
pub async fn get_games_with_params(api_client: &ApiClient, year: &str, params: Option<GamesParams<'_>>) -> Result<Vec<GamesResponse>, Error> {
    let mut games_params = params.unwrap_or_else(GamesParams::new);
    games_params.year = year;

    let endpoint = GAMES_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, games_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<GamesResponse> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}