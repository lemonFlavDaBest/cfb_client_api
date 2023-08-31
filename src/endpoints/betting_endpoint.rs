// betting_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const BETTING_ENDPOINT: &str = "betting";

//define the params struct
pub struct BettingParams<'a> {
    gameId: Option<&'a str>,
    year: Option<&'a str>,
    week: Option<&'a str>,
    seasonType: Option<&'a str>,
    team: Option<&'a str>,
    conference: Option<&'a str>,
    home: Option<&'a str>,
    away: Option<&'a str>,
}