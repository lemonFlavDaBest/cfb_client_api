//endpoint_tests.rs
mod common;

use common::setup;



#[cfg(test)]
mod tests {

    use cfb_client_api::{ApiClient, calendar_endpoint::get_calendar, };

    use super::*;

    
    #[test]
    fn test_get_calendar() {
        let client = setup();
        let year = "2016";
        let response = get_calendar(&client, year);
        println!("test_get_calendar");
    }

    #[test]
    fn test_get_calendar_year_range() {
        let client = setup();
        let start_year = 2016;
        let end_year = 2017;
        //let response = get_calendar_year_range(&CLIENT, start_year, end_year);
        println!("test_get_calendar_year_range");
    }
}   
