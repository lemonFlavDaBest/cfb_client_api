use reqwest::header;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = std::env::var("CFB_API_KEY").expect("CFB_API_TOKEN must be set.");
    let bearer_api_key = format!("Bearer {}", api_key);
    let mut headers = header::HeaderMap::new();
    let mut auth_value = header::HeaderValue::from_str(&bearer_api_key).expect("Bearer API key is invalid");
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client.get("api.collegefootballdata.com/").send().await?;


    println!("{:#?}", res);
    Ok(())
}

