//plays_details.rs


use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc, TimeZone, NaiveDateTime};
use serde_json::{Value};
use std::str::FromStr;
use tqdm::tqdm;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const PLAYS_ENDPOINT: &str = "plays";
const LIVE_ENDPOINT: &str = "live";
const TYPES_ENDPOINT: &str = "types";
const STATS_ENDPOINT: &str = "stats";
const STAT_ENDPOINT: &str = "stat";
const PLAY_ENDPOINT: &str = "play";

pub struct LivePlayParams<'a> {
    gameId: &'a str,
}
#[derive(Debug, Deserialize)]
pub struct LivePlayResponse{}

#[derive(Debug, Deserialize)]
pub struct PlayTypesResponse{}

pub struct PlayStatsParams<'a> {
    year: Option<&'a str>,
    week: Option<&'a str>,
    team: Option<&'a str>,
    gameId: Option<&'a str>,
    athleteId: Option<&'a str>,
    statTypeId: Option<&'a str>,
    seasonType: Option<&'a str>,
    conference: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct PlayStatsResponse{}

impl PlayStatsParams <'_> {
    pub fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(gameId) = self.gameId {
            params.push(("gameId", gameId));
        }
        if let Some(athleteId) = self.athleteId {
            params.push(("athleteId", athleteId));
        }
        if let Some(statTypeId) = self.statTypeId {
            params.push(("statTypeId", statTypeId));
        }
        if let Some(seasonType) = self.seasonType {
            params.push(("seasonType", seasonType));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        params
    }
}

pub async fn get_live_plays(api_client: &ApiClient, game_id: &str) -> Result<Vec<LivePlayResponse>, Error> {
    let params: Vec<(&str, &str)> = vec![("gameId", game_id)];
    //println!("Params: {:?}", params);
    let endpoint = format!("{}/{}", LIVE_ENDPOINT, PLAYS_ENDPOINT);
    
    let response = api_client.get_endpoint_with_params(&endpoint, params).await?;
    //println!("checkpoint");
    
    // Parse the response into JSON
    let json_response: Vec<LivePlayResponse> = response.json().await?;
    
    //println!("JSON Response: {:?}", json_response);
    Ok(json_response)
}

pub async fn get_play_types(api_client: &ApiClient) -> Result<Vec<PlayTypesResponse>, Error> {
    let endpoint = format!("{}/{}", PLAY_ENDPOINT, TYPES_ENDPOINT);
    let response = api_client.get_endpoint(&endpoint).await?;
    let json_response: Vec<PlayTypesResponse> = response.json().await?;
    Ok(json_response)
}

pub async fn get_play_stats(api_client: &ApiClient, params: PlayStatsParams<'_>) -> Result<Vec<PlayStatsResponse>, Error> {
    let endpoint = format!("{}/{}", PLAY_ENDPOINT, STATS_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: Vec<PlayStatsResponse> = response.json().await?;
    Ok(json_response)
}