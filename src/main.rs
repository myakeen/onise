use onise::KrakenClient;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("KRAKEN_API_KEY").ok();
    let api_secret = env::var("KRAKEN_API_SECRET").ok();

    // Build the client
    let client = KrakenClient::new(api_key, api_secret, None);

    // Call a public endpoint
    match client.get_server_time().await {
        Ok(server_time) => println!("Server time: {:?}", server_time),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Call a private endpoint (requires valid credentials)
    match client.get_balance().await {
        Ok(balance) => println!("Balance: {:?}", balance.balances),
        Err(e) => eprintln!("Error fetching balance: {}", e),
    }
}

// use std::env;
// use std::time::Duration;
// use tokio::time::sleep;

// use onise::error::KrakenResult;
// use onise::ws_client::KrakenWsClient; // or your own error module

// #[tokio::main]
// async fn main() -> KrakenResult<()> {
//     // Read an environment variable for the WebSocket URL, default to Kraken Spot v2
//     let url = env::var("WS_URL").unwrap_or_else(|_| "wss://ws.kraken.com/v2".to_string());

//     // Optionally read an auth token for private streams
//     let token = env::var("KRAKEN_WS_TOKEN").ok();

//     // Connect to the WebSocket
//     let client = KrakenWsClient::connect(&url).await?;

//     // If you have a token, store it in the client or call `authorize(...)`
//     if let Some(t) = token {
//         client.authorize(&t, Some(1)).await?;
//         // Optionally also store in client if you want:
//         // client.token = Some(t);
//     }

//     // Send a ping to confirm we can write messages
//     client.send_ping(Some(2)).await?;

//     // Subscribe to a Ticker channel if we want market data
//     client
//         .subscribe(
//             onise::ws_models::WsSubscriptionPayload::Ticker {
//                 symbol: "XBT/USD".into(),
//             },
//             Some(3),
//         )
//         .await?;

//     // Let the process run so we can see inbound messages
//     // In a production service, you might catch signals or run until cancelled.
//     // For demonstration, we'll just loop every 10 seconds.
//     println!("Connected to {url}. Listening for messages...");
//     loop {
//         sleep(Duration::from_secs(10)).await;
//         // You could do more requests here, or just keep it running
//     }
// }
