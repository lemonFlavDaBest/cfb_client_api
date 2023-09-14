//bring ApiClient into scope
use cfb_client_api::ApiClient;

pub fn setup(api_key: &str) -> ApiClient {
    let api_client = ApiClient::new(api_key).unwrap();
    api_client
}
