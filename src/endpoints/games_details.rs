//games_details.rs
use chrono::format;
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const GAMES_ENDPOINT: &str = "games";
const MEDIA_ENDPOINT: &str = "media";
const WEATHER_ENDPOINT: &str = "weather";
const PLAYERS_ENDPOINT: &str = "players";
const TEAMS_ENDPOINT: &str = "teams";


pub struct MediaParams<'a> {
    pub year: &'a str,
    pub week: Option<&'a str>,
    pub seasonType: Option<&'a str>,
    pub team: Option<&'a str>,
    pub conference: Option<&'a str>,
    pub mediaType: Option<&'a str>, 
    pub classification: Option<&'a str>, 
}

pub struct MediaResponse {

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
            seasonType: Some("both"),
            team: None,
            conference: None,
            mediaType: None,
            classification: None,
        } 
    }
}

pub struct WeatherParams<'a> {
    pub gameId: Option<&'a str>,
    pub year: Option<&'a str>,
    pub week: Option<&'a str>,
    pub seasonType: Option<&'a str>,
    pub team: Option<&'a str>,
    pub conference: Option<&'a str>,
    pub classification: Option<&'a str>, 
}

impl WeatherParams<'_> {
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
        if let Some(game_id) = self.gameId {
            params.push(("gameId", game_id));
        }
        if let Some(classification) = self.classification {
            params.push(("classification", classification));
        }  
        params
    }
}

impl Default for WeatherParams<'_> {

    fn default() -> Self {
        WeatherParams { 
            year: "2023", 
            week: None, 
            seasonType: Some("both"),
            team: None,
            conference: None,
            gameId: None,
            classification: None,
        } 
    }
}

pub struct WeatherResponse {}

pub struct PlayersParams<'a> {
    pub year: &'a str,
    pub week: Option<&'a str>,
    pub seasonType: Option<&'a str>,
    pub team: Option<&'a str>,
    pub conference: Option<&'a str>,
    pub category: Option<&'a str>, 
    pub gameId: Option<&'a str>,
}

pub struct PlayersResponse {}

impl PlayersParams<'_> {
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
        if let Some(category) = self.category {
            params.push(("category", category));
        }
        if let Some(game_id) = self.gameId {
            params.push(("gameId", game_id));
        }
        params
    }

}

impl Default for PlayersParams<'_> {
    fn default() -> Self {
        PlayersParams { 
            year: "2023", 
            week: None, 
            seasonType: Some("regular"),
            team: None,
            conference: None,
            category: None,
            gameId: None,
        } 
    }
}

pub struct TeamsParams<'a> {
    year: &'a str,
    week: Option<&'a str>,
    seasonType: Option<&'a str>,
    team: Option<&'a str>,
    conference: Option<&'a str>,
    classification: Option<&'a str>,
}
pub struct TeamsResponse {}
impl TeamsParams {
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
        if let Some(classification) = self.classification {
            params.push(("classification", classification));
        }  
        params
    }
}
impl Default for TeamsParams {
    fn default() -> Self {
        TeamsParams { 
            year: "2023", 
            week: None, 
            seasonType: Some("regular"),
            team: None,
            conference: None,
            classification: None,
        } 
    }
}

pub async fn get_games_media_with_params(api_client: &ApiClient, year: &str, params: Option<MediaParams<'_>>) -> Result<Vec<MediaResponse>, Error> {
    let mut games_params = params.unwrap_or_else(MediaParams::new);
    games_params.year = year;

    let endpoint = format!("{}/{}", GAMES_ENDPOINT, MEDIA_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, games_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<MediaResponsee> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}

pub async fn get_games_weather_with_params(api_client: &ApiClient, year: &str, params: Option<WeatherParams<'_>>) -> Result<Vec<WeatherResponse>, Error> {
    let mut games_params = params.unwrap_or_else(WeatherParams::new);
    games_params.year = year;

    let endpoint = format!("{}/{}", GAMES_ENDPOINT, WEATHER_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, games_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<WeatherResponse> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}

pub async fn get_games_teams_with_params(api_client: &ApiClient, year: &str, params: Option<TeamsParams<'_>>) -> Result<Vec<TeamsResponse>, Error> {
    let mut games_params = params.unwrap_or_else(TeamsParams::new);
    games_params.year = year;

    let endpoint = format!("{}/{}", GAMES_ENDPOINT, TEAMS_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, games_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<TeamsResponse> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}