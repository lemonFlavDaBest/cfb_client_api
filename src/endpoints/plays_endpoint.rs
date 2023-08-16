// plays_endpoints.rs

use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const PLAYS_ENDPOINT: &str = "plays";

//define the response struct
#[derive(Deserialize, Debug)]
pub struct PlaysResponse {
    #[serde(deserialize_with = "deserialize_u64_from_str")]
    id: Option<u64>,
    #[serde(deserialize_with = "deserialize_u32_from_str")]
    drive_id: Option<u32>,
    game_id: u32,
    drive_number: u8,
    play_number: u16,
    offense: Option<String>,
    offense_conference: Option<String>,
    offense_score: u8,
    defense: Option<String>,
    home: Option<String>,
    away: Option<String>,
    defense_conference: Option<String>,
    defense_score: u8,
    period: Option<u8>,
    clock: Clock,
    offense_timeouts: Option<u8>,
    defense_timeouts: Option<u8>,
    yard_line: Option<u16>,
    yards_to_goal: Option<u16>,
    down: Option<u8>,
    distance: Option<u16>,
    yards_gained: Option<u16>,
    scoring: Option<bool>,
    play_type: Option<String>,
    play_text: Option<String>,
    #[serde(deserialize_with = "deserialize_f64_from_str")]
    ppa: Option<f64>,
    wallclock: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Clock {
    minutes: Option<u8>,
    seconds: Option<u8>,
}
pub struct PlaysParams<'a> {
    seasonType: Option<&'a str>,
    year: &'a str,
    week: &'a str,
    team: Option<&'a str>,
    offense: Option<&'a str>,
    defense: Option<&'a str>,
    offenseConference: Option<&'a str>,
    defenseConference: Option<&'a str>,
    playType: Option<&'a str>,
    classification: Option<&'a str>,
}

impl PlaysParams<'_> {
    fn new() -> Self {
        Default::default()
    }

    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("year", self.year)];
        params.push(("week", self.week));
        // Add other fields if they exist in self
        if let Some(season_type) = self.seasonType {
            params.push(("seasonType", season_type));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(offense) = self.offense {
            params.push(("offense", offense));
        }
        if let Some(defense) = self.defense {
            params.push(("defense", defense));
        }
        if let Some(offenseConference) = self.offenseConference {
            params.push(("offenseConference", offenseConference));
        }
        if let Some(defenseConference) = self.defenseConference {
            params.push(("defenseConference", defenseConference));
        }
        if let Some(playType) = self.playType {
            params.push(("playType", playType));
        }  
        if let Some(classification) = self.classification {
            params.push(("classification", classification));
        }  
        params
    }
}

impl Default for PlaysParams<'_> {
    fn default() -> Self {
        PlaysParams { 
            seasonType: Some("regular"),
            year: "2022", 
            week: "1", 
            team: None,
            offense: None,
            defense: None,
            offenseConference: None,
            defenseConference: None,
            playType: None,
            classification: None,
        } 
    }
}

fn deserialize_u64_from_str<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;
    match value {
        Some(val_str) => {
            // Parse the string value into u64
            let parsed_val = u64::from_str(&val_str)
                .map_err(|_err| serde::de::Error::custom("Failed to parse u64 from string"))?;
            Ok(Some(parsed_val))
        }
        None => Ok(None),
    }
}

fn deserialize_u32_from_str<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;
    match value {
        Some(val_str) => {
            // Parse the string value into u32
            let parsed_val = u32::from_str(&val_str)
                .map_err(|_err| serde::de::Error::custom("Failed to parse u32 from string"))?;
            Ok(Some(parsed_val))
        }
        None => Ok(None),
    }
}

fn deserialize_f64_from_str<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<String> = Option::deserialize(deserializer)?;
    println!("lolololol");
    match value {
        Some(val_str) => {
            // Parse the string value into f64
            let parsed_val = f64::from_str(&val_str)
                .map_err(|_err| serde::de::Error::custom("Failed to parse f64 from string"))?;
            Ok(Some(parsed_val))
        }
        None => Ok(None),
    }
}

pub async fn get_plays_with_params(api_client: &ApiClient, year: &str, week: &str, params: Option<PlaysParams<'_>>) -> Result<Vec<PlaysResponse>, Error> {
    let mut plays_params = params.unwrap_or_else(PlaysParams::new);
    plays_params.year = year;
    plays_params.week = week;

    let endpoint = PLAYS_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, plays_params.as_query_params()).await?;
    println!("checkpoint");
    //print response as text
    
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<PlaysResponse> = response.json().await?;
    println!("json_response: {:?}", json_response);

    Ok(json_response)
}