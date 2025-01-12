use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//
// ──────────────────────────────────────────────────────────────────────────────
// ── REQUESTS (CLIENT → SERVER) ──────────────────────────────────────────────
// ──────────────────────────────────────────────────────────────────────────────
//

//
// 1. ADMIN / CONTROL / AUTH
//

/// "ping" request
#[derive(Debug, Deserialize, Serialize)]
pub struct WsPingRequest {
    pub event: String, // "ping"
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}

/// "heartbeat" request
#[derive(Debug, Serialize)]
pub struct WsHeartbeatRequest {
    pub event: String, // "heartbeat"
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}

/// "authorize" request (for user data/trading)
#[derive(Debug, Serialize)]
pub struct WsAuthorizeRequest {
    pub event: String, // "authorize"
    pub token: String,
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}

//
// 2. SUBSCRIBE / UNSUBSCRIBE
//

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsSubscribeRequest {
    pub event: String, // "subscribe"
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
    #[serde(flatten)]
    pub subscription: WsSubscriptionPayload,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsUnsubscribeRequest {
    pub event: String, // "unsubscribe"
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
    #[serde(flatten)]
    pub subscription: WsSubscriptionPayload,
}

/// Each subscription has a "name" plus specific fields (symbol, depth, interval, etc.)
#[derive(Debug, Serialize)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum WsSubscriptionPayload {
    Ticker {
        symbol: String,
    },
    Book {
        symbol: String,
        depth: u32,
    },
    Candles {
        symbol: String,
        interval: u32,
    },
    Trades {
        symbol: String,
    },
    Instruments {
        #[serde(skip_serializing_if = "Option::is_none")]
        symbol: Option<String>,
    },
    Orders {
        symbol: String,
    },
    Status,
    Heartbeat,
    Ping,
    Balances,
    Executions,
}

//
// 3. USER TRADING (Add/Amend/Edit/Cancel/Batch, etc.)
//

/// Add Order request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsAddOrderRequest {
    pub event: String, // "addOrder"
    pub token: String,
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,

    #[serde(rename = "orderType")]
    pub order_type: String, // "limit", "market", "stop", etc.
    pub symbol: String,
    pub side: String, // "buy" or "sell"
    pub quantity: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopPrice")]
    pub stop_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "limitPrice")]
    pub limit_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "timeInForce")]
    pub time_in_force: Option<String>, // "GTC", "IOC", "GTD"

    #[serde(skip_serializing_if = "Option::is_none", rename = "expireTime")]
    pub expire_time: Option<String>, // e.g. "2023-12-31T23:59:59Z"

    #[serde(skip_serializing_if = "Option::is_none", rename = "postOnly")]
    pub post_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "reduceOnly")]
    pub reduce_only: Option<bool>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "selfTradePrevention"
    )]
    pub self_trade_prevention: Option<String>, // "decrement", "cancel_old", "cancel_new", etc.

    #[serde(skip_serializing_if = "Option::is_none", rename = "triggerSignal")]
    pub trigger_signal: Option<String>, // "last_price", "index_price", "mark_price", etc.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>, // e.g. "2", "5", "none"

    #[serde(skip_serializing_if = "Option::is_none", rename = "clientOrderId")]
    pub client_order_id: Option<String>,

    // Conditional Close
    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfit")]
    pub take_profit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfitPrice")]
    pub take_profit_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLoss")]
    pub stop_loss: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLossPrice")]
    pub stop_loss_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "conditionalClose")]
    pub conditional_close: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "closePrice")]
    pub close_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfitTrigger")]
    pub take_profit_trigger: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLossTrigger")]
    pub stop_loss_trigger: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "positionId")]
    pub position_id: Option<String>,
}

/// Amend Order request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsAmendOrderRequest {
    pub event: String, // "amendOrder"
    pub token: String,
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
    pub txid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopPrice")]
    pub stop_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "limitPrice")]
    pub limit_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "timeInForce")]
    pub time_in_force: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "expireTime")]
    pub expire_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "postOnly")]
    pub post_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "reduceOnly")]
    pub reduce_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "triggerSignal")]
    pub trigger_signal: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfit")]
    pub take_profit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfitPrice")]
    pub take_profit_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLoss")]
    pub stop_loss: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLossPrice")]
    pub stop_loss_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "conditionalClose")]
    pub conditional_close: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "closePrice")]
    pub close_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfitTrigger")]
    pub take_profit_trigger: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLossTrigger")]
    pub stop_loss_trigger: Option<String>,
}

/// Edit Order request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsEditOrderRequest {
    pub event: String, // "editOrder"
    pub token: String,
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
    pub txid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopPrice")]
    pub stop_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "limitPrice")]
    pub limit_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "timeInForce")]
    pub time_in_force: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "expireTime")]
    pub expire_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "postOnly")]
    pub post_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "reduceOnly")]
    pub reduce_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "triggerSignal")]
    pub trigger_signal: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfit")]
    pub take_profit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfitPrice")]
    pub take_profit_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLoss")]
    pub stop_loss: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLossPrice")]
    pub stop_loss_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "conditionalClose")]
    pub conditional_close: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "closePrice")]
    pub close_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfitTrigger")]
    pub take_profit_trigger: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLossTrigger")]
    pub stop_loss_trigger: Option<String>,
}

/// Cancel Order request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsCancelOrderRequest {
    pub event: String, // "cancelOrder"
    pub token: String,
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
    pub txid: String,
}

/// Cancel All request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsCancelAllRequest {
    pub event: String, // "cancelAll"
    pub token: String,
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
}

/// Cancel On Disconnect request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsCancelOnDisconnectRequest {
    pub event: String, // "cancelOnDisconnect"
    pub token: String,
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
    pub enable: bool,
}

/// Batch Add request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsBatchAddRequest {
    pub event: String, // "batchAdd"
    pub token: String,
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
    pub orders: Vec<BatchAddOrderSpec>,
}

/// One order spec in batchAdd
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAddOrderSpec {
    #[serde(rename = "orderType")]
    pub order_type: String,
    pub symbol: String,
    pub side: String,
    pub quantity: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopPrice")]
    pub stop_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "limitPrice")]
    pub limit_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "timeInForce")]
    pub time_in_force: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "expireTime")]
    pub expire_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "postOnly")]
    pub post_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "reduceOnly")]
    pub reduce_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "triggerSignal")]
    pub trigger_signal: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "clientOrderId")]
    pub client_order_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfit")]
    pub take_profit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfitPrice")]
    pub take_profit_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLoss")]
    pub stop_loss: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLossPrice")]
    pub stop_loss_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "conditionalClose")]
    pub conditional_close: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "closePrice")]
    pub close_price: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "takeProfitTrigger")]
    pub take_profit_trigger: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "stopLossTrigger")]
    pub stop_loss_trigger: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "positionId")]
    pub position_id: Option<String>,
}

/// Batch Cancel request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsBatchCancelRequest {
    pub event: String, // "batchCancel"
    pub token: String,
    #[serde(rename = "req_id", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<u64>,
    pub orders: Vec<String>,
}

//
// ──────────────────────────────────────────────────────────────────────────────
// ── RESPONSES / UPDATES (SERVER → CLIENT) ───────────────────────────────────
// ──────────────────────────────────────────────────────────────────────────────
//

//
// 1. ADMIN / CONTROL
//

#[derive(Debug, Deserialize)]
#[serde(tag = "event", rename_all = "camelCase")]
pub enum WsAdminResponse {
    /// systemStatus
    #[serde(rename = "systemStatus")]
    SystemStatus { status: String, version: String },

    /// subscriptionStatus
    #[serde(rename = "subscriptionStatus")]
    SubscriptionStatus {
        channel: String,
        status: String, // "subscribed" or "unsubscribed"
        #[serde(rename = "req_id", default)]
        req_id: Option<u64>,
        #[serde(default)]
        error_message: Option<String>,
    },

    /// pingStatus or "pong"
    #[serde(rename = "pingStatus")]
    PingStatus {
        #[serde(rename = "req_id", default)]
        req_id: Option<u64>,
    },

    /// heartbeat
    #[serde(rename = "heartbeat")]
    Heartbeat {},

    #[serde(other)]
    Unknown,
}

//
// 2. MARKET DATA
//

/// Ticker message (level 1).
#[derive(Debug, Deserialize)]
pub struct WsTickerMessage {
    pub channel: String,
    pub symbol: String,
    pub best_ask_price: String,
    pub best_ask_quantity: String,
    pub best_bid_price: String,
    pub best_bid_quantity: String,
    pub last_trade_price: String,
    pub last_trade_quantity: String,
    pub volume_24h: String,
    pub vwap_24h: String,
    pub trades_24h: u64,
    pub low_24h: String,
    pub high_24h: String,
    pub open_24h: String,
}

/// Book (level 2) snapshot or updates
#[derive(Debug, Deserialize)]
pub struct WsBookMessage {
    pub channel: String,
    pub symbol: String,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
}

/// One side of the order book
#[derive(Debug, Deserialize)]
pub struct OrderBookEntry {
    pub price: String,
    pub quantity: String,
}

/// Candles (OHLC)
#[derive(Debug, Deserialize)]
pub struct WsCandlesMessage {
    pub channel: String,
    pub symbol: String,
    pub interval: u32,
    pub data: Vec<CandleData>,
}

#[derive(Debug, Deserialize)]
pub struct CandleData {
    pub time: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
}

/// Trades feed
#[derive(Debug, Deserialize)]
pub struct WsTradesMessage {
    pub channel: String,
    pub symbol: String,
    pub trades: Vec<TradeData>,
}

#[derive(Debug, Deserialize)]
pub struct TradeData {
    pub price: String,
    pub quantity: String,
    pub time: u64,
    pub side: String,
}

/// Instruments
#[derive(Debug, Deserialize)]
pub struct WsInstrumentsMessage {
    pub channel: String,
    #[serde(default)]
    pub symbol: Option<String>,
    pub data: Vec<InstrumentData>,
}

#[derive(Debug, Deserialize)]
pub struct InstrumentData {
    pub symbol: String,
    pub status: String,
    pub base_currency: String,
    pub quote_currency: String,
    #[serde(default)]
    pub price_decimals: Option<u32>,
    #[serde(default)]
    pub quantity_decimals: Option<u32>,
    pub marginable: bool,
    pub margin_ratio: String,
    pub max_leverage: String,
    pub min_leverage: String,
    pub maker_fee: String,
    pub taker_fee: String,
    pub min_volume: String,
    pub max_volume: String,
    pub tick_size: String,
    pub lot_size: String,
}

//
// 3. USER DATA (balances, executions)
//

#[derive(Debug, Deserialize)]
pub struct WsBalancesMessage {
    pub channel: String,
    pub balances: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct WsExecutionsMessage {
    pub channel: String,
    pub executions: Vec<ExecutionData>,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionData {
    pub symbol: String,
    pub order_id: String,
    pub exec_id: String,
    pub quantity: String,
    pub price: String,
    pub side: String,
    pub time: u64,
    pub cost: String,
    pub fee: String,
    pub fee_currency: String,
    pub liquidity: String, // "maker" or "taker"
}

//
// 4. USER TRADING RESPONSES (addOrderStatus, etc.)
//

#[derive(Debug, Deserialize)]
#[serde(tag = "event", rename_all = "camelCase")]
pub enum WsUserTradingResponse {
    #[serde(rename = "addOrderStatus")]
    AddOrderStatus {
        status: String,
        #[serde(default)]
        txid: Option<String>,
        #[serde(rename = "req_id", default)]
        req_id: Option<u64>,
        #[serde(default)]
        error_message: Option<String>,
    },

    #[serde(rename = "amendOrderStatus")]
    AmendOrderStatus {
        status: String,
        #[serde(default)]
        txid: Option<String>,
        #[serde(rename = "req_id", default)]
        req_id: Option<u64>,
        #[serde(default)]
        error_message: Option<String>,
    },

    #[serde(rename = "editOrderStatus")]
    EditOrderStatus {
        status: String,
        #[serde(default)]
        txid: Option<String>,
        #[serde(rename = "req_id", default)]
        req_id: Option<u64>,
        #[serde(default)]
        error_message: Option<String>,
    },

    #[serde(rename = "cancelOrderStatus")]
    CancelOrderStatus {
        status: String,
        #[serde(default)]
        txid: Option<String>,
        #[serde(rename = "req_id", default)]
        req_id: Option<u64>,
        #[serde(default)]
        error_message: Option<String>,
    },

    #[serde(rename = "cancelAllStatus")]
    CancelAllStatus {
        status: String,
        #[serde(default)]
        count: Option<u64>,
        #[serde(rename = "req_id", default)]
        req_id: Option<u64>,
        #[serde(default)]
        error_message: Option<String>,
    },

    #[serde(rename = "cancelOnDisconnectStatus")]
    CancelOnDisconnectStatus {
        status: String,
        #[serde(rename = "req_id", default)]
        req_id: Option<u64>,
        #[serde(default)]
        error_message: Option<String>,
    },

    #[serde(rename = "batchAddStatus")]
    BatchAddStatus {
        status: String,
        #[serde(default)]
        results: Option<Vec<BatchAddResult>>,
        #[serde(rename = "req_id", default)]
        req_id: Option<u64>,
        #[serde(default)]
        error_message: Option<String>,
    },

    #[serde(rename = "batchCancelStatus")]
    BatchCancelStatus {
        status: String,
        #[serde(default)]
        results: Option<Vec<BatchCancelResult>>,
        #[serde(rename = "req_id", default)]
        req_id: Option<u64>,
        #[serde(default)]
        error_message: Option<String>,
    },

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct BatchAddResult {
    #[serde(default)]
    pub txid: Option<String>,
    #[serde(default)]
    pub error_message: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub client_order_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BatchCancelResult {
    #[serde(default)]
    pub txid: Option<String>,
    #[serde(default)]
    pub error_message: Option<String>,
}

//
// 5. UNIFIED "WsIncomingMessage" - a top-level enum if you want to parse everything
//

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum WsIncomingMessage {
    Admin(WsAdminResponse),

    // Market Data
    TickerMsg(WsTickerMessage),
    BookMsg(WsBookMessage),
    CandlesMsg(WsCandlesMessage),
    TradesMsg(WsTradesMessage),
    InstrumentsMsg(WsInstrumentsMessage),

    // User Data
    BalancesMsg(WsBalancesMessage),
    ExecutionsMsg(WsExecutionsMessage),

    // User Trading
    Trading(WsUserTradingResponse),

    /// Catch-all for unknown or non-matching messages
    CatchAll(serde_json::Value),
}
