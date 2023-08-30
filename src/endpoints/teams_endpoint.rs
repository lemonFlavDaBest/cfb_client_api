//teams_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const TEAMS_ENDPOINT: &str = "teams";
const FBS_ENDPOINT: &str = "fbs";
const ROSTER_ENDPOINT: &str = "roster";
const TALENT_ENDPOINT: &str = "talent";
const MATCHUP_ENDPOINT: &str = "matchup";

pub struct TeamsParams<'a> {
    conference: Option<&'a str>,
}

pub struct FbsParams<'a> {
    year: Option<&'a str>,
}

pub struct RosterParams<'a> {
    team: Option<&'a str>,
    year: Option<&'a str>,
}

pub struct TalentParams<'a> {
    year: Option<&'a str>,
}

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

impl RosterParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        params
    }
}

impl TalentParams<'_> {
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
pub struct TeamsResponse {}

#[derive(Deserialize, Debug)]
pub struct FbsResponse {}

#[derive(Deserialize, Debug)]
pub struct RosterResponse {}

#[derive(Deserialize, Debug)]
pub struct TalentResponse {}

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

async fn get_roster(api_client: &ApiClient, params: Option<RosterParams<'_>>) -> Result<Vec<RosterResponse>, Error> {
    let endpoint = ROSTER_ENDPOINT;
    let roster_response: Vec<RosterResponse> = match params {
        Some(params) => {
            let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
            //println!("checkpoint");
            //print response as text
            response.json().await?
        },
        None => {
            let response = api_client.get_endpoint(endpoint).await?;
            //println!("checkpoint");
            //print response as text
            response.json().await?
        }
    };
    Ok(roster_response)
}

async fn get_talent(api_client: &ApiClient, params: Option<TalentParams<'_>>) -> Result<Vec<TalentResponse>, Error> {
    let endpoint = TALENT_ENDPOINT;
    let talent_response: Vec<TalentResponse> = match params {
        Some(params) => {
            let response = api_client.get_endpoint_with_params(endpoint, params.as_query_params()).await?;
            //println!("checkpoint");
            //print response as text
            response.json().await?
        },
        None => {
            let response = api_client.get_endpoint(endpoint).await?;
            //println!("checkpoint");
            //print response as text
            response.json().await?
        }
    };
    Ok(talent_response)
}