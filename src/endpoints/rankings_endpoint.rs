// rankings_endpoint.rs

use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc, TimeZone, NaiveDateTime};
use serde_json::{Value};
use std::str::FromStr;
use tqdm::tqdm;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module\

//define the endpoint
const RANKINGS_ENDPOINT: &str = "rankings";

//define the params struct
pub struct RankingsParams<'a> {
    year: &'a str,
    week: Option<&'a str>,
    seasonType: Option<&'a str>, //like regular or postseason
}

impl RankingsParams <'_> {
    pub fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        params.push(("year", self.year));
        // Add other fields if they exist in self
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(seasonType) = self.seasonType {
            params.push(("seasonType", seasonType));
        }
        params
    }
}
#[derive(Deserialize, Debug)]
pub struct RankingsResponse {}

async fn get_rankings_with_params(api_client: &ApiClient, params: RankingsParams<'_>) -> Result<RankingsResponse, Error> {
    let endpoint = RANKINGS_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
    let json_response: RankingsResponse = response.json().await?;
    Ok(json_response)
}