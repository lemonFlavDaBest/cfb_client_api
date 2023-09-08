//teams_endpoint.rs

use chrono::{DateTime, Utc, TimeZone, format};
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

pub struct MatchupParams<'a> {
    team1: &'a str,
    team2: &'a str,
    minYear: Option<&'a str>,
    maxYear: Option<&'a str>,
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

impl MatchupParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        params.push(("team1", self.team1));
        params.push(("team2", self.team2));
        // Add other fields if they exist in self
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
pub struct Team {
    id: Option<i64>,
    school: Option<String>,
    mascot: Option<String>,
    abbreviation: Option<String>,
    alt_name_1: Option<String>,
    alt_name_2: Option<String>,
    alt_name_3: Option<String>,
    classification: Option<String>,
    conference: Option<String>,
    division: Option<String>,
    color: Option<String>,
    alt_color: Option<String>,
    logos: Option<Vec<String>>,
    twitter: Option<String>,
    location: Option<Location>,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    venue_id: Option<i64>,
    name: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country_code: Option<String>,
    timezone: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    elevation: Option<f64>,
    capacity: Option<u64>,
    year_constructed: Option<u64>,
    grass: Option<bool>,
    dome: Option<bool>,
}


#[derive(Deserialize, Debug)]
pub struct Player {
    id: Option<i64>,
    first_name: Option<String>,
    last_name: Option<String>,
    team: Option<String>,
    height: Option<u16>,
    weight: Option<u16>,
    jersey: Option<u8>,
    year: Option<u8>,
    position: Option<String>,
    home_city: Option<String>,
    home_state: Option<String>,
    home_country: Option<String>,
    home_latitude: Option<f64>,
    home_longitude: Option<f64>,
    home_county_fips: Option<String>,
    recruit_ids: Option<Vec<i64>>,
}

#[derive(Deserialize, Debug)]
pub struct TeamTalent {
    year: Option<u64>,
    school: Option<String>,
    talent: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct MatchupResponse {}

pub async fn get_teams(api_client: &ApiClient, params: Option<TeamsParams<'_>>) -> Result<Vec<Team>, Error> {
    // I want to match params with some and none. if some, then unwrap and call get_endpoint_with_parms.
    //if none, then just call get_endpoint
    let teams_response: Vec<Team> = match params {
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

pub async fn get_fbs_teams(api_client: &ApiClient, params: Option<FbsParams<'_>>) -> Result<Vec<Team>, Error> {
    let endpoint = format!("{}/{}", TEAMS_ENDPOINT, FBS_ENDPOINT);
    let fbs_response: Vec<Team> = match params {
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

async fn get_roster(api_client: &ApiClient, params: Option<RosterParams<'_>>) -> Result<Vec<Player>, Error> {
    let endpoint = ROSTER_ENDPOINT;
    let roster_response: Vec<Player> = match params {
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

async fn get_talent(api_client: &ApiClient, params: Option<TalentParams<'_>>) -> Result<Vec<TeamTalent>, Error> {
    let endpoint = TALENT_ENDPOINT;
    let talent_response: Vec<TeamTalent> = match params {
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

async fn get_matchups(api_client: &ApiClient, params: MatchupParams<'_>) -> Result<Vec<MatchupResponse>, Error> {
    let endpoint = format!("{}/{}", TEAMS_ENDPOINT, MATCHUP_ENDPOINT);
    let matchup_response: Vec<MatchupResponse> = {
        let response = api_client.get_endpoint_with_params(&endpoint, params.as_query_params()).await?;
        //println!("checkpoint");
        //print response as text
        response.json().await?
    };
    Ok(matchup_response)
}