//coaches_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const COACHES_ENDPOINT: &str = "coaches";

//define the params struct
pub struct CoachesParams<'a> {
    firstName: Option<&'a str>,
    lastName: Option<&'a str>,
    team: Option<&'a str>,
    year: Option<&'a str>,
    minYear: Option<&'a str>, // inclusive
    maxYear: Option<&'a str>, // inclusive
}

impl CoachesParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let Some(firstName) = self.firstName {
            params.push(("firstName", firstName));
        }
        if let Some(lastName) = self.lastName {
            params.push(("lastName", lastName));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        if let Some(minYear) = self.minYear {
            params.push(("minYear", minYear));
        }
        if let Some(maxYear) = self.maxYear {
            params.push(("maxYear", maxYear));
        }
        params
    }
}

#[derive(Deserialize, Debug)]
pub struct CoachesResponse {}

async fn get_coaches_with_params(api_client: &ApiClient, params: Option<CoachesParams<'_>>) -> Result<CoachesResponse, Error> {
    let endpoint = COACHES_ENDPOINT;
    let response = match params {
        Some(params) => api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?,
        None => api_client.get_endpoint(COACHES_ENDPOINT).await?,
    };
    let json_response: CoachesResponse = response.json().await?;
    Ok(json_response)
}