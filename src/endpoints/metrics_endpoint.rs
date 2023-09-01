// metric_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const PPA_PREDICTED_ENDPOINT: &str = "ppa/predicted";
const PPA_TEAMS_ENDPOINT: &str = "ppa/teams";
const PPA_GAMES_ENDPOINT: &str = "ppa/games";
