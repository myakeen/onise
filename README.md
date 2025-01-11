# Onise, A Kraken Client for Rust

A **comprehensive, typed, rate-limited, testable** Rust client for [Kraken's Spot REST API](https://docs.kraken.com/rest/). Supports:

- **Public (unauthenticated)** endpoints: Market Data, System Status, etc.
- **Private (authenticated)** endpoints: Account Balance, Orders, Funding, Subaccounts, Earn, etc.
- **Strongly typed** request/response models for many endpoints, referencing the official Kraken docs.
- **Advanced error handling** that parses Kraken's error codes.
- **Configurable rate limiting** with a token-bucket approach ([governor] crate).
- **Integration tests** (both mocked with [wiremock-rs] and optional live tests).
- **Configurable base URL** for easy testing or custom endpoints.

> **Disclaimer**  
> This library is **not** an official product by Kraken. Always refer to the [official docs](https://docs.kraken.com/rest/) for the latest endpoint details and confirm your usage meets [Kraken's API policies](https://support.kraken.com/hc/en-us/articles/205893708-What-are-the-API-call-rate-limits-).

## Features

- **Fully Typed Models**: Many endpoints have typed structs covering most data fields returned by Kraken.
- **Advanced Error Handling**: Automatically parses common error codes (e.g., `EAPI:`, `EOrder:`, `ETrade:`).
- **Configurable Rate Limiting**: The [governor] token-bucket approach is more flexible than naive semaphores.
- **Integration Testing**: Mocks with [wiremock-rs] or live calls with real credentials.
- **Customizable**: Choose your own base URL, adjust rate-limit parameters, or extend typed models as needed.

## Requirements

- **Rust** (edition 2021 or later)
- **Cargo** (the Rust package manager)
- An **Internet connection** (for actual Kraken calls)
- **API Key & Secret** from [Kraken's account management](https://docs.kraken.com/rest/#section/Authentication) if you want to use **private** (authenticated) endpoints.

## Installation

### Option A: Using `cargo add` (if you publish your library to crates.io)

```bash
cargo add kraken-client
```

Then in your code:

```toml
[dependencies]
kraken-client = "1.0.0"
```

### Option B: Local / Git dependency

If it's **not** on crates.io yet, or you're cloning from a Git repo, add something like this to your project's `Cargo.toml`:

```toml
[dependencies]
kraken-client = { path = "../kraken-client" }
# or from a Git repo:
# kraken-client = { git = "https://github.com/yourorg/kraken-client-rs.git", branch = "main" }
```

## Quick Start

Create a new Rust project or use an existing one:

```bash
cargo new my-kraken-app
cd my-kraken-app
```

Add the client as a dependency (see [Installation](#installation) above). Then in your `main.rs` or library code:

```rust
use kraken_client::{KrakenClient, models::*};
use std::env;

#[tokio::main]
async fn main() {
    // (Optional) Set your Kraken credentials:
    let api_key = env::var("KRAKEN_API_KEY").ok();
    let api_secret = env::var("KRAKEN_API_SECRET").ok();

    // Create client with default https://api.kraken.com base URL,
    // rate limit ~3 requests/sec, plus a small burst of 2
    let client = KrakenClient::new(api_key, api_secret, None, 3, 2);

    // Public endpoint example: get server time
    match client.get_server_time().await {
        Ok(time_resp) => println!("Server time: {:?}", time_resp),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Private endpoint example: get account balance
    match client.get_account_balance().await {
        Ok(balance) => println!("Account balance: {:?}", balance.balances),
        Err(e) => eprintln!("Error fetching balance: {}", e),
    }
}
```

Compile and run:

```bash
cargo run
```

- If you didn't provide valid credentials, **private endpoints** will fail with an `InvalidUsage("API key not set")` error.

## Configuration & Usage

### Public Endpoints

Public endpoints (e.g. `get_server_time`, `get_system_status`, `get_asset_info`, `get_ticker_information`, etc.) **do not** require credentials. Simply call them on the `KrakenClient`:

```rust
// e.g. get system status
let status = client.get_system_status().await?;
println!("System Status: {:?}", status);
```

### Private Endpoints

Private endpoints (e.g. `get_account_balance`, `get_trade_balance`, `get_open_orders`, etc.) **require** a valid API key and secret. Obtain these from your Kraken dashboard and set them in environment variables like:

```bash
export KRAKEN_API_KEY="your_key"
export KRAKEN_API_SECRET="your_secret_in_base64"
```

When you create the client, it will use these credentials for signing requests.

### Rate Limiting

The client uses [governor] for a **token-bucket** approach. You configure it with:

```rust
// new(api_key, api_secret, base_url, requests_per_second, burst_size)
let client = KrakenClient::new(
    Some("key".into()),
    Some("secret".into()),
    None, // => defaults to https://api.kraken.com
    3,    // steady rate: 3 requests/second
    2,    // can burst 2 extra tokens beyond the steady rate
);
```

This helps avoid hitting Kraken's rate-limit errors. Adjust the **requests_per_second** and **burst_size** to your usage pattern. If you need more complex logic, you can modify the code in `rate_limiter.rs`.

### Error Handling

All methods return a `KrakenResult<T>` which is `Result<T, KrakenError>`. The `KrakenError` type:

- Can be **`Reqwest`** errors (for network/HTTP issues).
- Maps **Kraken** errors like `EAPI:Rate limit exceeded` or `EOrder:Invalid order` to specialized variants (`KrakenError::RateLimitExceeded`, `KrakenError::OrderError`, etc.).
- Falls back to a general `KrakenError::Kraken(Vec<String>)` if unrecognized codes are encountered.

You can match on these variants:

```rust
match client.get_server_time().await {
    Ok(resp) => println!("Server time: {:?}", resp),
    Err(KrakenError::RateLimitExceeded { message }) => {
        eprintln!("Hit rate limit: {}", message);
    },
    Err(e) => eprintln!("Some other error: {}", e),
}
```

### Custom Base URL

The constructor accepts an optional `base_url` parameter. By default, it's `https://api.kraken.com`, but you can override it for testing or special routes:

```rust
let client = KrakenClient::new(
    None,
    None,
    Some("http://localhost:8080".into()), // e.g. for mock testing
    5,
    1
);
```

## Running & Testing

### Local Testing

1. **Clone** this repo or add it as a local dependency.
2. **Install** dependencies:
   ```bash
   cargo build
   ```
   This fetches all crates (like `reqwest`, `governor`, `wiremock`, etc.).
3. **Set environment variables** (optional for private endpoints).
4. **Run**:
   ```bash
   cargo run
   ```

### Integration Tests

The library includes tests under the `tests/` directory. There are two types:

1. **Mocked tests** using [wiremock-rs]. These do **not** require real credentials or network calls.
2. **Live tests** that call the real Kraken API. These require valid credentials and typically only run if an environment variable like `ENABLE_LIVE_TESTS=1` is set.

To run all tests:

```bash
cargo test
```

If you only want to run **live** tests, set up credentials and enable them:

```bash
export KRAKEN_API_KEY="your_key"
export KRAKEN_API_SECRET="your_secret"
export ENABLE_LIVE_TESTS=1
cargo test -- --nocapture
```

> **Warning**: Live tests may place real calls against your account. Use them with caution (or ensure your keys are read-only / test-limited).

### Production

In production, you should:

1. **Store** your credentials securely (e.g., environment variables in a secure environment or a secrets manager).
2. **Tune** the rate-limiter (`requests_per_second`, `burst_size`) to your usage and Kraken's policy.
3. **Monitor** the logs and errors, especially for rate-limits or transient network issues.
4. **Update** regularly as Kraken's API may change. Keep an eye on `KrakenError::Kraken(Vec<String>)` for new error codes you might want to handle explicitly.

To build a release version:

```bash
cargo build --release
```

Then run or deploy the resulting binary in your production environment.

## Examples

Below is a simple snippet that uses some typical endpoints:

```rust
use kraken_client::KrakenClient;
use std::env;

#[tokio::main]
async fn main() {
    let api_key = env::var("KRAKEN_API_KEY").ok();
    let api_secret = env::var("KRAKEN_API_SECRET").ok();

    let client = KrakenClient::new(api_key, api_secret, None, 3, 2);

    // Public: get ticker info for BTC/USD
    let ticker = client.get_ticker_information("XBTUSD").await;
    println!("Ticker: {:?}", ticker);

    // Private: get open orders
    let open_orders = client.get_open_orders(&[]).await;
    println!("Open orders: {:?}", open_orders);
}
```

Running:

```bash
cargo run
```

You should see the ticker info printed, and if you have valid credentials, your open orders as well.

## License

This project is available under the **MIT License**, which permits both commercial and private use. See the [LICENSE](LICENSE) file for details.

## Contributing

- **Contributions**: If you find issues or want to add typed models for additional endpoints, please submit a PR or open an issue.
- **Support**: For Kraken-specific issues (e.g., account permissions, API key creation), consult [Kraken's Support Docs](https://support.kraken.com/hc/en-us) directly.

**Enjoy** building with the **Rust Kraken Client**!
