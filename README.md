# Onise, Kraken Client for Rust

A **comprehensive, typed, rate-limited, testable** Rust client for:

1. **Kraken's Spot REST API** (all public and private endpoints).
2. **Kraken's Spot WebSocket API v2** (market data, user data, and user trading).

This library provides:

- **Strongly typed** request/response models for Kraken's endpoints.
- **Advanced error handling** that parses Kraken's known error codes.
- **Configurable rate limiting** (token-bucket or others) to avoid hitting limits.
- **Integration tests** (mocked and local) for both REST and WebSocket.
- **WebSocket support** with a split read/write approach, typed subscription/unsubscription, user trading, etc.

> **Disclaimer**  
> This is **not** an official Kraken product. Always consult the [official docs](https://docs.kraken.com/rest/) and [WebSocket v2 docs](https://docs.kraken.com/websockets-v2/) for the latest changes or usage policies.

## Features

- **Complete REST coverage**: All documented endpoints (public & private).
- **Spot WebSocket API v2** coverage: Market Data (Ticker, Book, Candles, Trades, Instruments), User Data (Executions, Balances), User Trading (Add/Amend/Edit/Cancel, etc.).
- **Fully typed** models: no placeholders or stubs for request/response fields.
- **Rate limiting**: A token-bucket approach (via [governor] or similar) can be configured.
- **Integration tests**: Local mocking for the WebSocket, real environment tests for REST (if you provide credentials).

## Requirements

- **Rust** (edition 2021 or later).
- **Cargo** for dependency management.
- An **Internet connection** (for real calls to Kraken).
- **Kraken API Key & Secret** if you need private endpoints (REST) or user trading/ private data over WebSocket.
- Optionally, a **WebSocket token** for private data/trading (obtained via `GetWebSocketsToken` from the REST API).

## Installation

In your `Cargo.toml`:

```toml
[dependencies]
kraken-client = { git = "https://github.com/yourorg/kraken-client-rs.git", branch = "main" }
```

_(Adjust the Git URL or version to match your fork/repo.)_

Then `cargo build` to download and compile.

## REST Usage

**REST** endpoints are in a `KrakenClient` struct (example name) with methods for:

- **Public**: `get_server_time`, `get_system_status`, `get_asset_info`, `get_ticker_information`, etc.
- **Private**: `get_balance`, `get_trade_balance`, `get_open_orders`, `add_order`, etc.

**Example** snippet (in `main.rs` or anywhere):

```rust
use kraken_client::KrakenClient;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("KRAKEN_API_KEY").ok();
    let api_secret = env::var("KRAKEN_API_SECRET").ok();

    // Create a rate-limited client
    let client = KrakenClient::new(api_key, api_secret, None, 3, 2); // (key, secret, base_url?, requests/sec, burst)

    // Public call: get server time
    match client.get_server_time().await {
        Ok(time_resp) => println!("Server time: {:?}", time_resp),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Private call: get balance
    match client.get_balance().await {
        Ok(balance) => println!("Balance: {:?}", balance.balances),
        Err(e) => eprintln!("Error fetching balance: {}", e),
    }
}
```

## WebSocket Usage

**Spot WebSocket API v2** is covered by a `KrakenWsClient`:

- **Connect** with `KrakenWsClient::connect("wss://ws.kraken.com/v2").await?`.
- **Split** internally into read (stream) and write (sink), spawning a read loop.
- **Send** typed requests like `ping`, `authorize`, `subscribe`, `add_order`.
- **Receive** typed responses automatically in the background task, printing or logging them.

**Example** in `main.rs`:

```rust
use kraken_client::ws_client::KrakenWsClient;
use kraken_client::models_ws::WsSubscriptionPayload;
use std::env;

#[tokio::main]
async fn main() {
    let url = env::var("WS_URL").unwrap_or("wss://ws.kraken.com/v2".to_string());
    let token = env::var("KRAKEN_WS_TOKEN").ok();

    let client = KrakenWsClient::connect(&url).await.expect("Failed to connect");

    // Authorize if you have a token for private data/trading
    if let Some(t) = token {
        client.authorize(&t, Some(1)).await.expect("Auth failed");
    }

    // Send a ping
    client.send_ping(Some(2)).await.expect("Ping failed");

    // Subscribe to ticker updates for BTC/USD
    client.subscribe(
        WsSubscriptionPayload::Ticker { symbol: "XBT/USD".to_string() },
        Some(3),
    ).await.expect("Subscribe failed");

    println!("Connected to {url}, listening...");
    loop {
        // Just wait indefinitely to see inbound messages
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}
```

**Key Points**:

- The **read loop** is automatically spawned, so inbound messages (like ticker updates, user trading confirmations) are continuously processed.
- No `clone()` errors—**split** approach is used internally.
- Use the provided typed request methods (`add_order`, `amend_order`, `cancel_order`, etc.) for user trading flows.

## Testing & Integration

### REST Testing

- **Unit tests**: Each REST endpoint can have a unit test with **mock** responses.
- **Live integration**: Export your real `KRAKEN_API_KEY` and `KRAKEN_API_SECRET`, then call the endpoints.

```bash
export KRAKEN_API_KEY="..."
export KRAKEN_API_SECRET="..."
cargo test -- --nocapture
```

### WebSocket Testing

1. **Local Integration Test**:

   - A file like `tests/ws_integration_test.rs` can spin up a local WebSocket server (using `tokio_tungstenite`), accept one client, read a message, and close.
   - The `KrakenWsClient` connects to `ws://127.0.0.1:XXXX` for offline test.

2. **Live**:
   - Set `WS_URL="wss://ws.kraken.com/v2"`, optionally `KRAKEN_WS_TOKEN` if you want private streams.
   - `cargo test -- --nocapture` or run a dedicated test that verifies you can ping, subscribe, etc.

**Example** local test snippet:

```rust
// tests/ws_integration_test.rs

#[tokio::test]
async fn test_local_websocket_integration() {
    // Start local server on ephemeral port
    // Connect with KrakenWsClient::connect("ws://127.0.0.1:some_port")
    // Send a ping, check server logs
    // ...
}
```

## Production Considerations

- **Secrets**: Do **not** commit your API key/secret to source control. Use environment variables or a secure vault.
- **Rate-Limiting**: Adjust the token-bucket quotas for REST usage. For the WebSocket, handle **subscribe**/**unsubscribe** calls carefully.
- **Reconnection**: If the WebSocket closes or errors, you may want to automatically reconnect and resubscribe. The example does not show an automatic reconnection logic.
- **Logging**: Expand upon the `eprintln!` calls to a structured logger if you need advanced observability.

## Final Notes

With this library, you can **fully** interact with Kraken's **Spot REST** and **Spot WebSocket API v2** in **Rust**, using **typed** models for **all** endpoints, **no** stubs. Feel free to open issues or PRs to keep it updated as Kraken evolves. Happy trading!
