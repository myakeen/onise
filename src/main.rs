use std::env;
use std::time::Duration;
use tokio::time::sleep;
use dotenv::dotenv;

use onise::error::KrakenResult;
use onise::KrakenClient;
use onise::ws_client::KrakenWsClient;
use onise::ws_models::WsSubscriptionPayload; // for WebSocket subscriptions

#[tokio::main]
async fn main() -> KrakenResult<()> {
    // Decide which mode to run: "rest" or "ws"
    // You can do: cargo run -- rest  OR  cargo run -- ws
    // Default to "rest" if no argument is given
    let mode = env::args().nth(1).unwrap_or_else(|| "rest".to_string());

    match mode.as_str() {
        "rest" => run_rest().await,
        "ws" => run_ws().await,
        other => {
            eprintln!("Unknown mode: {}. Usage: cargo run -- [rest|ws]", other);
            Ok(())
        }
    }
}

/// Run the Spot REST API example
async fn run_rest() -> KrakenResult<()> {
    dotenv().ok();
    let api_key = env::var("KRAKEN_API_KEY").ok();
    let api_secret = env::var("KRAKEN_API_SECRET").ok();

    // Build the REST client
    let client = KrakenClient::new(api_key, api_secret, None);

    // Call a public endpoint
    match client.get_server_time().await {
        Ok(server_time) => println!("Server time: {:?}", server_time),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Call a private endpoint (requires valid credentials)
    match client.get_balance().await {
        Ok(balance) => {
            println!("Balance: {:?}", balance.balances);
        }
        Err(e) => {
            eprintln!("Error fetching balance: {}", e);
        }
    }

    Ok(())
}

/// Run the Spot WebSocket API example
async fn run_ws() -> KrakenResult<()> {
    // Read an environment variable for the WebSocket URL, default to Kraken Spot v2
    let url = env::var("WS_URL").unwrap_or_else(|_| "wss://ws.kraken.com/v2".to_string());

    // Optionally read an auth token for private streams
    let token = env::var("KRAKEN_WS_TOKEN").ok();

    // Connect to the WebSocket
    let client = KrakenWsClient::connect(&url).await?;

    // If you have a token, authorize for private data
    if let Some(t) = token {
        client.authorize(&t, Some(1)).await?;
    }

    // Send a ping to confirm we can write messages
    client.send_ping(Some(2)).await?;

    // Subscribe to a Ticker channel if we want market data
    client
        .subscribe(
            WsSubscriptionPayload::Ticker {
                symbol: "XBT/USD".into(),
            },
            Some(3),
        )
        .await?;

    println!("Connected to {url}. Listening for WS messages...");
    // In real usage, we might run indefinitely, or until a signal
    loop {
        sleep(Duration::from_secs(10)).await;
    }
}
