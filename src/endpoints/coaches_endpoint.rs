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
pub struct Coach {
    first_name: Option<String>,
    last_name: Option<String>,
    hire_date: Option<String>,
    seasons: Option<Vec<Season>>,
}

#[derive(Deserialize, Debug)]
pub struct Season {
    school: Option<String>,
    year: Option<String>,
    games: Option<i16>,
    wins: Option<i16>,
    losses: Option<i16>,
    ties: Option<i16>,
    preseason_rank: Option<i16>,
    postseason_rank: Option<i16>,
    srs: Option<f32>,
    sp_overall: Option<f32>,
    sp_offense: Option<f32>,
    sp_defense: Option<f32>,
}

async fn get_coaches_with_params(api_client: &ApiClient, params: Option<CoachesParams<'_>>) -> Result<Coach, Error> {
    let endpoint = COACHES_ENDPOINT;
    let response = match params {
        Some(params) => api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?,
        None => api_client.get_endpoint(COACHES_ENDPOINT).await?,
    };
    let json_response: Coach = response.json().await?;
    Ok(json_response)
}