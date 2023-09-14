//endpoint_tests.rs
mod common;

use common::setup;

#[cfg(test)]
mod tests {
    use cfb_client_api::{ApiClient, calendar_endpoint::get_calendar};

    use super::*;
    static API_CLIENT: ApiClient = setup();
    
    #[test]
    fn test_get_calendar() {
        let year = "2016";
        let response = get_calendar(&API_CLIENT, year);
        assert_eq!(response.len(), 2);
    }
}   
