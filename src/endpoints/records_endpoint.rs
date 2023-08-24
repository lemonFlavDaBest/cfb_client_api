// records_endpoint.rs

use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

const RECORDS_ENDPOINT: &str = "records";

pub struct RecordsParams<'a> {
    year: Option<&'a str>,
    team: Option<&'a str>,
    conference: Option<&'a str>,
}

impl RecordsParams<'_> {
    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::new();
        // Add other fields if they exist in self
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        params
    }
}

//create function to get records that take the api client as a parameter and an optional recordsparams struct
pub async fn get_records_with_params(api_client: &ApiClient, params: Option<RecordsParams<'_>>) -> Result<Vec<GamesResponse>, Error> {
    let mut records_params = params.unwrap_or_else(|| RecordsParams { year: None, team: None, conference: None });
    let endpoint = RECORDS_ENDPOINT;
    let response = api_client.get_endpoint_with_params(endpoint, records_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<GamesResponse> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}