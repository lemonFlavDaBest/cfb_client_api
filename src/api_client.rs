use reqwest::{Client, Error, Response, Url};
use std::collections::HashMap;
use serde::Deserialize;

pub struct ApiClient {
    client: Client,
    base_url: Url,
    auth_header: String,
}

impl ApiClient {
    pub fn new(api_key: &str) -> Result<Self, Error> {
        let client = Client::new();
        let base_url = Url::parse("https://api.collegefootballdata.com")?;
        let auth_header = format!("Bearer {}", api_key);

        Ok(ApiClient {
            client,
            base_url,
            auth_header,
        })
    }

    pub async fn get(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<Response, Error> {
        let url = self.base_url.join(endpoint)?;
        let request = self.client.get(url).query(params);
        let response = request.header("Authorization", &self.auth_header).send().await?;
        Ok(response)
    }

    pub async fn get_json<T>(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<T, Error>
    where
        T: Deserialize<'static>,
    {
        let response = self.get(endpoint, params).await?;
        let json = response.json::<T>().await?;
        Ok(json)
    }
}
