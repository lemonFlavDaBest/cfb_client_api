//calendar_tests.rs
mod common;

use common::setup;

#[cfg(test)]
mod tests {

    use cfb_client_api::{calendar_endpoint::get_calendar, ApiClient};

    use super::*;

    
    #[tokio::test]
    async fn test_get_calendar() {
        let client = setup();
        let year = "2016";
        let response = get_calendar(&client, year).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_calendar_year_range() {
        let client = setup();
        let start_year = 2016;
        let end_year = 2017;
        //let response = get_calendar_year_range(&CLIENT, start_year, end_year);
        println!("test_get_calendar_year_range");
    }
}   
