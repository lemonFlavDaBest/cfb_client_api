// stats_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const STATS_ENDPOINT: &str = "stats";
const SEASON_ENDPOINT: &str = "stats/season";
const SEASON_ADVANCED_ENDPOINT: &str = "stats/season/advanced";
const GAME_ADVANCED_ENDPOINT: &str = "stats/game/advanced";
const CATEGORIES_ENDPOINT: &str = "stats/categories";