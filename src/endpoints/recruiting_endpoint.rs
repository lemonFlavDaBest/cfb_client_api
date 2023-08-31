// recruiting_endpoint.rs

use chrono::{DateTime, Utc, TimeZone};
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const RECRUITING_ENDPOINT: &str = "recruiting";
const PLAYERS_ENDPOINT: &str = "players";
const TEAMS_ENDPOINT: &str = "teams";
const GROUPS_ENDPOINT: &str = "groups";

//define the params struct
pub struct RecruitingPlayersParams<'a> {
    year: Option<&'a str>, //required if no team
    classification: Option<&'a str>, // HighSchool, JUCO, PrepSchool
    postition: Option<&'a str>,
    state: Option<&'a str>,
    team: Option<&'a str>, //required if no year
}

#[derive(Deserialize, Debug)]
pub struct RecruitingPlayersResponse {}