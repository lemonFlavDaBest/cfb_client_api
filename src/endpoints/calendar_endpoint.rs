// calendar_endpoint.rs

use reqwest::{Error, Response};
use serde::Deserialize;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

#[derive(Debug, Deserialize)]

pub struct CalendarResponse {
    // Define the structure to match the JSON response
    // For example: field_name: field_type,
}

pub async fn get_calendar(api_client: &ApiClient, year: &str) -> Result<CalendarResponse, Error> {
    let endpoint = "calendar";
    let params = [("year", year)];
    let url = format!("{}{}", api_client.base_url, endpoint);

    let response = api_client.get_with_params(&url, &params).await?;
    let json_response: CalendarResponse = response.json().await?;

    Ok(json_response)
}