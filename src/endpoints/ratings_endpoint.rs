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
#[derive(Debug, Deserialize)]
pub struct RatingsSPResponse {}

#[derive(Debug, Deserialize)]
pub struct RatingsSRSResponse {}

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