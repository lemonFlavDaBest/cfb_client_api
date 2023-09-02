// scoreboard_endpoints.rs

use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc, TimeZone, NaiveDateTime};
use serde_json::{Value};
use std::str::FromStr;
use tqdm::tqdm;

use crate::api_client::ApiClient; 

const SCOREBOARD_ENDPOINT: &str = "scoreboard";

#[derive(Debug, serde::Deserialize)]
pub struct ScoreboardResponse {
    id: Option<i64>,
    startDate: Option<String>,
    startTimeTBD: Option<bool>,
    tv: Option<String>,
    neutralSite: Option<bool>,
    conferenceGame: Option<bool>,
    status: Option<String>,
    period: Option<i8>,
    clock: Option<String>,
    situation: Option<String>,
    possession: Option<String>,
    venue: Option<Venue>,
    homeTeam: Option<Team>,
    awayTeam: Option<Team>,
    weather: Option<Weather>,
    betting: Option<Betting>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Venue {
    name: Option<String>,
    city: Option<String>,
    state: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Team {
    id: Option<i64>,
    name: Option<String>,
    conference: Option<String>,
    classification: Option<String>,
    points: Option<u32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Weather {
    //temperature: Option<f32>,
    //description: Option<String>,
    //windSpeed: Option<f32>,
    //windDirection: Option<f32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Betting {
    //spread: Option<f64>,
    //overUnder: Option<f64>,
    //homeMoneyline: Option<i32>,
    //awayMoneyline: Option<i32>,
}


pub struct ScoreboardParams<'a> {
    pub classification: Option<&'a str>,
    pub conference: Option<&'a str>,
}

impl ScoreboardParams<'_> {
    pub fn new() -> Self {
        Self {
            classification: None,
            conference: None,
        }
    }

    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut query_params = vec![];
        if let Some(classification) = self.classification {
            query_params.push(("classification", classification));
        } 
        if let Some(conference) = self.conference {
            query_params.push(("conference", conference));
        } 
        query_params
    }
}

pub async fn get_scoreboard(api_client: &ApiClient) -> Result<Vec<ScoreboardResponse>, Error> {

    let response = api_client.get_endpoint(SCOREBOARD_ENDPOINT,).await?;
    //println!("checkpoint");

    // Parse the response into JSON
    let json_response: Vec<ScoreboardResponse> = response.json().await?;

    //println!("JSON Response: {:?}", json_response);
    Ok(json_response)
}

pub async fn get_scoreboard_with_params(api_client: &ApiClient, params: Option<ScoreboardParams<'_>>) -> Result<Vec<ScoreboardResponse>, Error> {
    let scoreboard_params = params.unwrap_or_else(ScoreboardParams::new);
    let endpoint = SCOREBOARD_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, scoreboard_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
    
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<ScoreboardResponse> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}
