use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

use onise::error::KrakenResult;
use onise::ws_client::KrakenWsClient;
use onise::ws_models::{WsAdminResponse, WsIncomingMessage, WsPingRequest}; // The client we created

#[tokio::test]
async fn test_local_websocket_integration() -> KrakenResult<()> {
    // 1) Start a local TCP listener on an ephemeral port
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let local_addr = listener.local_addr()?;
    println!("Test WebSocket server listening on {}", local_addr);

    // 2) Spawn a server task that accepts exactly one client, reads a message, then closes
    tokio::spawn(async move {
        if let Ok((stream, addr)) = listener.accept().await {
            println!("Server accepted connection from {addr}");
            handle_ws_connection(stream, addr).await;
        }
    });

    // 3) Build a ws:// URL to connect to our local server
    let ws_url = format!("ws://{}", local_addr);

    // 4) Connect our KrakenWsClient to the local server
    let client = KrakenWsClient::connect(&ws_url).await?;
    println!("Client connected to local test server at {}", ws_url);

    // 5) Send a ping request
    // We'll just do "ping" with req_id=Some(42) for demonstration
    client.send_ping(Some(42)).await?;

    // 6) Possibly wait a bit for server to respond or to ensure no panic
    // For demonstration, short delay:
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // 7) The server side will close the connection after reading the ping.
    // The read loop might see a "Close" message soon.

    Ok(())
}

/// Our server handler for a single WebSocket connection.
/// We'll read one message and optionally respond, then close.
async fn handle_ws_connection(stream: TcpStream, addr: SocketAddr) {
    let addr_string = addr.to_string();
    let ws_result = accept_async(stream).await;

    let mut ws_stream = match ws_result {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Failed to accept websocket from {addr_string}: {e}");
            return;
        }
    };

    println!("Server handshake done, waiting for messages from {addr_string}");

    while let Some(incoming) = ws_stream.next().await {
        match incoming {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => {
                        println!("Server received text: {text}");
                        // We can parse if we want to see if it's a ping
                        match serde_json::from_str::<WsPingRequest>(&text) {
                            Ok(ping_req) => {
                                if ping_req.event == "ping" {
                                    println!(
                                        "Received a ping from client with req_id={:?}",
                                        ping_req.req_id
                                    );
                                    // Optionally, we can send a "pingStatus" or just ignore.
                                    // We'll just log it and then close the socket
                                    let _ = ws_stream.send(Message::Close(None)).await;
                                    break;
                                }
                            }
                            Err(_e) => {
                                println!("Unknown text, ignoring: {text}");
                            }
                        }
                    }
                    Message::Binary(bin) => {
                        println!("Server received binary: {:?}", bin);
                    }
                    Message::Ping(payload) => {
                        println!("Server received tungstenite Ping: {:?}", payload);
                        // Let tungstenite handle the Pong automatically or send ourselves:
                        let _ = ws_stream.send(Message::Pong(payload)).await;
                    }
                    Message::Pong(payload) => {
                        println!("Server received tungstenite Pong: {:?}", payload);
                    }
                    Message::Close(close_frame) => {
                        println!("Server received Close: {:?}", close_frame);
                        break;
                    }
                    Message::Frame(frame) => {
                        println!("Server received raw frame: {:?}", frame);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {e}");
                break;
            }
        }
    }

    println!("Server is closing the connection for {addr_string}");
}
