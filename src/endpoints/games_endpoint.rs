// games_endpoint.rs

use reqwest::{Error, Response};
use serde::Deserialize;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const GAMES_ENDPOINT: &str = "games";

//define the response struct
#[derive(Deserialize, Debug)]
pub struct GamesResponse {

}

#[derive(Debug)]
pub struct GamesParams<'a> {
    year: &'a str,
    week: Option<&'a str>,
    seasonType: Option<&'a str>,
    team: Option<&'a str>,
    home: Option<&'a str>,
    away: Option<&'a str>,
    conference: Option<&'a str>,
    division: Option<&'a str>, //Division classification filter (fbs/fcs/ii/iii)
    id: Option<&'a str>, // id filter for querying a single game
}

impl GamesParams<'_> {
    fn new(year: &str) -> Self {
        Default::default()
    }
}

impl Default for GamesParams<'_> {
    fn default() -> Self {
        GamesParams { 
            year: "2023", 
            week: None, 
            seasonType: Some("regular"),
            team: None,
            home: None,
            away: None,
            conference: None,
            division: None,
            id: None,
        } 
    }
}

//create function to get games that take the api client as a parameter and an optional parameters struct
pub async fn get_games_with_params(api_client: &ApiClient, params: Option<GamesParams>) -> Result<GamesResponseResponse, Error> {
}