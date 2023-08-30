// players_endpoint.rs

use chrono::{DateTime, Utc, TimeZone, format};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const PLAYER_ENDPOINT: &str = "player";
const SEARCH_ENDPOINT: &str = "search";
const USAGE_ENDPOINT: &str = "usage";
const RETURNING_ENDPOINT: &str = "returning";
const STATS_ENDPOINT: &str = "stats";
const PORTAL_ENDPOINT: &str = "portal";
const SEASON_ENDPOINT: &str = "season";

pub struct PlayerSearchParams<'a> {
    searchTerm: &'a str,
    position: Option<&'a str>,
    team: Option<&'a str>,
    year: Option<&'a str>,
}

impl PlayerSearchParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        params.push(("searchTerm", self.searchTerm));
        // Add other fields if they exist in self
        if let Some(position) = self.position {
            params.push(("position", position));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        params
    }
}

#[derive(Debug, Deserialize)]
pub struct PlayerSearchResponse {}

pub async fn get_player_search_with_params(api_client: &ApiClient, params: PlayerSearchParams<'_>) -> Result<PlayerSearchResponse, Error> {
    let endpoint = format!("{}/{}", PLAYER_ENDPOINT, SEARCH_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: PlayerSearchResponse = response.json().await?;
    Ok(json_response)
}
