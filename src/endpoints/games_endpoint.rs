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

pub struct GamesParams {

}

//create function to get games that take the api client as a parameter and an optional parameters struct
pub async fn get_games_with_params(api_client: &ApiClient, params: Option<GamesParams>) -> Result<Response, Error> {
    //create the url
    

    //return the response
    Ok(response)
}