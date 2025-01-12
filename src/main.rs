mod error;
mod lib;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Hello, kraken!");
//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = lib::get_server_time().await?;
    println!("Server time: {:?}", time);
    Ok(())
}
