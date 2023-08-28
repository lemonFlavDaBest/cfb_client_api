//drives_endpoint.rs
use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const DRIVES_ENDPOINT: &str = "drives";

//define the response struct
#[derive(Deserialize, Debug)]
pub struct DrivesResponse {

}

//define the params struct
pub struct DrivesParams<'a> {
    year: &'a str,
    team: Option<&'a str>,
    week: Option<&'a str>,
    offense: Option<&'a str>,
    defense: Option<&'a str>,
    conference: Option<&'a str>,
    offenseConference: Option<&'a str>,
    defenseConference: Option<&'a str>,
    classification: Option<&'a str>,
}

impl DrivesParams<'_> {
    pub fn new() -> Self {
        Default::default()
    }

    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let year = self.year {
            params.push(("year", year));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(offense) = self.offense {
            params.push(("offense", offense));
        }
        if let Some(defense) = self.defense {
            params.push(("defense", defense));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(offenseConference) = self.offenseConference {
            params.push(("offenseConference", offenseConference));
        }
        if let Some(defenseConference) = self.defenseConference {
            params.push(("defenseConference", defenseConference));
        }
        if let Some(classification) = self.classification {
            params.push(("classification", classification));
        }
        params
    }
}

impl Default for DrivesParams<'_> {
    fn default() -> Self {
        DrivesParams { 
            year: "2023", 
            team: None,
            week: None,
            offense: None,
            defense: None,
            conference: None,
            offenseConference: None,
            defenseConference: None,
            classification: None,
        }
    }
}

pub async fn get_drives_with_params(api_client: &ApiClient, params: Option<DrivesParams<'_>>) -> Result<Vec<DrivesResponse>, Error> {
    let drives_params = params.unwrap_or_else(DrivesParams::new);
    let endpoint = DRIVES_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, drives_params.as_query_params()).await?;
    //println!("checkpoint");
    
    // Parse the response into JSON
    let json_response: Vec<DrivesResponse> = response.json().await?;
    
    //println!("JSON Response: {:?}", json_response);
    Ok(json_response)
}