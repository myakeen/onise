use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream};

use crate::error::{KrakenError, KrakenResult};
use crate::ws_models::{
    WsAddOrderRequest,
    WsAdminResponse,
    WsAmendOrderRequest,
    WsAuthorizeRequest,
    WsBatchAddRequest,
    WsBatchCancelRequest,

    WsCancelAllRequest,
    WsCancelOnDisconnectRequest,
    WsCancelOrderRequest,
    WsEditOrderRequest,
    WsHeartbeatRequest,
    // Responses (server → client)
    WsIncomingMessage,
    // Requests (client → server)
    WsPingRequest,
    WsSubscribeRequest,
    WsSubscriptionPayload,
    WsUnsubscribeRequest,
    WsUserTradingResponse,
};

/// `KrakenWsClient` manages a connection to the Spot WebSocket API v2.
/// - It splits the WebSocket into read (stream) and write (sink) halves.
/// - It spawns a task to continuously read messages in `read_loop`.
/// - It offers methods to send typed requests: `ping`, `authorize`, `subscribe`,
///   user trading requests like `add_order`, etc.
/// - It handles all tungstenite `Message` variants, including `Frame(_)`.
/// - It maps inbound JSON into typed `WsIncomingMessage` from `models_ws.rs`.
pub struct KrakenWsClient {
    /// The write half (sink) wrapped in a Mutex for concurrency,
    /// and in an Arc for shared ownership.
    write_half: Arc<
        Mutex<
            futures_util::stream::SplitSink<
                tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
                Message,
            >,
        >,
    >,

    /// If you need an auth token for user data / trading, store it here.
    pub token: Option<String>,
}

impl KrakenWsClient {
    /// Connect to the specified WebSocket `url` (e.g. "wss://ws.kraken.com/v2").
    /// Splits into read & write halves, spawns a read loop task, and returns `KrakenWsClient`.
    pub async fn connect(url: &str) -> KrakenResult<Self> {
        let (ws_stream, _response) = connect_async(url)
            .await
            .map_err(|err| KrakenError::InvalidUsage(format!("WebSocket connect error: {err}")))?;

        // Split into a write sink and read stream
        let (write_half, read_half) = ws_stream.split();

        // Arc<Mutex<...>> so multiple calls can lock and send messages
        let write_half = Arc::new(Mutex::new(write_half));

        // Spawn the read loop in the background
        tokio::spawn(async move {
            if let Err(e) = Self::read_loop(read_half).await {
                eprintln!("Read loop ended with error: {e}");
            }
        });

        Ok(Self {
            write_half,
            token: None,
        })
    }

    /// The continuous read loop. Reads messages, matches their type, and parses
    /// them into `WsIncomingMessage` if they are textual JSON.
    async fn read_loop(
        mut read_half: futures_util::stream::SplitStream<
            tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>,
        >,
    ) -> KrakenResult<()> {
        while let Some(msg_result) = read_half.next().await {
            let msg = msg_result
                .map_err(|err| KrakenError::InvalidUsage(format!("WebSocket read error: {err}")))?;

            match msg {
                Message::Text(text) => {
                    // Attempt to parse the text as WsIncomingMessage
                    match serde_json::from_str::<WsIncomingMessage>(&text) {
                        Ok(incoming) => {
                            Self::handle_incoming(incoming).await;
                        }
                        Err(e) => {
                            eprintln!("Failed to parse text: {e}\nRaw text: {text}");
                        }
                    }
                }
                Message::Binary(bin) => {
                    eprintln!("Received binary message: {bin:?}");
                }
                Message::Ping(payload) => {
                    eprintln!("Received ping: {payload:?}");
                }
                Message::Pong(payload) => {
                    eprintln!("Received pong: {payload:?}");
                }
                Message::Close(close_frame) => {
                    eprintln!("WebSocket closed: {close_frame:?}");
                    break;
                }
                Message::Frame(frame) => {
                    eprintln!("Received raw frame: {frame:?}");
                }
            }
        }
        Ok(())
    }

    /// Handle a typed incoming message variant.
    async fn handle_incoming(msg: WsIncomingMessage) {
        match msg {
            WsIncomingMessage::Admin(admin_resp) => match admin_resp {
                WsAdminResponse::SystemStatus { status, version } => {
                    eprintln!("SystemStatus => status={status}, version={version}");
                }
                WsAdminResponse::SubscriptionStatus {
                    channel,
                    status,
                    req_id,
                    error_message,
                } => {
                    eprintln!(
                            "SubscriptionStatus => channel={channel}, status={status}, req_id={req_id:?}, error={error_message:?}"
                        );
                }
                WsAdminResponse::PingStatus { req_id } => {
                    eprintln!("PingStatus => req_id={req_id:?}");
                }
                WsAdminResponse::Heartbeat {} => {
                    eprintln!("Heartbeat => received");
                }
                WsAdminResponse::Unknown => {
                    eprintln!("Unknown Admin event => unrecognized fields");
                }
            },

            // Market Data
            WsIncomingMessage::TickerMsg(ticker) => {
                eprintln!(
                    "Ticker => symbol={}, bestBid={}, bestAsk={}",
                    ticker.symbol, ticker.best_bid_price, ticker.best_ask_price
                );
            }
            WsIncomingMessage::BookMsg(book) => {
                eprintln!(
                    "Book => symbol={}, #bids={}, #asks={}",
                    book.symbol,
                    book.bids.len(),
                    book.asks.len()
                );
            }
            WsIncomingMessage::CandlesMsg(candles) => {
                eprintln!(
                    "Candles => symbol={}, interval={}, #data={}",
                    candles.symbol,
                    candles.interval,
                    candles.data.len()
                );
            }
            WsIncomingMessage::TradesMsg(trades) => {
                eprintln!(
                    "Trades => symbol={}, #trades={}",
                    trades.symbol,
                    trades.trades.len()
                );
            }
            WsIncomingMessage::InstrumentsMsg(instr) => {
                eprintln!("Instruments => #instruments={}", instr.data.len());
            }

            // User Data
            WsIncomingMessage::BalancesMsg(balances_msg) => {
                eprintln!(
                    "Balances => channel={}, #assets={}",
                    balances_msg.channel,
                    balances_msg.balances.len()
                );
            }
            WsIncomingMessage::ExecutionsMsg(exec_msg) => {
                eprintln!(
                    "Executions => channel={}, #executions={}",
                    exec_msg.channel,
                    exec_msg.executions.len()
                );
            }

            // User Trading
            WsIncomingMessage::Trading(trade_resp) => match trade_resp {
                WsUserTradingResponse::AddOrderStatus {
                    status,
                    txid,
                    req_id,
                    error_message,
                } => {
                    eprintln!(
                            "AddOrderStatus => status={status}, txid={txid:?}, req_id={req_id:?}, error={error_message:?}"
                        );
                }
                WsUserTradingResponse::AmendOrderStatus {
                    status,
                    txid,
                    req_id,
                    error_message,
                } => {
                    eprintln!(
                            "AmendOrderStatus => status={status}, txid={txid:?}, req_id={req_id:?}, error={error_message:?}"
                        );
                }
                WsUserTradingResponse::EditOrderStatus {
                    status,
                    txid,
                    req_id,
                    error_message,
                } => {
                    eprintln!(
                            "EditOrderStatus => status={status}, txid={txid:?}, req_id={req_id:?}, error={error_message:?}"
                        );
                }
                WsUserTradingResponse::CancelOrderStatus {
                    status,
                    txid,
                    req_id,
                    error_message,
                } => {
                    eprintln!(
                            "CancelOrderStatus => status={status}, txid={txid:?}, req_id={req_id:?}, error={error_message:?}"
                        );
                }
                WsUserTradingResponse::CancelAllStatus {
                    status,
                    count,
                    req_id,
                    error_message,
                } => {
                    eprintln!(
                            "CancelAllStatus => status={status}, count={count:?}, req_id={req_id:?}, error={error_message:?}"
                        );
                }
                WsUserTradingResponse::CancelOnDisconnectStatus {
                    status,
                    req_id,
                    error_message,
                } => {
                    eprintln!(
                            "CancelOnDisconnectStatus => status={status}, req_id={req_id:?}, error={error_message:?}"
                        );
                }
                WsUserTradingResponse::BatchAddStatus {
                    status,
                    results,
                    req_id,
                    error_message,
                } => {
                    eprintln!(
                            "BatchAddStatus => status={status}, req_id={req_id:?}, err={error_message:?}, results={:?}",
                            results
                        );
                }
                WsUserTradingResponse::BatchCancelStatus {
                    status,
                    results,
                    req_id,
                    error_message,
                } => {
                    eprintln!(
                            "BatchCancelStatus => status={status}, req_id={req_id:?}, err={error_message:?}, results={:?}",
                            results
                        );
                }
                WsUserTradingResponse::Unknown => {
                    eprintln!("Unknown user trading response => unrecognized fields");
                }
            },

            // CatchAll for untagged or unknown messages
            WsIncomingMessage::CatchAll(unparsed) => {
                eprintln!("CatchAll => unparsed: {unparsed}");
            }
        }
    }

    /// Helper to send a request object T as JSON text over the WebSocket.
    async fn send_message<T: serde::Serialize>(&self, request: &T) -> KrakenResult<()> {
        let json_text = serde_json::to_string(request)
            .map_err(|err| KrakenError::InvalidUsage(format!("Serialize error: {err}")))?;
        let mut sink = self.write_half.lock().await;
        // Use .into() so it matches the expected tungstenite text type
        sink.send(Message::Text(json_text.into()))
            .await
            .map_err(|err| KrakenError::InvalidUsage(format!("WebSocket send error: {err}")))?;
        Ok(())
    }

    // ─────────────────────────────────────────────────────────────────────
    // EXAMPLE HELPER METHODS FOR EACH REQUEST
    // ─────────────────────────────────────────────────────────────────────

    /// Send a ping request (WsPingRequest)
    pub async fn send_ping(&self, req_id: Option<u64>) -> KrakenResult<()> {
        let ping_req = WsPingRequest {
            event: "ping".to_string(),
            req_id,
        };
        self.send_message(&ping_req).await
    }

    /// Send a heartbeat request (WsHeartbeatRequest)
    pub async fn send_heartbeat(&self, req_id: Option<u64>) -> KrakenResult<()> {
        let hb_req = WsHeartbeatRequest {
            event: "heartbeat".to_string(),
            req_id,
        };
        self.send_message(&hb_req).await
    }

    /// Authorize with a token (WsAuthorizeRequest)
    pub async fn authorize(&self, token: &str, req_id: Option<u64>) -> KrakenResult<()> {
        let auth_req = WsAuthorizeRequest {
            event: "authorize".to_string(),
            token: token.to_string(),
            req_id,
        };
        self.send_message(&auth_req).await
    }

    /// Subscribe to a channel (WsSubscribeRequest)
    pub async fn subscribe(
        &self,
        subscription: WsSubscriptionPayload,
        req_id: Option<u64>,
    ) -> KrakenResult<()> {
        let req = WsSubscribeRequest {
            event: "subscribe".to_string(),
            req_id,
            subscription,
        };
        self.send_message(&req).await
    }

    /// Unsubscribe from a channel (WsUnsubscribeRequest)
    pub async fn unsubscribe(
        &self,
        subscription: WsSubscriptionPayload,
        req_id: Option<u64>,
    ) -> KrakenResult<()> {
        let req = WsUnsubscribeRequest {
            event: "unsubscribe".to_string(),
            req_id,
            subscription,
        };
        self.send_message(&req).await
    }

    /// Add order (WsAddOrderRequest)
    pub async fn add_order(&self, add_req: WsAddOrderRequest) -> KrakenResult<()> {
        self.send_message(&add_req).await
    }

    /// Amend order (WsAmendOrderRequest)
    pub async fn amend_order(&self, amend_req: WsAmendOrderRequest) -> KrakenResult<()> {
        self.send_message(&amend_req).await
    }

    /// Edit order (WsEditOrderRequest)
    pub async fn edit_order(&self, edit_req: WsEditOrderRequest) -> KrakenResult<()> {
        self.send_message(&edit_req).await
    }

    /// Cancel order (WsCancelOrderRequest)
    pub async fn cancel_order(&self, cancel_req: WsCancelOrderRequest) -> KrakenResult<()> {
        self.send_message(&cancel_req).await
    }

    /// Cancel all (WsCancelAllRequest)
    pub async fn cancel_all(&self, req: WsCancelAllRequest) -> KrakenResult<()> {
        self.send_message(&req).await
    }

    /// Cancel on disconnect (WsCancelOnDisconnectRequest)
    pub async fn cancel_on_disconnect(&self, req: WsCancelOnDisconnectRequest) -> KrakenResult<()> {
        self.send_message(&req).await
    }

    /// Batch add orders (WsBatchAddRequest)
    pub async fn batch_add(&self, req: WsBatchAddRequest) -> KrakenResult<()> {
        self.send_message(&req).await
    }

    /// Batch cancel orders (WsBatchCancelRequest)
    pub async fn batch_cancel(&self, req: WsBatchCancelRequest) -> KrakenResult<()> {
        self.send_message(&req).await
    }
}
