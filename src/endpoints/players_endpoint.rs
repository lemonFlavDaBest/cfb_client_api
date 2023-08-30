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
pub struct PlayerSearchResponse {}

#[derive(Debug, Deserialize)]
pub struct PlayerUsageResponse {}

#[derive(Debug, Deserialize)]
pub struct PlayerReturningResponse {}

#[derive(Debug, Deserialize)]
pub struct PlayerSeasonStatsResponse {}

#[derive(Debug, Deserialize)]
pub struct PlayerPortalResponse {}

pub async fn get_player_search_with_params(api_client: &ApiClient, params: PlayerSearchParams<'_>) -> Result<PlayerSearchResponse, Error> {
    let endpoint = format!("{}/{}", PLAYER_ENDPOINT, SEARCH_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: PlayerSearchResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_player_usage_with_params(api_client: &ApiClient, params: PlayerUsageParams<'_>) -> Result<PlayerUsageResponse, Error> {
    let endpoint = format!("{}/{}", PLAYER_ENDPOINT, USAGE_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: PlayerUsageResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_player_returning_with_params(api_client: &ApiClient, params: Option<PlayerReturningParams<'_>>) -> Result<PlayerReturningResponse, Error> {
    let endpoint = format!("{}/{}", PLAYER_ENDPOINT, RETURNING_ENDPOINT);
    let response = match params {
        Some(params) => api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?,
        None => api_client.get_endpoint(&endpoint).await?,
    };
    let json_response: PlayerReturningResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_player_season_stats(api_client: &ApiClient, params: PlayerSeasonStatsParams<'_>) -> Result<PlayerSeasonStatsResponse, Error> {
    let endpoint = format!("{}/{}/{}", STATS_ENDPOINT, PLAYER_ENDPOINT, SEASON_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: PlayerSeasonStatsResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_player_portal(api_client: &ApiClient, params: PlayerPortalParams<'_>) -> Result<PlayerPortalResponse, Error> {
    let endpoint = format!("{}/{}", PLAYER_ENDPOINT, PORTAL_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: PlayerPortalResponse = response.json().await?;
    Ok(json_response)
}