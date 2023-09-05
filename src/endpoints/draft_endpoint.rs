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

pub struct DraftPicksParams<'a>{
    year: Option<&'a str>,
    nflTeam: Option<&'a str>,
    college: Option<&'a str>,
    conference: Option<&'a str>,
    position: Option<&'a str>,
}

impl DraftPicksParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        if let Some(nflTeam) = self.nflTeam {
            params.push(("nflTeam", nflTeam));
        }
        if let Some(college) = self.college {
            params.push(("college", college));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(position) = self.position {
            params.push(("position", position));
        }
        params
    }
}

#[derive(Deserialize, Debug)]
pub struct DraftTeamsResponse{
    location: Option<String>,
    nickname: Option<String>,
    display_name: Option<String>,
    logo: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DraftPositionsResponse{
    name: Option<String>,
    abbreviation: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DraftPicksResponse {
    college_athlete_id: Option<i64>,
    nfl_athlete_id: Option<i64>,
    college_id: Option<i64>,
    college_team: Option<String>,
    college_conference: Option<String>,
    nfl_team: Option<String>,
    year: Option<u64>,
    overall: Option<u64>,
    round: Option<u8>,
    pick: Option<u32>,
    name: Option<String>,
    position: Option<String>,
    height: Option<i8>,
    weight: Option<i16>,
    pre_draft_ranking: Option<u32>,
    pre_draft_position_ranking: Option<u32>,
    pre_draft_grade: Option<i64>,
    hometown_info: Option<Hometown>,
}
#[derive(Deserialize, Debug)]
pub struct Hometown {}

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

pub async fn get_draft_picks(api_client: &ApiClient, params: Option<DraftPicksParams<'_>>) -> Result<DraftPicksResponse, Error> {
    let draft_picks_response: DraftPicksResponse = match params {
        Some(params) => {
            let response = api_client.get_endpoint_with_params(PICKS_ENDPOINT, params.as_query_params()).await?;
            response.json().await?
        },
        None => {
            let endpoint = PICKS_ENDPOINT;
            let response = api_client.get_endpoint(endpoint).await?;
            response.json().await?
        }
    };
    Ok(draft_picks_response)
}