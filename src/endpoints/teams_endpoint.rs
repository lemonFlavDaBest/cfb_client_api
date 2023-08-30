//teams_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const TEAMS_ENDPOINT: &str = "teams";
const FBS_ENDPOINT: &str = "fbs";

pub struct TeamsParams<'a> {
    conference: Option<&'a str>,
}

pub struct FbsParams<'a> {
    year: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
pub struct TeamsResponse {}

impl TeamsParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        params
    }
}

impl FbsParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        params
    }
}

#[derive(Deserialize, Debug)]
pub struct FbsResponse {}

pub async fn get_teams(api_client: &ApiClient, params: Option<TeamsParams<'_>>) -> Result<Vec<TeamsResponse>, Error> {
    // I want to match params with some and none. if some, then unwrap and call get_endpoint_with_parms.
    //if none, then just call get_endpoint
    let teams_response: Vec<TeamsResponse> = match params {
        Some(params) => {
            let response = api_client.get_endpoint_with_params(TEAMS_ENDPOINT, params.as_query_params()).await?;
            //println!("checkpoint");
            //print response as text
            response.json().await?
        },
        None => {
            let endpoint = TEAMS_ENDPOINT;
            let response = api_client.get_endpoint(endpoint).await?;
            //println!("checkpoint");
            //print response as text
            response.json().await?
        }
    };
    Ok(teams_response)
}

pub async fn get_fbs_teams(api_client: &ApiClient, params: Option<FbsParams<'_>>) -> Result<Vec<FbsResponse>, Error> {
    let endpoint = format!("{}/{}", TEAMS_ENDPOINT, FBS_ENDPOINT);
    let fbs_response: Vec<FbsResponse> = match params {
        Some(params) => {
            let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
            //println!("checkpoint");
            //print response as text
            response.json().await?
        },
        None => {
            let endpoint = FBS_ENDPOINT;
            let response = api_client.get_endpoint(&endpoint).await?;
            //println!("checkpoint");
            //print response as text
            response.json().await?
        }
    };
    Ok(fbs_response)
}