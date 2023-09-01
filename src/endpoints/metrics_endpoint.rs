// metric_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const PPA_PREDICTED_ENDPOINT: &str = "ppa/predicted";
const PPA_TEAMS_ENDPOINT: &str = "ppa/teams";
const PPA_GAMES_ENDPOINT: &str = "ppa/games";
const PPA_PLAYERS_GAMES_ENDPOINT: &str = "ppa/players/games";
const PPA_PLAYERS_SEASON_ENDPOINT: &str = "ppa/players/season";
const METRICS_WP_ENDPOINT: &str = "metrics/wp";
const METRICS_WP_PREGAME_ENDPOINT: &str = "metrics/wp/pregame";

pub struct PPAPredictedParams<'a> {
    down: &'a str,
    distance: &'a str,
}

impl PPAPredictedParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        params.push(("down", self.down));
        params.push(("distance", self.distance));
        params
    }
}

pub struct PPAPredictedResponse {}

pub async fn get_ppa_predicted_with_params(api_client: &ApiClient, params: PPAPredictedParams<'_>) -> Result<PPAPredictedResponse, Error> {
    let endpoint = PPA_PREDICTED_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: PPAPredictedResponse = response.json().await?;
    Ok(json_response)
}