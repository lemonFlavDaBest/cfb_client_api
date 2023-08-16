// calendar_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

#[derive(Debug, Deserialize)]
pub struct CalendarResponse {
    pub season: String,
    pub week: i32,
    pub seasonType: String,
    #[serde(deserialize_with = "deserialize_game_start")]
    pub firstGameStart: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_game_start")]
    pub lastGameStart: Option<DateTime<Utc>>,
}


fn deserialize_game_start<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<Value> = Option::deserialize(deserializer)?;
    match value {
        Some(val) => {
            let wallclock_str = val
                .as_str()
                .ok_or_else(|| serde::de::Error::custom("Invalid wallclock format"))?;

            // Define a custom date and time format
            let custom_format = "%Y-%m-%dT%H:%M:%S%.3fZ";

            // Parse the wallclock string using the custom format
            let parsed_datetime = Utc
                .datetime_from_str(wallclock_str, custom_format)
                .map_err(|_err| serde::de::Error::custom("Failed to parse wallclock datetime"))?;

            Ok(Some(parsed_datetime))
        }
        None => Ok(None),
    }
}

//define the calendar endpoint
const CALENDAR_ENDPOINT: &str = "calendar";

    pub async fn get_calendar(api_client: &ApiClient, year: &str) -> Result<Vec<CalendarResponse>, Error> {
        let params: Vec<(&str, &str)> = vec![("year", year)];
        //println!("Params: {:?}", params);
        
        let response = api_client.get_endpoint_with_params(CALENDAR_ENDPOINT, params).await?;
        //println!("checkpoint");
    
        // Parse the response into JSON
        let json_response: Vec<CalendarResponse> = response.json().await?;
    
        //println!("JSON Response: {:?}", json_response);
        Ok(json_response)
    }