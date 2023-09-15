// plays_tests.rs

mod common;

use common::setup;


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_plays() {
        let client = setup();
        println!("test_get_plays");
        println!("API_CLIENT: {:#?}", client);
    }
}   
