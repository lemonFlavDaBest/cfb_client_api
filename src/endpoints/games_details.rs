//games_details.rs
use chrono::format;
use reqwest::{Error, Response};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

use crate::api_client::ApiClient; // Import the ApiClient from the parent module

//define the endpoint
const GAMES_ENDPOINT: &str = "games";
const MEDIA_ENDPOINT: &str = "media";
const WEATHER_ENDPOINT: &str = "weather";
const PLAYERS_ENDPOINT: &str = "players";
const TEAMS_ENDPOINT: &str = "teams";
const BOX_ADVANCED_ENDPOINT: &str = "box/advanced";


pub struct MediaParams<'a> {
    pub year: &'a str,
    pub week: Option<&'a str>,
    pub seasonType: Option<&'a str>,
    pub team: Option<&'a str>,
    pub conference: Option<&'a str>,
    pub mediaType: Option<&'a str>, 
    pub classification: Option<&'a str>, 
}

#[derive(Debug, Deserialize)]
pub struct GameMedia {
    id: Option<i64>,
    season: Option<i64>,
    week: Option<u16>,
    season_type: Option<String>,
    start_time: Option<String>,
    is_start_time_tbd: Option<bool>,
    home_team: Option<String>,
    home_conference: Option<String>,
    away_team: Option<String>,
    away_conference: Option<String>,
    media_type: Option<String>,
    outlet: Option<String>,
}

impl MediaParams<'_> {
    pub fn new() -> Self {
        Default::default()
    }

    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("year", self.year)];
        // Add other fields if they exist in self
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(season_type) = self.seasonType {
            params.push(("seasonType", season_type));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(media_type) = self.mediaType {
            params.push(("mediaType", media_type));
        }
        if let Some(classification) = self.classification {
            params.push(("classification", classification));
        }  
        params
    }
}

impl Default for MediaParams<'_> {

    fn default() -> Self {
        MediaParams { 
            year: "2023", 
            week: None, 
            seasonType: Some("both"),
            team: None,
            conference: None,
            mediaType: None,
            classification: None,
        } 
    }
}

pub struct WeatherParams<'a> {
    pub gameId: Option<&'a str>,
    pub year: Option<&'a str>,
    pub week: Option<&'a str>,
    pub seasonType: Option<&'a str>,
    pub team: Option<&'a str>,
    pub conference: Option<&'a str>,
    pub classification: Option<&'a str>, 
}

impl WeatherParams<'_> {
    pub fn new() -> Self {
        Default::default()
    }

    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![];
        if let Some(year) = self.year {
            params.push(("year", year));
        }
        // Add other fields if they exist in self
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(season_type) = self.seasonType {
            params.push(("seasonType", season_type));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(game_id) = self.gameId {
            params.push(("gameId", game_id));
        }
        if let Some(classification) = self.classification {
            params.push(("classification", classification));
        }  
        params
    }
}

impl Default for WeatherParams<'_> {

    fn default() -> Self {
        WeatherParams { 
            year: Some("2023"), 
            week: None, 
            seasonType: Some("both"),
            team: None,
            conference: None,
            gameId: None,
            classification: None,
        } 
    }
}

#[derive(Debug, Deserialize)]
pub struct GameWeather {
    id: Option<i64>,
    season: Option<i64>,
    week: Option<u16>,
    season_type: Option<String>,
    start_time: Option<String>,
    game_indoors: Option<bool>,
    home_team: Option<String>,
    home_conference: Option<String>,
    away_team: Option<String>,
    away_conference: Option<String>,
    venue_id: Option<i64>,
    venue: Option<String>,
    temperature: Option<f64>,
    dew_point: Option<f64>,
    humidity: Option<f64>,
    precipitation: Option<f64>,
    snowfall: Option<f64>,
    wind_direction: Option<f64>,
    wind_speed: Option<f64>,
    pressure: Option<f64>,
    weather_condition_code: Option<i64>,
    weather_condition: Option<String>,
}

pub struct PlayersParams<'a> {
    pub year: &'a str,
    pub week: Option<&'a str>,
    pub seasonType: Option<&'a str>,
    pub team: Option<&'a str>,
    pub conference: Option<&'a str>,
    pub category: Option<&'a str>, 
    pub gameId: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
pub struct PlayerGame {
    id: Option<i64>,
    teams: Option<Vec<PlayerTeam>>,
}

// i want to eventually make these into a shared trait.
#[derive(Debug, Deserialize)]
pub struct PlayerTeam {
    school: Option<String>,
    conference: Option<String>,
    home_away: Option<String>,
    points: Option<i64>,
    categories: Option<Vec<Category>>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    category: Option<String>,
    stat: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    school: Option<String>,
    conference: Option<String>,
    home_away: Option<String>,
    points: Option<i64>,
    stats: Option<Vec<Stat>>,
}

#[derive(Debug, Deserialize)]
pub struct Stat {
    category: Option<String>,
    stat: Option<String>,
}

impl PlayersParams<'_> {
    pub fn new() -> Self {
        Default::default()
    }

    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("year", self.year)];
        // Add other fields if they exist in self
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(season_type) = self.seasonType {
            params.push(("seasonType", season_type));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(category) = self.category {
            params.push(("category", category));
        }
        if let Some(game_id) = self.gameId {
            params.push(("gameId", game_id));
        }
        params
    }

}

impl Default for PlayersParams<'_> {
    fn default() -> Self {
        PlayersParams { 
            year: "2023", 
            week: None, 
            seasonType: Some("regular"),
            team: None,
            conference: None,
            category: None,
            gameId: None,
        } 
    }
}

pub struct TeamsParams<'a> {
    year: &'a str,
    week: Option<&'a str>,
    seasonType: Option<&'a str>,
    team: Option<&'a str>,
    conference: Option<&'a str>,
    classification: Option<&'a str>,
}
#[derive(Debug, Deserialize)]
pub struct TeamGame {
    id: Option<i64>,
    teams: Option<Vec<Team>>,
}

impl TeamsParams<'_> {
    pub fn new() -> Self {
        Default::default()
    }

    fn as_query_params(&self) -> Vec<(&str, &str)> {
        let mut params = vec![("year", self.year)];
        // Add other fields if they exist in self
        if let Some(week) = self.week {
            params.push(("week", week));
        }
        if let Some(season_type) = self.seasonType {
            params.push(("seasonType", season_type));
        }
        if let Some(team) = self.team {
            params.push(("team", team));
        }
        if let Some(conference) = self.conference {
            params.push(("conference", conference));
        }
        if let Some(classification) = self.classification {
            params.push(("classification", classification));
        }  
        params
    }
}

impl Default for TeamsParams<'_> {
    fn default() -> Self {
        TeamsParams { 
            year: "2023", 
            week: None, 
            seasonType: Some("regular"),
            team: None,
            conference: None,
            classification: None,
        } 
    }
}

#[derive(Debug, Deserialize)]
pub struct BoxScore{
    teams: Option<BoxTeams>,
    players: Option<BoxPlayers>,
}

#[derive(Debug, Deserialize)]
pub struct BoxTeams {
    ppa: Option<Vec<BoxPpa>>,
    cumulative_ppa: Option<Vec<BoxCumulativePpa>>,
    success_rates: Option<Vec<BoxSuccessRates>>,
    explosiveness: Option<Vec<BoxExplosiveness>>,
    rushing: Option<Vec<BoxRushing>>,
    havoc: Option<Vec<BoxHavoc>>,
    scoring_opp: Option<Vec<BoxScoringOpp>>,
    field_position: Option<Vec<BoxFieldPosition>>,
}

#[derive(Debug, Deserialize)]
pub struct BoxPpa {}

#[derive(Debug, Deserialize)]
pub struct BoxCumulativePpa {}

#[derive(Debug, Deserialize)]
pub struct BoxSuccessRates {}

#[derive(Debug, Deserialize)]
pub struct BoxExplosiveness {}

#[derive(Debug, Deserialize)]
pub struct BoxRushing {}

#[derive(Debug, Deserialize)]
pub struct BoxHavoc {}

#[derive(Debug, Deserialize)]
pub struct BoxScoringOpp {}

#[derive(Debug, Deserialize)]
pub struct BoxFieldPosition {}

#[derive(Debug, Deserialize)]
pub struct BoxPlayers {
    usage: Option<Vec<BoxUsage>>,
    ppa: Option<Vec<BoxPlayersPpa>>,
}

#[derive(Debug, Deserialize)]
pub struct BoxUsage {}

#[derive(Debug, Deserialize)]
pub struct BoxPlayersPpa {}

pub struct BoxParams<'a> {
    gameId: &'a str,
}

pub async fn get_games_media_with_params(api_client: &ApiClient, year: &str, params: Option<MediaParams<'_>>) -> Result<Vec<GameMedia>, Error> {
    let mut games_params = params.unwrap_or_else(MediaParams::new);
    games_params.year = year;

    let endpoint = format!("{}/{}", GAMES_ENDPOINT, MEDIA_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, games_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<GameMedia> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}

pub async fn get_games_weather_with_params(api_client: &ApiClient, year: Option<&str>, params: Option<WeatherParams<'_>>) -> Result<Vec<GameWeather>, Error> {
    let mut games_params = params.unwrap_or_else(WeatherParams::new);
    games_params.year = year;

    let endpoint= format!("{}/{}", GAMES_ENDPOINT, WEATHER_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, games_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<GameWeather> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}

pub async fn get_games_players_with_params(api_client: &ApiClient, year: &str, params: Option<PlayersParams<'_>>) -> Result<Vec<PlayerGame>, Error> {
    let mut games_params = params.unwrap_or_else(PlayersParams::new);
    games_params.year = year;

    let endpoint = format!("{}/{}", GAMES_ENDPOINT, PLAYERS_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, games_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<PlayerGame> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}

pub async fn get_games_teams_with_params(api_client: &ApiClient, year: &str, params: Option<TeamsParams<'_>>) -> Result<Vec<TeamGame>, Error> {
    let mut games_params = params.unwrap_or_else(TeamsParams::new);
    games_params.year = year;

    let endpoint = format!("{}/{}", GAMES_ENDPOINT, TEAMS_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, games_params.as_query_params()).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<TeamGame> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}

pub async fn get_games_box_with_params(api_client: &ApiClient, game_id: &str) -> Result<Vec<BoxScore>, Error> {
    let games_params = vec![("gameId", game_id)];
    let endpoint = format!("{}/{}", GAMES_ENDPOINT, BOX_ADVANCED_ENDPOINT);
    let response = api_client.get_endpoint_with_params(&endpoint, games_params).await?;
    //println!("checkpoint");
    //print response as text
   
    //Ok(response.text().await?)

    //Deserialize the response into GamesResponse struct
    let json_response: Vec<BoxScore> = response.json().await?;
    //println!("json_response: {:?}", json_response);

    Ok(json_response)
}