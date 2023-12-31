// stats_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Serialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const STATS_ENDPOINT: &str = "stats";
const SEASON_ENDPOINT: &str = "stats/season";
const SEASON_ADVANCED_ENDPOINT: &str = "stats/season/advanced";
const GAME_ADVANCED_ENDPOINT: &str = "stats/game/advanced";
const CATEGORIES_ENDPOINT: &str = "stats/categories";

//define the params struct
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeasonStatsParams<'a> {
    year: Option<&'a str>, // required if team not specified
    team: Option<&'a str>, // required if year not specified
    conference: Option<&'a str>,
    start_week: Option<&'a str>,
    end_week: Option<&'a str>,
}
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SeasonStatsAdvancedParams<'a> {
    year: Option<&'a str>, // required if team not specified
    team: Option<&'a str>, // required if year not specified
    exclude_garbage_time: Option<&'a str>,
    start_week: Option<&'a str>,
    end_week: Option<&'a str>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameStatsAdvancedParams<'a> {
    year: Option<&'a str>, // required if team not specified
    week: Option<&'a str>,
    team: Option<&'a str>, // required if year not specified
    opponent: Option<&'a str>,
    exclude_garbage_time: Option<&'a str>, // boolean
    season_type: Option<&'a str>, // regular, postseason, or both
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
        if let Some(startWeek) = self.start_week {
            params.push(("startWeek", startWeek));
        }
        if let Some(endWeek) = self.end_week {
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
        if let Some(excludeGarbageTime) = self.exclude_garbage_time {
            params.push(("excludeGarbageTime", excludeGarbageTime));
        }
        if let Some(startWeek) = self.start_week {
            params.push(("startWeek", startWeek));
        }
        if let Some(endWeek) = self.end_week {
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
        if let Some(excludeGarbageTime) = self.exclude_garbage_time {
            params.push(("excludeGarbageTime", excludeGarbageTime));
        }
        if let Some(seasonType) = self.season_type {
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
#[serde(rename_all = "camelCase")]
pub struct AdvancedOffense {
    plays: Option<i32>,
    drives: Option<i32>,
    ppa: Option<f32>,
    total_ppa: Option<f32>,
    success_rate: Option<f32>,
    explosiveness: Option<f32>,
    power_success: Option<f32>,
    stuff_rate: Option<f32>,
    line_yards: Option<f32>,
    line_yards_total: Option<f32>,
    second_level_yards: Option<f32>,
    second_level_yards_total: Option<u32>,
    open_field_yards: Option<f32>,
    open_field_yards_total: Option<u32>,
    total_opportunities: Option<u32>,
    points_per_opportunity: Option<f32>,
    field_position: Option<FieldPosition>,
    havoc: Option<Havoc>,
    standard_downs: Option<StandardDowns>,
    passing_downs: Option<PassingDowns>,
    rushing_plays: Option<RushingPlays>,
    passing_plays: Option<PassingPlays>,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedDefense {
    plays: Option<i32>,
    drives: Option<i32>,
    ppa: Option<f32>,
    total_ppa: Option<f32>,
    success_rate: Option<f32>,
    explosiveness: Option<f32>,
    power_success: Option<f32>,
    stuff_rate: Option<f32>,
    line_yards: Option<f32>,
    line_yards_total: Option<f32>,
    second_level_yards: Option<f32>,
    second_level_yards_total: Option<u32>,
    open_field_yards: Option<f32>,
    open_field_yards_total: Option<u32>,
    total_opportunities: Option<u32>,
    points_per_opportunity: Option<f32>,
    field_position: Option<FieldPosition>,
    havoc: Option<Havoc>,
    standard_downs: Option<StandardDowns>,
    passing_downs: Option<PassingDowns>,
    rushing_plays: Option<RushingPlays>,
    passing_plays: Option<PassingPlays>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldPosition {
    average_start: Option<f32>,
    average_predicted_points: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Havoc {
    total_havoc: Option<f32>,
    front_seven: Option<f32>,
    db: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardDowns{
    rate: Option<f32>,
    ppa: Option<f32>,
    success_rate: Option<f32>,
    explosiveness: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PassingDowns {
    rate: Option<f32>,
    ppa: Option<f32>,
    success_rate: Option<f32>,
    explosiveness: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RushingPlays {
    rate: Option<f32>,
    ppa: Option<f32>,
    success_rate: Option<f32>,
    explosiveness: Option<f32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PassingPlays {
    rate: Option<f32>,
    ppa: Option<f32>,
    success_rate: Option<f32>,
    explosiveness: Option<f32>,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedGameStat {
    game_id: Option<i64>,
    season: Option<u64>,
    week: Option<u8>,
    team: Option<String>,
    opponent: Option<String>,
    offense: Option<AdvancedOffense>,
    defense: Option<AdvancedDefense>,
}

#[derive(Deserialize, Debug)]
pub struct StatsCategoriesResponse {}

pub async fn get_season_stats_with_params(api_client: &ApiClient, params: SeasonStatsParams<'_>) -> Result<TeamSeasonStat, Error> {
    let endpoint = SEASON_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: TeamSeasonStat = response.json().await?;
    Ok(json_response)
}

pub async fn get_season_stats_advanced_with_params(api_client: &ApiClient, params: SeasonStatsAdvancedParams<'_>) -> Result<AdvancedSeasonStat, Error> {
    let endpoint = SEASON_ADVANCED_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: AdvancedSeasonStat = response.json().await?;
    Ok(json_response)
}

pub async fn get_game_stats_advanced_with_params(api_client: &ApiClient, params: GameStatsAdvancedParams<'_>) -> Result<AdvancedGameStat, Error> {
    let endpoint = GAME_ADVANCED_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: AdvancedGameStat = response.json().await?;
    Ok(json_response)
}

pub async fn get_stats_categories(api_client: &ApiClient) -> Result<StatsCategoriesResponse, Error> {
    let endpoint = CATEGORIES_ENDPOINT;
    let response = api_client.get_endpoint(endpoint).await?;
    let json_response: StatsCategoriesResponse = response.json().await?;
    Ok(json_response)
}