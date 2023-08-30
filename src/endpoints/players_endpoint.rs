// players_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const PLAYER_ENDPOINT: &str = "player";
const SEARCH_ENDPOINT: &str = "search";
const USAGE_ENDPOINT: &str = "usage";
const RETURNING_ENDPOINT: &str = "returning";
const STATS_ENDPOINT: &str = "stats";
const PORTAL_ENDPOINT: &str = "portal";
const SEASON_ENDPOINT: &str = "season";

