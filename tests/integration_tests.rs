use onise::KrakenClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use std::env;
use tokio;

#[tokio::test]
async fn test_get_server_time_mock() {
    // Start a local mock server
    let mock_server = MockServer::start().await;

    // Prepare a mock response for /0/public/Time
    let mock_body = r#"{
      "error": [],
      "result": {
        "unixtime": 1672531199,
        "rfc1123": "Mon, 01 Jan 2023 00:59:59 GMT"
      }
    }"#;

    Mock::given(method("GET"))
        .and(path("/0/public/Time"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(mock_body, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a client pointing to the mock server
    let client = KrakenClient::new(
        None,
        None,
        Some(mock_server.uri()), // base_url override
    );

    // Call our method
    let resp = client.get_server_time().await.expect("Should succeed");
    assert_eq!(resp.unixtime, 1672531199);
    assert_eq!(resp.rfc1123, "Mon, 01 Jan 2023 00:59:59 GMT");
}

#[tokio::test]
async fn test_get_server_time_live() {
    // Only run this if we have a real environment variable set, e.g. "ENABLE_LIVE_TESTS=1"
    if env::var("ENABLE_LIVE_TESTS").unwrap_or_default() != "1" {
        eprintln!("Skipping live test_get_server_time_live because ENABLE_LIVE_TESTS != 1");
        return;
    }

    // We assume no API key/secret needed for public endpoint:
    let client = KrakenClient::new(
        None, None, None, // default base URL => https://api.kraken.com
    );

    let resp = client.get_server_time().await.expect("Live call failed");
    println!("Live server time response: {:?}", resp);
    assert!(resp.unixtime > 0);
}
