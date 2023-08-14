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
    
    //need to evewntuallu change to CalendarResponse as the result
    pub async fn get_calendar(api_client: &ApiClient, year: &str) -> Result<Vec<CalendarResponse>, Error> {
        let endpoint = "calendar";
        let params = [("year", year)];
        println!("Params: {:?}", params);
        let response = api_client.get_endpoint_with_params(endpoint, &params).await?;
        println!("checkpoint");
    
        // Parse the response into JSON
        let json_response: Vec<CalendarResponse> = response.json().await?;
    
        println!("JSON Response: {:?}", json_response);
        Ok(json_response)
    }