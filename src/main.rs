// mod error;
// mod lib;

// // #[tokio::main]
// // async fn main() -> Result<(), Box<dyn std::error::Error>> {
// //     println!("Hello, kraken!");
// //     Ok(())
// // }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let time = lib::get_server_time().await?;
//     println!("Server time: {:?}", time);
//     Ok(())
// }
// main.rs
//use kraken_client::KrakenClient; // or your crate::KrakenClient
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
