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
pub struct VenuesResponse {}

pub async fn get_venues(api_client: &ApiClient) -> Result<VenuesResponse, Error> {
    let response = api_client.get_endpoint(VENUES_ENDPOINT).await?;
    let json_response: VenuesResponse = response.json().await?;
    Ok(json_response)
}