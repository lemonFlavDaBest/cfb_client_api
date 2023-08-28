//plays_details.rs


use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc, TimeZone, NaiveDateTime};
use serde_json::{Value};
use std::str::FromStr;
use tqdm::tqdm;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const PLAYS_ENDPOINT: &str = "plays";
const LIVE_ENDPOINT: &str = "live";
const TYPES_ENDPOINT: &str = "types";
const STATS_ENDPOINT: &str = "stats";
const STAT_ENDPOINT: &str = "stat";