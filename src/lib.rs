//lib.rs

//modules
mod api_client;
mod endpoints;

//exports
pub use api_client::ApiClient;
pub use endpoints::calendar_endpoint;