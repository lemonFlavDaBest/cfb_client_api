use reqwest::{Client, ClientBuilder, Error, Response};
use reqwest::header;

pub struct ApiClient {
    client: Client,
    pub base_url: String,
}

impl ApiClient {
    pub fn new(api_key: &str) -> Result<Self, Error> {
        let api_key = api_key.trim();
        let bearer_api_key = format!("Bearer {}", api_key);
        let mut headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_str(&bearer_api_key).expect("Bearer API key is invalid");
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        Ok(ApiClient { client, base_url: "https://api.collegefootballdata.com/".to_string()})
    }

    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        let res = self.client.get(url).send().await?;
        Ok(res)
    }
}