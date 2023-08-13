mod api_client; 
use dotenv::dotenv;
use api_client::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = std::env::var("CFB_API_KEY").expect("CFB_API_KEY must be set.");
    let api_client = ApiClient::new(&api_key)?;

    let endpoint = "calendar";  // Adjust the endpoint according to your requirements
    let params = [("year", "2022")];  // Adjust query parameters as needed
    let url = format!("{}{}", api_client.base_url, endpoint);

    let res = api_client.get_with_params(&url, &params).await?;
    
    println!("{:#?}", res);
    Ok(())
}

