// scoreboard_endpoints.rs

use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc, TimeZone, NaiveDateTime};
use serde_json::{Value};
use std::str::FromStr;
use tqdm::tqdm;

const SCOREBOARD_ENDPOINT: &str = "scoreboard";

#[derive(Debug, serde::Deserialize)]
pub struct ScoreboardResponse {
    id: Option<u64>,
    startDate: Option<String>,
    startTimeTBD: Option<bool>,
    tv: Option<String>,
    neutralSite: Option<bool>,
    conferenceGame: Option<bool>,
    status: Option<String>,
    period: Option<u8>,
    clock: Option<String>,
    situation: Option<String>,
    possession: Option<String>,
    venue: Option<Venue>,
    homeTeam: Option<Team>,
    awayTeam: Option<Team>,
    weather: Option<Weather>,
    betting: Option<Betting>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Venue {
    name: Option<String>,
    city: Option<String>,
    state: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Team {
    id: Option<u64>,
    name: Option<String>,
    conference: Option<String>,
    classification: Option<String>,
    points: Option<u32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Weather {
    temperature: Option<i32>,
    description: Option<String>,
    windSpeed: Option<i32>,
    windDirection: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Betting {
    spread: Option<f64>,
    overUnder: Option<f64>,
    homeMoneyline: Option<i32>,
    awayMoneyline: Option<i32>,
}


