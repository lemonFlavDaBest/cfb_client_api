//draft_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const DRAFT_ENDPOINT: &str = "draft";
const TEAMS_ENDPOINT: &str = "draft/teams";
const POSITIONS_ENDPOINT: &str = "draft/positions";
const PICKS_ENDPOINT: &str = "draft/picks";

