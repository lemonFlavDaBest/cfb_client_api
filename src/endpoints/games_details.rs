//games_details.rs
use chrono::format;
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const GAMES_ENDPOINT: &str = "games";
const MEDIA_ENDPOINT: &str = "media";


pub struct MediaParams<'a> {
    year: &'a str,
    week: Option<&'a str>,
    pub seasonType: Option<&'a str>,
    team: Option<&'a str>,
    pub conference: Option<&'a str>,
    pub mediaType: Option<&'a str>, 
    pub classification: Option<&'a str>, 
}

impl MediaParams<'_> {
    pub fn new() -> Self {
        Default::default()
    }

    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("year", self.year)];
        // Add other fields if they exist in self
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(season_type) = self.seasonType {
            params.push(("seasonType", season_type));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(media_type) = self.mediaType {
            params.push(("mediaType", media_type));
        }
        if let Some(classification) = self.classification {
            params.push(("classification", classification));
        }  
        params
    }
}

impl Default for MediaParams<'_> {
    fn default() -> Self {
        MediaParams { 
            year: "2023", 
            week: None, 
            seasonType: Some("regular"),
            team: None,
            conference: None,
            mediaType: None,
            classification: None,
        } 
    }
}


pub async fn get_games_media_with_params(api_client: &ApiClient, year: &str, params: Option<GamesParams<'_>>) -> Result<Vec<GamesResponse>, Error> {
    let mut games_params = params.unwrap_or_else(GamesParams::new);
    games_params.year = year;

    let endpoint = format!("{}/{}", GAMES_ENDPOINT, MEDIA_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, games_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<GamesResponse> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}