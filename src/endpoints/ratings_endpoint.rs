// ratings_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const RATINGS_ENDPOINT: &str = "ratings";
const SP_ENDPOINT: &str = "ratings/sp";
const SP_CONFERENCES_ENDPOINT: &str = "ratings/sp/conferences";
const ELO_ENDPOINT: &str = "ratings/elo";