// ratings_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const RATINGS_ENDPOINT: &str = "ratings";
const SP_ENDPOINT: &str = "ratings/sp";
const SRS_ENDPOINT : &str = "ratings/srs";
const SP_CONFERENCES_ENDPOINT: &str = "ratings/sp/conferences";
const ELO_ENDPOINT: &str = "ratings/elo";

//define the params struct
pub struct RatingsSPParams<'a> {
    year: Option<&'a str>,
    team: Option<&'a str>,
}

pub struct RatingsSRSParams<'a> {
    year: Option<&'a str>, //required if team not specified
    team: Option<&'a str>, //required if year not specified
    conference: Option<&'a str>,
}

pub struct RatingsSPConferencesParams<'a> {
    year: Option<&'a str>,
    conference: Option<&'a str>,
}

pub struct RatingsEloParams<'a> {
    year: Option<&'a str>,
    week: Option<&'a str>,
    team: Option<&'a str>,
    conference: Option<&'a str>,
}

impl RatingsSPParams<'_> {
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
        params
    }
}

impl RatingsSRSParams<'_>{
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
        params
    }
}

impl  RatingsSPConferencesParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // one of year or team must be provided
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        params
    }
}

impl RatingsEloParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // one of year or team must be provided
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        if let Some(week) = self.week {
            params.push(("week", week));
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingsSPResponse {
    year: Option<u64>,
    team: Option<String>,
    conference: Option<String>,
    rating: Option<f64>,
    ranking: Option<f64>,
    second_order_wins: Option<f64>,
    sos: Option<f64>,
    offense: Option<OffenseSP>,
    defense: Option<DefenseSP>,
    special_teams: Option<SpecialTeamsSP>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OffenseSP {
    ranking: Option<f64>,
    rating: Option<f64>,
    success: Option<f64>,
    explosiveness: Option<f64>,
    rushing: Option<f64>,
    passing: Option<f64>,
    standard_downs: Option<f64>,
    passing_downs: Option<f64>,
    run_rate: Option<f64>,
    pace: Option<f64>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefenseSP {
    ranking: Option<f64>,
    rating: Option<f64>,
    success: Option<f64>,
    explosiveness: Option<f64>,
    rushing: Option<f64>,
    passing: Option<f64>,
    standard_downs: Option<f64>,
    passing_downs: Option<f64>,
    havoc: Option<Havoc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Havoc {
    total: Option<f64>,
    front_seven: Option<f64>,
    db: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct SpecialTeamsSP {
    rating: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct RatingsSRSResponse {  
}

#[derive(Debug, Deserialize)]
pub struct RatingsSPConferencesResponse {}

pub async fn get_ratings_sp_with_params(api_client: &ApiClient, params: RatingsSPParams<'_>) -> Result<RatingsSPResponse, Error> {
    let endpoint = SP_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: RatingsSPResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_ratings_srs_with_params(api_client: &ApiClient, params: RatingsSRSParams<'_>) -> Result<RatingsSRSResponse, Error> {
    let endpoint = SRS_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: RatingsSRSResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_conferences_sp_ratings(api_client: &ApiClient, params: Option<RatingsSPConferencesParams<'_>>) -> Result<RatingsSPConferencesResponse, Error> {
    let endpoint = SP_CONFERENCES_ENDPOINT;
    let response = match params {
        Some(params) => {
            api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?
        },
        None => {
            api_client.get_endpoint(endpoint).await?
        }
    };
    let json_response: RatingsSPConferencesResponse = response.json().await?;
    Ok(json_response)
}

pub async fn get_elo_ratings(api_client: &ApiClient, params: Option<RatingsEloParams<'_>>) -> Result<Value, Error> {
    let endpoint = ELO_ENDPOINT;
    let response = match params {
        Some(params) => {
            api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?
        },
        None => {
            api_client.get_endpoint(endpoint).await?
        }
    };
    let json_response: Value = response.json().await?;
    Ok(json_response)
}