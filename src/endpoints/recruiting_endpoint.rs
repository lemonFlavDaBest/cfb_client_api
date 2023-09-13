// recruiting_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const RECRUITING_ENDPOINT: &str = "recruiting";
const PLAYERS_ENDPOINT: &str = "players";
const TEAMS_ENDPOINT: &str = "teams";
const GROUPS_ENDPOINT: &str = "groups";
const RECRUITING_PLAYERS_ENDPOINT: &str = "recruiting/players";
const RECRUITING_TEAMS_ENDPOINT: &str = "recruiting/teams";
const RECRUITING_GROUPS_ENDPOINT: &str = "recruiting/groups";

//define the params struct
pub struct RecruitingPlayersParams<'a> {
    year: Option<&'a str>, //required if no team
    classification: Option<&'a str>, // HighSchool, JUCO, PrepSchool
    position: Option<&'a str>,
    state: Option<&'a str>,
    team: Option<&'a str>, //required if no year
}

pub struct RecruitingTeamsParams<'a>{
    year: Option<&'a str>,
    team: Option<&'a str>,
}

pub struct RecruitingGroupsParams<'a>{
    startYear: Option<&'a str>,
    endYear: Option<&'a str>,
    team: Option<&'a str>,
    conference: Option<&'a str>,
}

impl RecruitingPlayersParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // either year or team must exist, the rest are optional
        if let Some(year) = self.year {
            params.push(("year", year));
            // check if team exists alos, if so, add it
            if let Some(team) = self.team {
                params.push(("team", team));
            }
        } else if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(classification) = self.classification {
            params.push(("classification", classification));
        }
        if let Some(position) = self.position {
            params.push(("position", position));
        }
        if let Some(state) = self.state {
            params.push(("state", state));
        }

        params
    }
}

impl RecruitingTeamsParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // either year or team must exist, the rest are optional
        if let Some(year) = self.year {
            params.push(("year", year));
        } 
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        params
    }
}

impl RecruitingGroupsParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // either year or team must exist, the rest are optional
        if let Some(startYear) = self.startYear {
            params.push(("startYear", startYear));
        } 
        if let Some(endYear) = self.endYear {
            params.push(("endYear", endYear));
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecruitingPlayersResponse {
    id: Option<i64>,
    athlete_id: Option<i64>,
    recruit_type: Option<String>,
    year: Option<u64>,
    name: Option<String>,
    school: Option<String>,
    committed_to: Option<String>,
    position: Option<String>,
    height: Option<f64>,
    weight: Option<u64>,
    stars: Option<u8>,
    rating: Option<f64>,
    city: Option<String>,
    state_province: Option<String>,
    country: Option<String>,
    hometown_info: Option<Hometown>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Hometown {
    latitude: Option<f64>,
    longitude: Option<f64>,
    county_fips: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct RecruitingTeamsResponse {
    year: Option<u64>,
    rank: Option<u64>,
    team: Option<String>,
    points: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecruitingGroupsResponse {
    team: Option<String>,
    conference: Option<String>,
    position_group: Option<String>,
    average_rating: Option<f64>,
    total_rating: Option<f64>,
    commits: Option<f64>,
    average_starts: Option<f64>,
}

pub async fn get_recruiting_players_with_params(api_client: &ApiClient, params: RecruitingPlayersParams<'_>) -> Result<RecruitingPlayersResponse, Error> {
    let endpoint = RECRUITING_PLAYERS_ENDPOINT;
    let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
    let json_response: RecruitingPlayersResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_recruiting_teams(api_client: &ApiClient, params: Option<RecruitingTeamsParams<'_>>) -> Result<RecruitingTeamsResponse, Error> {
    let endpoint = RECRUITING_TEAMS_ENDPOINT
    let response = match params {
        Some(params) => {
            api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?
        }
        None => {
            api_client.get_endpoint(&endpoint).await?
        }
    };
    let json_response: RecruitingTeamsResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_recruiting_groups(api_client: &ApiClient, params: Option<RecruitingGroupsParams<'_>>) -> Result<RecruitingGroupsResponse, Error> {
    let endpoint = RECRUITING_GROUPS_ENDPOINT;
    let response = match params {
        Some(params) => {
            api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?
        }
        None => {
            api_client.get_endpoint(&endpoint).await?
        }
    };
    let json_response: RecruitingGroupsResponse = response.json().await?;
    Ok(json_response)
}