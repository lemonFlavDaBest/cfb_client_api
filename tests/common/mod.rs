use cfb_api::api_client::ApiClient;

pub fn setup(api_key: &str) -> ApiClient {
    let api_client = ApiClient::new(api_key).unwrap();
}
