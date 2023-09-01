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

pub struct PPAPredictedResponse {}

pub struct PPATeamsResponse {}

pub struct PPAGamesResponse {}

pub struct PPAPlayersGamesResponse {}

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
