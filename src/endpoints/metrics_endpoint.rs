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

pub struct PPATeamsParams<'a> {
    year: Option<&'a str>, // required if team not specified
    team: Option<&'a str>, // required if year not specified
    conference: Option<&'a str>,
    excludeGarbageTime: Option<&'a str>,
}

pub struct PPAGamesParams<'a> {
    year: &'a str,
    week: Option<&'a str>,
    team: Option<&'a str>,
    conference: Option<&'a str>,
    excludeGarbageTime: Option<&'a str>, // boolean
    seasonType: Option<&'a str>, // regular or postseason
}

pub struct PPAPlayersGamesParams<'a> {
    year: Option<&'a str>,
    week: Option<&'a str>,
    team: Option<&'a str>,
    position: Option<&'a str>,
    playerId: Option<&'a str>,
    threshold: Option<&'a str>,
    excludeGarbageTime: Option<&'a str>,
    seasonType: Option<&'a str>, // default to regular
}

pub struct PPAPlayersSeasonParams<'a> {
    year: Option<&'a str>,
    team: Option<&'a str>,
    conference: Option<&'a str>,
    position: Option<&'a str>,
    playerId: Option<&'a str>,
    threshold: Option<&'a str>,
    excludeGarbageTime: Option<&'a str>,
}

pub struct MetricsWPParams<'a> {
    gameId: &'a str,
}

pub struct MetricsWPPregameParams<'a> {
    year: Option<&'a str>,
    week: Option<&'a str>,
    team: Option<&'a str>,
    seasonType: Option<&'a str>, // regular or postseason
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

impl PPATeamsParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let Some(year) = self.year {
            params.push(("year", year));
            // check if team exists alos, if so, add it
            if let Some(team) = self.team {
                params.push(("team", team));
            }
        } else if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(excludeGarbageTime) = self.excludeGarbageTime {
            params.push(("excludeGarbageTime", excludeGarbageTime));
        }
        params
    }
}

impl PPAGamesParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("year", self.year)];
        // Add other fields if they exist in self
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(excludeGarbageTime) = self.excludeGarbageTime {
            params.push(("excludeGarbageTime", excludeGarbageTime));
        }
        if let Some(seasonType) = self.seasonType {
            params.push(("seasonType", seasonType));
        } else {
            params.push(("seasonType", "regular")); // the default case
        }
        params
    }
}

impl PPAPlayersGamesParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
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
        if let Some(position) = self.position {
            params.push(("position", position));
        }
        if let Some(playerId) = self.playerId {
            params.push(("playerId", playerId));
        }
        if let Some(threshold) = self.threshold {
            params.push(("threshold", threshold));
        }
        if let Some(excludeGarbageTime) = self.excludeGarbageTime {
            params.push(("excludeGarbageTime", excludeGarbageTime));
        }
        if let Some(seasonType) = self.seasonType {
            params.push(("seasonType", seasonType));
        } else {
            params.push(("seasonType", "regular")); // the default case
        }
        params
    }
}

impl PPAPlayersSeasonParams<'_> {
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
        if let Some(position) = self.position {
            params.push(("position", position));
        }
        if let Some(playerId) = self.playerId {
            params.push(("playerId", playerId));
        }
        if let Some(threshold) = self.threshold {
            params.push(("threshold", threshold));
        }
        if let Some(excludeGarbageTime) = self.excludeGarbageTime {
            params.push(("excludeGarbageTime", excludeGarbageTime));
        }
        params
    }
}

impl MetricsWPParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("gameId", self.gameId)];
        params
    }
}

impl MetricsWPPregameParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
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
        if let Some(seasonType) = self.seasonType {
            params.push(("seasonType", seasonType));
        } 
        params
    }
}

#[derive(Debug, Deserialize)]
pub struct PPAPredictedResponse {
    yardline: Option<i32>,
    ppa: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct PPATeamsResponse {
    season: Option<u64>,
    team: Option<String>,
    conference: Option<String>,
    offense: Option<OffenseBox>,
    defense: Option<DefenseBox>,
}

#[derive(Debug, Deserialize)]
pub struct OffenseBox {
    overall: Option<f64>,
    passing: Option<f64>,
    rushing: Option<f64>,
    first_down: Option<f64>,
    second_down: Option<f64>,
    third_down: Option<f64>,
    cumulative: Option<CumulativeScores>,
}

#[derive(Debug, Deserialize)]
pub struct DefenseBox {
    overall: Option<f64>,
    passing: Option<f64>,
    rushing: Option<f64>,
    first_down: Option<f64>,
    second_down: Option<f64>,
    third_down: Option<f64>,
    cumulative: Option<CumulativeScores>,
}

#[derive(Debug, Deserialize)]
pub struct CumulativeScores {
    total: Option<f64>,
    passing: Option<f64>,
    rushing: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct PPAGamesResponse {}

#[derive(Debug, Deserialize)]
pub struct PPAPlayersGamesResponse {}

#[derive(Debug, Deserialize)]
pub struct PPAPlayersSeasonResponse {}

#[derive(Debug, Deserialize)]
pub struct MetricsWPResponse {}

#[derive(Debug, Deserialize)]
pub struct MetricsWPPregameResponse {}

pub async fn get_ppa_predicted_with_params(api_client: &ApiClient, params: PPAPredictedParams<'_>) -> Result<PPAPredictedResponse, Error> {
    let endpoint = PPA_PREDICTED_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: PPAPredictedResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_ppa_teams_with_params(api_client: &ApiClient, params: PPATeamsParams<'_>) -> Result<PPATeamsResponse, Error> {
    let endpoint = PPA_TEAMS_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: PPATeamsResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_ppa_games_with_params(api_client: &ApiClient, params: PPAGamesParams<'_>) -> Result<PPAGamesResponse, Error> {
    let endpoint = PPA_GAMES_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: PPAGamesResponse = response.json().await?;
    Ok(json_response)
}

async fn get_ppa_players_games_with_params(api_client: &ApiClient, params: PPAPlayersGamesParams<'_>) -> Result<PPAPlayersGamesResponse, Error> {
    let endpoint = PPA_PLAYERS_GAMES_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: PPAPlayersGamesResponse = response.json().await?;
    Ok(json_response)
}

async fn get_ppa_players_season_with_params(api_client: &ApiClient, params: PPAPlayersSeasonParams<'_>) -> Result<PPAPlayersSeasonResponse, Error> {
    let endpoint = PPA_PLAYERS_SEASON_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: PPAPlayersSeasonResponse = response.json().await?;
    Ok(json_response)
}

async fn get_metrics_wp_with_params(api_client: &ApiClient, params: MetricsWPParams<'_>) -> Result<MetricsWPResponse, Error> {
    let endpoint = METRICS_WP_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: MetricsWPResponse = response.json().await?;
    Ok(json_response)
}

async fn get_metrics_wp_pregame_with_params(api_client: &ApiClient, params: MetricsWPPregameParams<'_>) -> Result<MetricsWPPregameResponse, Error> {
    let endpoint = METRICS_WP_PREGAME_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: MetricsWPPregameResponse = response.json().await?;
    Ok(json_response)
}