//venues_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const VENUES_ENDPOINT: &str = "venues";

//define the response struct
#[derive(Deserialize, Debug)]
pub struct Venue {
    id: Option<i64>,
    name: Option<String>,
    capacity: Option<i64>,
    grass: Option<bool>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country_code: Option<String>,
    location:Option<Coordinates>,
    elevation: Option<f64>,
    year_constructed: Option<i64>, 
    dome: Option<bool>,
    timezone: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Coordinates {
    x: f64, 
    y: f64
}

pub async fn get_venues(api_client: &ApiClient) -> Result<Vec<Venue>, Error> {
    let response = api_client.get_endpoint(VENUES_ENDPOINT).await?;
    let json_response: Vec<Venue> = response.json().await?;
    Ok(json_response)
}