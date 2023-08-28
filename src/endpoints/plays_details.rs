//plays_details.rs


use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc, TimeZone, NaiveDateTime};
use serde_json::{Value};
use std::str::FromStr;
use tqdm::tqdm;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const PLAYS_ENDPOINT: &str = "plays";
const LIVE_ENDPOINT: &str = "live";
const TYPES_ENDPOINT: &str = "types";
const STATS_ENDPOINT: &str = "stats";
const STAT_ENDPOINT: &str = "stat";

pub struct LivePlayParams {
    gameId: &'a str,
}

pub struct LivePlayResponse{}

pub async fn get_live_plays(api_client: &ApiClient, game_id: &str) -> Result<Vec<LivePlayResponse>, Error> {
    let params: Vec<(&str, &str)> = vec![("gameId", game_id)];
    //println!("Params: {:?}", params);
    
    let response = api_client.get_endpoint_with_params(LIVE_ENDPOINT, params).await?;
    //println!("checkpoint");
    
    // Parse the response into JSON
    let json_response: Vec<LivePlayResponse> = response.json().await?;
    
    //println!("JSON Response: {:?}", json_response);
    Ok(json_response)
}