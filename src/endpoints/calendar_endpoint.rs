// calendar_endpoint.rs

use reqwest::{Error, Response};
use serde::Deserialize;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

#[derive(Debug, Deserialize)]
pub struct CalendarResponse {
    pub season: String,
    pub week: i32,
    pub seasonType: String,
    pub firstGameStart: String,
    pub lastGameStart: String,
}

//define the calendar endpoint
const CALENDAR_ENDPOINT: &str = "calendar";

    pub async fn get_calendar(api_client: &ApiClient, year: &str) -> Result<Vec<CalendarResponse>, Error> {
        let params: Vec<(&str, &str)> = vec![("year", year)];
        println!("Params: {:?}", params);
        
        let response = api_client.get_endpoint_with_params(CALENDAR_ENDPOINT, params).await?;
        println!("checkpoint");
    
        // Parse the response into JSON
        let json_response: Vec<CalendarResponse> = response.json().await?;
    
        println!("JSON Response: {:?}", json_response);
        Ok(json_response)
    }