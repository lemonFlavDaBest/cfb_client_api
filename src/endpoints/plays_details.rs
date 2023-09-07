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
#[serde(rename_all = "camelCase")]
pub struct LivePlayResponse{
    id: Option<i64>,
    status: Option<String>,
    period: Option<i8>,
    possession: Option<String>,
    down: Option<u8>,
    distance: Option<i16>,
    yards_to_goal: Option<i16>,
    teams: Option<Vec<Teams>>,
    drives: Option<Vec<Drives>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teams {
    team_id: Option<i64>,
    team: Option<String>,
    home_away: Option<String>,
    points: Option<i16>,
    drives: Option<u16>,
    scoring_opportunities: Option<u16>,
    points_per_opportunity: Option<f64>,
    plays: Option<u16>,
    line_yards: Option<i64>,
    line_yards_per_rush: Option<f64>,
    second_level_yards: Option<i64>,
    second_level_yards_per_rush: Option<f64>,
    open_field_yards: Option<i64>,
    open_field_yards_per_rush: Option<f64>,
    epa_per_play: Option<f64>,
    total_epa: Option<f64>,
    passing_epa: Option<f64>,
    epa_per_pass: Option<f64>,
    rushing_epa: Option<f64>,
    epa_per_rush: Option<f64>,
    success_rate: Option<f64>,
    standard_downs_success_rate: Option<f64>,
    passing_downs_success_rate: Option<f64>,
    explosiveness: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drives {
    id: Option<i64>,
    offense_id: Option<i64>,
    offense: Option<String>,
    defense_id: Option<i64>,
    defense: Option<String>,
    play_count: Option<u16>,
    yards: Option<i16>,
    start_period: Option<i8>,
    start_clock: Option<String>,
    start_yards_to_goal: Option<i16>,
    end_period: Option<i8>,
    end_clock: Option<String>,
    end_yards_to_goal: Option<i16>,
    duration: Option<String>,
    scoring_play: Option<bool>,
    plays: Option<Vec<DrivePlays>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrivePlays {
    // add the rest of the fields
    id: Option<i64>,
    home_score: Option<i16>,
    away_score: Option<i16>,
    period: Option<i8>,
    clock: Option<String>,
    wallclock: Option<String>,
    team_id: Option<i64>,
    team: Option<String>,
    down: Option<u8>,
    distance: Option<i16>,
    yards_to_goal: Option<i16>,
    yards_gained: Option<i16>,
    play_type_id: Option<i64>,
    play_type: Option<String>,
    epa: Option<f64>,
    garbage_time: Option<bool>,
    success: Option<bool>,
    rush_pass: Option<String>,
    down_type: Option<String>,
    play_text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlayTypesResponse{
    id: Option<i64>,
    text: Option<String>,
    abbreviation: Option<String>,
}

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
#[serde(rename_all = "camelCase")]
pub struct PlayStatsResponse{
    game_id: Option<i64>,
    season: Option<u64>,
    week: Option<u8>,
    team: Option<String>,
    conference: Option<String>,
    opponent: Option<String>,
    team_score: Option<u16>,
    opponent_score: Option<u16>,
    drive_id: Option<i64>,
    play_id: Option<i64>,
    period: Option<u8>,
    clock: Option<Clock>,
    yards_to_goal: Option<u16>,
    down: Option<u8>,
    distance: Option<u8>,
    athlete_id: Option<i64>,
    stat_type: Option<String>,
    stat: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Clock {
    minutes: Option<u8>,
    seconds: Option<u8>,
}

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

#[derive(Debug, Deserialize)]
pub struct PlayStatTypesResponse{
    id: Option<i64>,
    name: Option<String>,
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

pub async fn get_play_stat_types(api_client: &ApiClient) -> Result<Vec<PlayStatTypesResponse>, Error> {
    let endpoint = format!("{}/{}/{}", PLAY_ENDPOINT, STAT_ENDPOINT, TYPES_ENDPOINT);
    let response = api_client.get_endpoint(&endpoint).await?;
    let json_response: Vec<PlayStatTypesResponse> = response.json().await?;
    Ok(json_response)
}