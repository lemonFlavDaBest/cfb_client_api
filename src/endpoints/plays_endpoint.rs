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

pub async fn get_plays_with_params(api_client: &ApiClient, year: &str, week: &str, params: Option<PlaysParams<'_>>) -> Result<String, Error> {
    let mut plays_params = params.unwrap_or_else(PlaysParams::new);
    plays_params.year = year;
    plays_params.week = week;

    let endpoint = PLAYS_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, plays_params.as_query_params()).await?;
    println!("checkpoint");
    //print response as text
    
    Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    //let json_response: Vec<GamesResponse> = response.json().await?;
   //println!("json_response: {:?}", json_response);

    //Ok(json_response)
}