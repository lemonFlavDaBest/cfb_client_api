// betting_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const BETTING_ENDPOINT: &str = "betting";

//define the params struct
pub struct BettingParams<'a> {
    gameId: Option<&'a str>,
    year: Option<&'a str>,
    week: Option<&'a str>,
    seasonType: Option<&'a str>,
    team: Option<&'a str>,
    conference: Option<&'a str>,
    home: Option<&'a str>,
    away: Option<&'a str>,
}

impl BettingParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let Some(gameId) = self.gameId {
            params.push(("gameId", gameId));
        }
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        // Default to regular season
        if let Some(seasonType) = self.seasonType {
            params.push(("seasonType", seasonType));
        } else {
            params.push(("seasonType", "regular"));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(home) = self.home {
            params.push(("home", home));
        }
        if let Some(away) = self.away {
            params.push(("away", away));
        }
        params
    }
}

#[derive(Debug, Deserialize)]
pub struct BettingResponse {
    id: Option<i32>,
    season: Option<i32>,
    week: Option<u16>,
    season_type: Option<String>,
    start_date: Option<String>,
    home_team: Option<String>,
    home_conference: Option<String>,
    home_score: Option<u16>,
    away_team: Option<String>,
    away_conference: Option<String>,
    away_score: Option<u16>,
    lines: Option<Vec<Line>>,
}

pub async fn get_betting_with_params(api_client: &ApiClient, params: BettingParams<'_>) -> Result<BettingResponse, Error> {
    let endpoint = BETTING_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: BettingResponse = response.json().await?;
    Ok(json_response)
}