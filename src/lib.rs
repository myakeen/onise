use reqwest::Client; // For making HTTP requests
use serde::Deserialize; // For deserializing JSON

// A typed response for the getServerTime endpoint
#[derive(Debug, Deserialize)]
pub struct ServerTimeResult {
    pub unixtime: u64,
    pub rfc1123: String,
}

#[derive(Debug, Deserialize)]
struct KrakenResponse<T> {
    error: Vec<String>,
    result: T,
}

pub async fn get_server_time() -> Result<ServerTimeResult, reqwest::Error> {
    let url = "https://api.kraken.com/0/public/Time";
    let resp = Client::new()
        .get(url)
        .send()
        .await?
        .json::<KrakenResponse<ServerTimeResult>>()
        .await?;

    if resp.error.is_empty() {
        Ok(resp.result)
    } else {
        // For simplicity, just panic here. In real code, handle gracefully.
        panic!("Kraken returned error: {:?}", resp.error);
    }
}
