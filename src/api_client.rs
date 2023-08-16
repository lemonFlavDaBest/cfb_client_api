use reqwest::{Client, ClientBuilder, Error, Response, Request, RequestBuilder };
use reqwest::header;

#[derive(Debug)]
pub struct ApiClient {
    client: Client,
    pub base_url: String,
}

//note to self. remember to use self in the function signature for methods and in the body
impl ApiClient {
    pub fn new(api_key: &str) -> Result<Self, Error> {
        let api_key = api_key.trim();
        let bearer_api_key = format!("Bearer {}", api_key);
        let mut headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_str(&bearer_api_key).expect("Bearer API key is invalid");
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()?;
        
        println!("Client: {:?}", client);

        Ok(ApiClient { client, base_url: "https://api.collegefootballdata.com/".to_string()})
    }

    pub async fn _get(&self, url: &str) -> Result<Response, Error> {
        let res = self.client.get(url).send().await?;
        Ok(res)
    }

    pub async fn _get_endpoint(&self, endpoint: &str) -> Result<Response, Error> {
        let url = format!("{}{}", self.base_url, endpoint);
        let res = self.client.get(&url).send().await?;
        Ok(res)
    }

    pub async fn get_endpoint_with_params(&self, endpoint: &str, params: Vec<(&str, &str)>) -> Result<Response, Error> {
        let url = format!("{}{}", self.base_url, endpoint);
        //println!("{:?}", self.client);
        let mut request_builder: RequestBuilder = self.client.get(&url);
        //println!("Request Builder: {:?}", request_builder);
        for (param_key, param_value) in params {
            request_builder = request_builder.query(&[(param_key, param_value)]);
        }
        //println!("Request Builder: {:?}", request_builder);
    
        let res = request_builder.send().await?;
        //println!("Response: {:?}", res);
        Ok(res)
    }

}