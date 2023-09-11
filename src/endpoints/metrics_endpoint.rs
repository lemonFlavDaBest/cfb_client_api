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
pub struct PredictedPoints {
    yardline: Option<i32>,
    ppa: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct TeamPPA {
    season: Option<u64>,
    team: Option<String>,
    conference: Option<String>,
    offense: Option<OffenseBox>,
    defense: Option<DefenseBox>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct GamePPA {
    game_id: Option<i64>,
    season: Option<u64>,
    week: Option<u16>,
    team: Option<String>,
    conference: Option<String>,
    opponent: Option<String>,
    offense: Option<TeamRankings>,
    defense: Option<TeamRankings>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamRankings {
    overall: Option<f64>,
    passing: Option<f64>,
    rushing: Option<f64>,
    first_down: Option<f64>,
    second_down: Option<f64>,
    third_down: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerGamePPA {
    season: Option<u64>,
    week: Option<u16>,
    name: Option<String>,
    position: Option<String>,
    team: Option<String>,
    opponent: Option<String>,
    average_ppa: Option<AveragePPA>,
}

#[derive(Debug, Deserialize)]
pub struct AveragePPA {
    all: Option<f64>,
    pass: Option<f64>,
    rush: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSeasonPPA {
    season: Option<u64>,
    id: Option<i64>,
    name: Option<String>,
    position: Option<String>,
    team: Option<String>,
    average_ppa: Option<SeasonAveragePPA>,
    total_ppa: Option<SeasonAveragePPA>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeasonAveragePPA {
    all: Option<f64>,
    pass: Option<f64>,
    rush: Option<f64>,
    first_down: Option<f64>,
    second_down: Option<f64>,
    third_down: Option<f64>,
    standard_downs: Option<f64>,
    passing_downs: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsWPResponse {
    games_id: Option<i64>,
    play_id: Option<i64>,
    play_text: Option<String>,
    home_id: Option<i64>,
    home: Option<String>,
    away_id: Option<i64>,
    away: Option<String>,
    spread: Option<f64>,
    home_ball: Option<bool>,
    home_score: Option<u32>,
    away_score: Option<u32>,
    time_remaining: Option<i64>,
    yard_line: Option<i64>,
    down: Option<u8>,
    distance: Option<u32>,
    home_win_prob: Option<f64>,
    play_number: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetricsWPPregameResponse {
    season: Option<u64>,
    season_type: Option<String>,
    week: Option<u16>,
    game_id: Option<i64>,
    home_team: Option<String>,
    away_team: Option<String>,
    spread: Option<f64>,
    home_win_prob: Option<f64>,
}

pub async fn get_ppa_predicted_with_params(api_client: &ApiClient, params: PPAPredictedParams<'_>) -> Result<PredictedPoints, Error> {
    let endpoint = PPA_PREDICTED_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: PredictedPoints = response.json().await?;
    Ok(json_response)
}

pub async fn get_ppa_teams_with_params(api_client: &ApiClient, params: PPATeamsParams<'_>) -> Result<TeamPPA, Error> {
    let endpoint = PPA_TEAMS_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: TeamPPA = response.json().await?;
    Ok(json_response)
}

pub async fn get_ppa_games_with_params(api_client: &ApiClient, params: PPAGamesParams<'_>) -> Result<GamePPA, Error> {
    let endpoint = PPA_GAMES_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: GamePPA = response.json().await?;
    Ok(json_response)
}

async fn get_ppa_players_games_with_params(api_client: &ApiClient, params: PPAPlayersGamesParams<'_>) -> Result<PlayerGamePPA, Error> {
    let endpoint = PPA_PLAYERS_GAMES_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: PlayerGamePPA = response.json().await?;
    Ok(json_response)
}

async fn get_ppa_players_season_with_params(api_client: &ApiClient, params: PPAPlayersSeasonParams<'_>) -> Result<PlayerSeasonPPA, Error> {
    let endpoint = PPA_PLAYERS_SEASON_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: PlayerGamePPA = response.json().await?;
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