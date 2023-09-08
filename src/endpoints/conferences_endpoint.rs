//conferences_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const CONFERENCES_ENDPOINT: &str = "conferences";

//define the response struct
#[derive(Deserialize, Debug)]
pub struct Conference {
    id: Option<i64>,
    name: Option<String>,
    abbreviation: Option<String>,
    class: Option<String>,
}

async fn get_conferences(api_client: &ApiClient) -> Result<Conference, Error> {
    let response = api_client.get_endpoint(CONFERENCES_ENDPOINT).await?;
    let json_response: Conference = response.json().await?;
    Ok(json_response)
}