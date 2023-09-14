//endpoint_tests.rs
mod common;

use common::setup;

#[cfg(test)]
mod tests {
    use cfb_client_api::ApiClient;

    use super::*;
    static API_CLIENT: ApiClient = setup();
    
    #[test]
    fn test_get_calendar() {
        let start_year = 2016;
        let end_year = 2017;
        let response = get_calendar(&API_CLIENT, start_year, end_year).unwrap();
        assert_eq!(response.len(), 2);
    }
}   
