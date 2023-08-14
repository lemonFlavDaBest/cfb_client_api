// calendar_endpoint.rs

use reqwest::{Error, Response};
use serde::Deserialize;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

#[derive(Debug, Deserialize)]
pub struct CalendarResponse {
    //season: Option<u16>,
    //week: Option<u8>,
    //seasonType: Option<String>,
    //firstGameStart: Option<String>,
    //lastGameStart: Option<String>,
}

//pub async fn get_calendar(api_client: ApiClient, year: &str) -> Result<String, Error> {
    //let endpoint = "calendar";
    //let params = [("year", year)];
    //let response: Response = api_client.get_with_params(endpoint, api_client, &params).await?;
    //let response_text = response.text().await?;
    //println!("Raw JSON Response: {}", response_text);
    //let json_response: CalendarResponse = response.json().await?;
    //Ok(json_response)
    //let json_response: Result<CalendarResponse, serde_json::Error> = serde_json::from_str(&response_text);
    //if let Err(e) = json_response {
    //eprintln!("Deserialization Error: {:?}", e);
    //}
    //Ok(response_text)
    
    pub async fn get_calendar(api_client: &ApiClient, year: &str) -> Result<CalendarResponse, Error> {
        let endpoint = "calendar";
        let params = [("year", year)];
        let response = api_client.get_endpoint_with_params(endpoint, &params).await?;
        let json_response = response.json::<CalendarResponse>().await?;
        Ok(json_response)
    }