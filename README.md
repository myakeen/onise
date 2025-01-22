# Onise, Kraken Client for Rust

A **comprehensive, typed, rate-limited, testable** Rust client for:

1. **Kraken's Spot REST API** (all public and private endpoints)
2. **Kraken's Spot WebSocket API v2** (market data, user data, and user trading)

This library provides:

- **Strongly typed** request/response models for Kraken's endpoints
- **Advanced error handling** that parses Kraken's known error codes
- **Configurable rate limiting** (token-bucket or others) to avoid hitting limits
- **Integration tests** (mocked and local) for both REST and WebSocket
- **WebSocket support** with a split read/write approach, typed subscription/unsubscription, user trading, etc.

> **Disclaimer**  
> This is **not** an official Kraken product. Always consult the [official docs](https://docs.kraken.com/rest/) and [WebSocket v2 docs](https://docs.kraken.com/websockets-v2/) for the latest changes or usage policies.

## Features

- **Complete REST coverage**: All documented endpoints (public & private)
- **Spot WebSocket API v2** coverage: Market Data (Ticker, Book, Candles, Trades, Instruments), User Data (Executions, Balances), User Trading (Add/Amend/Edit/Cancel, etc.)
- **Fully typed** models: no placeholders or stubs for request/response fields
- **Rate limiting**: A token-bucket approach (via [governor] or similar) can be configured
- **Integration tests**: Local mocking for the WebSocket, real environment tests for REST (if you provide credentials)

## Requirements

- **Rust** (edition 2021 or later)
- **Cargo** for dependency management
- An **Internet connection** (for real calls to Kraken)
- **Kraken API Key & Secret** if you need private endpoints (REST) or user trading/private data over WebSocket
- Optionally, a **WebSocket token** for private data/trading (obtained via `GetWebSocketsToken` from the REST API)

## Installation

In your `Cargo.toml`:

```toml
[dependencies]
onise = "0.1.0"
# or
# onise = { git = "https://github.com/yourorg/onise-rs.git", branch = "main" }
```

_(Adjust the version or Git URL to match your repository or crates.io version.)_

Then run:

```bash
cargo build
```

to download and compile.

## Usage Modes (REST vs. WebSocket)

Our **`main.rs`** supports **two** modes: **REST** and **WebSocket**, selected by a **command-line argument**:

1. `cargo run -- rest` — Runs the **REST** client logic
2. `cargo run -- ws` — Runs the **WebSocket** client logic
3. Omit or use another argument to default to **REST**

### Example: Running the REST client

```bash
cargo run -- rest
```

- Reads `KRAKEN_API_KEY` and `KRAKEN_API_SECRET` from your environment if you want private endpoint calls
- Calls `get_server_time()`, then calls `get_balance()`

### Example: Running the WebSocket client

```bash
cargo run -- ws
```

- Reads `WS_URL` from environment (defaults to `wss://ws.kraken.com/v2`)
- Optionally reads `KRAKEN_WS_TOKEN` for private data
- Connects, sends a ping, subscribes to a Ticker channel, and loops indefinitely to process incoming messages

You can customize or extend this logic in `main.rs` to handle more endpoints, advanced trading flows, or reconnection strategies.

## REST Usage (API Details)

**REST** endpoints are in a `KrakenClient` struct with methods for:

- **Public**: `get_server_time`, `get_system_status`, `get_asset_info`, `get_ticker_information`, etc.
- **Private**: `get_balance`, `get_trade_balance`, `get_open_orders`, `add_order`, etc.

**Example** snippet (how the code might look if you ran it solely in REST mode):

```rust
use onise::KrakenClient;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("KRAKEN_API_KEY").ok();
    let api_secret = env::var("KRAKEN_API_SECRET").ok();

    // Create a rate-limited client
    let client = KrakenClient::new(api_key, api_secret, None, 3, 2);

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

## WebSocket Usage (API Details)

**Spot WebSocket API v2** is handled by a `KrakenWsClient`:

- **Connect** with `KrakenWsClient::connect("wss://ws.kraken.com/v2").await?`
- **Send** typed requests (e.g., `ping`, `authorize`, `subscribe`, `add_order`)
- **Automatically** spawns a **read loop** to process messages like Ticker updates or ExecutionReports

**Example** (if you ran it in WebSocket mode):

```rust
use onise::ws_client::KrakenWsClient;
use onise::ws_models::WsSubscriptionPayload;
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

## Testing & Integration

### REST Testing

- **Unit tests**: Each REST endpoint can have a unit test with **mock** responses (via [wiremock] or similar)
- **Live integration**: Provide real credentials:

```bash
export KRAKEN_API_KEY="..."
export KRAKEN_API_SECRET="..."
cargo test -- --nocapture
```

### WebSocket Testing

1. **Local Integration Test**:

   - A file like `tests/ws_integration_test.rs` can spin up a local WebSocket server using `tokio_tungstenite`
   - The `KrakenWsClient` connects to `ws://127.0.0.1:some_port`, sends a ping, you confirm on the server side

2. **Live**:
   - Set `WS_URL="wss://ws.kraken.com/v2"`, optionally `KRAKEN_WS_TOKEN` if you want private streams
   - Run `cargo test -- --nocapture` or a dedicated test verifying ping, subscribe, user trading, etc.

## Production Considerations

- **Secrets**: Do **not** commit your API key/secret to version control. Use environment variables or a secure vault
- **Rate-Limiting**: Adjust token-bucket quotas for REST usage; handle `subscribe`/`unsubscribe` carefully in WebSocket usage
- **Reconnection**: For WebSocket, handle reconnection if the socket closes unexpectedly. The example does not show automatic reconnection logic
- **Logging**: Convert simple `eprintln!` calls into structured logs (e.g. with [tracing] or [log]/[env_logger]) if you need advanced debugging

## Final Notes

By **combining** both **REST** and **WebSocket** modes in a **single** binary, you can select between them at runtime with:

```bash
# REST mode
cargo run -- rest

# WebSocket mode
cargo run -- ws
```

Either way, **Onise** offers a robust, typed interface to **Kraken's Spot REST** and **Spot WebSocket API v2**—no stubs, with typed requests and responses for all major endpoints. Enjoy building your Kraken-based applications in Rust!
