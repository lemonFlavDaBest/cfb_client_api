// players_endpoint.rs

use reqwest::Error;
use serde::Deserialize;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const PLAYER_ENDPOINT: &str = "player";
const SEARCH_ENDPOINT: &str = "player/search";
const USAGE_ENDPOINT: &str = "player/usage";
const RETURNING_ENDPOINT: &str = "returning";
const STATS_ENDPOINT: &str = "stats";
const PORTAL_ENDPOINT: &str = "player/portal";
const SEASON_ENDPOINT: &str = "season";

pub struct PlayerSearchParams<'a> {
    searchTerm: &'a str,
    position: Option<&'a str>,
    team: Option<&'a str>,
    year: Option<&'a str>,
}

pub struct PlayerUsageParams<'a> {
    year: &'a str,
    team: Option<&'a str>,
    conference: Option<&'a str>,
    position: Option<&'a str>,
    playerId: Option<&'a str>,
    excludeGarbageTime: Option<&'a str>,
}

pub struct PlayerReturningParams<'a> {
    year: Option<&'a str>,
    team: Option<&'a str>,
    conference: Option<&'a str>,
}

pub struct PlayerSeasonStatsParams<'a> {
    year: &'a str,
    team: Option<&'a str>,
    conference: Option<&'a str>,
    startWeek: Option<&'a str>,
    endWeek: Option<&'a str>,
    seasonType: Option<&'a str>,
    category: Option<&'a str>,
}

pub struct PlayerPortalParams<'a> {
    year:&'a str,
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

impl PlayerUsageParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        params.push(("year", self.year));
        // Add other fields if they exist in self
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(position) = self.position {
            params.push(("position", position));
        }
        if let Some(playerId) = self.playerId {
            params.push(("playerId", playerId));
        }
        if let Some(excludeGarbageTime) = self.excludeGarbageTime {
            params.push(("excludeGarbageTime", excludeGarbageTime));
        }
        params
    }
}

impl PlayerReturningParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        params
    }
}

impl PlayerSeasonStatsParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        params.push(("year", self.year));
        // Add other fields if they exist in self
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(startWeek) = self.startWeek {
            params.push(("startWeek", startWeek));
        }
        if let Some(endWeek) = self.endWeek {
            params.push(("endWeek", endWeek));
        }
        if let Some(seasonType) = self.seasonType {
            params.push(("seasonType", seasonType));
        }
        if let Some(category) = self.category {
            params.push(("category", category));
        }
        params
    }
}

impl PlayerPortalParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        params.push(("year", self.year));
        params
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSearchResult {
    id: Option<i64>,
    team: Option<String>,
    name: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    weight: Option<u16>,
    height: Option<u16>,
    jersey: Option<u8>,
    position: Option<String>,
    hometown: Option<String>,
    team_color: Option<String>,
    team_color_secondary: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerUsage {
    season: Option<u64>,
    id: Option<i64>,
    name: Option<String>,
    position: Option<String>,
    team: Option<String>,
    conference: Option<String>,
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    overall: Option<f32>,
    pass: Option<f32>,
    rush: Option<f32>,
    first_down: Option<f32>,
    second_down: Option<f32>,
    third_down: Option<f32>,
    standard_downs: Option<f32>,
    passing_downs: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReturningProduction {
    season: Option<u64>,
    team: Option<String>,
    conference: Option<String>,
    total_ppa: Option<f32>,
    total_passing_ppa: Option<f32>,
    total_receiving_ppa: Option<f32>,
    total_rushing_ppa: Option<f32>,
    usage: Option<f32>,
    passing_usage: Option<f32>,
    receiving_usage: Option<f32>,
    rushing_usage: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSeasonStat {
    season: Option<u64>,
    player_id: Option<i64>,
    player: Option<String>,
    team: Option<String>,
    conference: Option<String>,
    category: Option<String>,
    stat_type: Option<String>,
    stat: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalPlayer {
    season: Option<u64>,
    first_name: Option<String>,
    last_name: Option<String>,
    position: Option<String>,
    origin: Option<String>,
    destination: Option<String>,
    transfer_date: Option<String>, //need to do date conversion
    rating: Option<f32>,
    stars: Option<u8>,
    eligibility: Option<String>,
}

pub async fn get_player_search_with_params(api_client: &ApiClient, params: PlayerSearchParams<'_>) -> Result<PlayerSearchResult, Error> {
    let endpoint =  SEARCH_ENDPOINT;
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: PlayerSearchResult = response.json().await?;
    Ok(json_response)
}

pub async fn get_player_usage_with_params(api_client: &ApiClient, params: PlayerUsageParams<'_>) -> Result<PlayerUsage, Error> {
    let endpoint  = USAGE_ENDPOINT;
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: PlayerUsage = response.json().await?;
    Ok(json_response)
}

pub async fn get_player_returning_with_params(api_client: &ApiClient, params: Option<PlayerReturningParams<'_>>) -> Result<ReturningProduction, Error> {
    let endpoint = format!("{}/{}", PLAYER_ENDPOINT, RETURNING_ENDPOINT);
    let response = match params {
        Some(params) => api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?,
        None => api_client.get_endpoint(&endpoint).await?,
    };
    let json_response: ReturningProduction = response.json().await?;
    Ok(json_response)
}

pub async fn get_player_season_stats(api_client: &ApiClient, params: PlayerSeasonStatsParams<'_>) -> Result<PlayerSeasonStat, Error> {
    let endpoint = format!("{}/{}/{}", STATS_ENDPOINT, PLAYER_ENDPOINT, SEASON_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: PlayerSeasonStat= response.json().await?;
    Ok(json_response)
}

pub async fn get_player_portal(api_client: &ApiClient, params: PlayerPortalParams<'_>) -> Result<PortalPlayer, Error> {
    let endpoint = PORTAL_ENDPOINT;
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: PortalPlayer = response.json().await?;
    Ok(json_response)
}