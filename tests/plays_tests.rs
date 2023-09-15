// plays_tests.rs

mod common;

use common::setup;

#[cfg(test)]
mod tests {
    use cfb_client_api::ApiClient;

    use super::*;

    static CLIENT: ApiClient = setup();
    
    #[test]
    fn test_get_plays() {
        let client: ApiClient = setup();
        println!("test_get_plays");
        println!("API_CLIENT: {:#?}", CLIENT);
    }
}   
