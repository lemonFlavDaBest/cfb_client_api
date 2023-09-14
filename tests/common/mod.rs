//bring ApiClient into scope
use cfb_client_api::ApiClient;

pub fn setup() -> ApiClient {
    let api_key = "test";
    let api_client = ApiClient::new(api_key).unwrap();
    api_client
}
