//draft_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const DRAFT_ENDPOINT: &str = "draft";
const TEAMS_ENDPOINT: &str = "draft/teams";
const POSITIONS_ENDPOINT: &str = "draft/positions";
const PICKS_ENDPOINT: &str = "draft/picks";

pub struct DraftPicksParams<'a>{}

#[derive(Deserialize, Debug)]
pub struct DraftTeamsResponse{}

#[derive(Deserialize, Debug)]
pub struct DraftPositionsResponse{}

pub async fn get_draft_teams(api_client: &ApiClient) -> Result<DraftTeamsResponse, Error> {
    let response = api_client.get_endpoint(TEAMS_ENDPOINT).await?;
    let json_response: DraftTeamsResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_draft_positions(api_client: &ApiClient) -> Result<DraftPositionsResponse, Error> {
    let response = api_client.get_endpoint(POSITIONS_ENDPOINT).await?;
    let json_response: DraftPositionsResponse = response.json().await?;
    Ok(json_response)
}