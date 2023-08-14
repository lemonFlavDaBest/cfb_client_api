use reqwest::{Client, ClientBuilder, Error, Response, Request, RequestBuilder};
use reqwest::header;

#[derive(Debug)]
pub struct ApiClient {
    pub client: Client,
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

    //pub async fn get_with_params(&self, endpoint: &str, client: ApiClient, _params: &[(&str, &str)]) -> Result<Response, Error> {
        
        //let url = format!("{}{}", self.base_url, endpoint);

        //let request = Request::new(reqwest::Method::GET, url.parse().unwrap());
        //let request_build: RequestBuilder = RequestBuilder::from_parts(client.client, request);
        //let res = request_build.send().await?;
        //Ok(res)
    //}
}