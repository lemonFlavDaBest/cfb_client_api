// ratings_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const RATINGS_ENDPOINT: &str = "ratings";
const SP_ENDPOINT: &str = "ratings/sp";
const SP_CONFERENCES_ENDPOINT: &str = "ratings/sp/conferences";
const ELO_ENDPOINT: &str = "ratings/elo";

//define the params struct
pub struct RatingsSPParams<'a> {
    year: Option<&'a str>,
    team: Option<&'a str>,
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

pub struct RatingsSPResponse {}

pub async fn get_ratings_sp(api_client: &ApiClient, params: RatingsSPParams<'_>) -> Result<RatingsSPResponse, Error> {
    let endpoint = SP_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: RatingsSPResponse = response.json().await?;
    Ok(json_response)
}