// stats_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const STATS_ENDPOINT: &str = "stats";
const SEASON_ENDPOINT: &str = "stats/season";
const SEASON_ADVANCED_ENDPOINT: &str = "stats/season/advanced";
const GAME_ADVANCED_ENDPOINT: &str = "stats/game/advanced";
const CATEGORIES_ENDPOINT: &str = "stats/categories";

//define the params struct
pub struct SeasonStatsParams<'a> {
    year: Option<&'a str>, // required if team not specified
    team: Option<&'a str>, // required if year not specified
    conference: Option<&'a str>,
    startWeek: Option<&'a str>,
    endWeek: Option<&'a str>,
}

pub struct SeasonStatsAdvancedParams<'a> {
    year: Option<&'a str>, // required if team not specified
    team: Option<&'a str>, // required if year not specified
    excludeGarbageTime: Option<&'a str>,
    startWeek: Option<&'a str>,
    endWeek: Option<&'a str>,
}

pub struct GameStatsAdvancedParams<'a> {
    year: Option<&'a str>, // required if team not specified
    week: Option<&'a str>,
    team: Option<&'a str>, // required if year not specified
    opponent: Option<&'a str>,
    excludeGarbageTime: Option<&'a str>, // boolean
    seasonType: Option<&'a str>, // regular, postseason, or both
}

impl SeasonStatsParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // one of year or team must be provided
        if let Some(year) = self.year {
            params.push(("year", year));

            if let Some(team) = self.team {
                params.push(("team", team));
            }

        } else if let Some(team) = self.team {
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
        params
    }
}

impl SeasonStatsAdvancedParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // one of year or team must be provided
        if let Some(year) = self.year {
            params.push(("year", year));

            if let Some(team) = self.team {
                params.push(("team", team));
            }

        } else if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(excludeGarbageTime) = self.excludeGarbageTime {
            params.push(("excludeGarbageTime", excludeGarbageTime));
        }
        if let Some(startWeek) = self.startWeek {
            params.push(("startWeek", startWeek));
        }
        if let Some(endWeek) = self.endWeek {
            params.push(("endWeek", endWeek));
        }
        params
    }
}

impl GameStatsAdvancedParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // one of year or team must be provided
        if let Some(year) = self.year {
            params.push(("year", year));

            if let Some(team) = self.team {
                params.push(("team", team));
            }

        } else if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(opponent) = self.opponent {
            params.push(("opponent", opponent));
        }
        if let Some(excludeGarbageTime) = self.excludeGarbageTime {
            params.push(("excludeGarbageTime", excludeGarbageTime));
        }
        if let Some(seasonType) = self.seasonType {
            params.push(("seasonType", seasonType));
        } 
        params
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamSeasonStat {
    season: Option<u64>,
    team: Option<String>,
    conference: Option<String>,
    stat_name: Option<String>,
    stat_value: Option<i16>,
}

#[derive(Deserialize, Debug)]
pub struct AdvancedSeasonStat {
    season: Option<u64>,
    team: Option<String>,
    conference: Option<String>,
    offense: Option<AdvancedOffense>,
    defense: Option<AdvancedDefense>,
}

#[derive(Deserialize, Debug)]
pub struct AdvancedOffense {

}

#[derive(Deserialize, Debug)]
pub struct GameStatsAdvancedResponse {}

#[derive(Deserialize, Debug)]
pub struct StatsCategoriesResponse {}

pub async fn get_season_stats_with_params(api_client: &ApiClient, params: SeasonStatsParams<'_>) -> Result<TeamSeasonStat, Error> {
    let endpoint = SEASON_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: TeamSeasonStat = response.json().await?;
    Ok(json_response)
}

pub async fn get_season_stats_advanced_with_params(api_client: &ApiClient, params: SeasonStatsAdvancedParams<'_>) -> Result<SeasonStatsAdvancedResponse, Error> {
    let endpoint = SEASON_ADVANCED_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: SeasonStatsAdvancedResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_game_stats_advanced_with_params(api_client: &ApiClient, params: GameStatsAdvancedParams<'_>) -> Result<GameStatsAdvancedResponse, Error> {
    let endpoint = GAME_ADVANCED_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: GameStatsAdvancedResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_stats_categories(api_client: &ApiClient) -> Result<StatsCategoriesResponse, Error> {
    let endpoint = CATEGORIES_ENDPOINT;
    let response = api_client.get_endpoint(endpoint).await?;
    let json_response: StatsCategoriesResponse = response.json().await?;
    Ok(json_response)
}