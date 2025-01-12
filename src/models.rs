use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//
// ──────────────────────────────────────────────────────────────────────────────
//   1. PUBLIC ENDPOINTS
//   (Time, SystemStatus, Assets, AssetPairs, Ticker, OHLC, Depth, Trades, Spread)
// ──────────────────────────────────────────────────────────────────────────────
//

/// /0/public/Time
#[derive(Debug, Deserialize, Serialize)]
pub struct ServerTimeResponse {
    pub unixtime: u64,
    pub rfc1123: String,
}

/// /0/public/SystemStatus
#[derive(Debug, Deserialize, Serialize)]
pub struct SystemStatusResponse {
    pub status: String,
    pub timestamp: String,
}

/// /0/public/Assets
///
/// The result is a map from asset symbol (e.g. "ADA") to its info.
#[derive(Debug, Deserialize, Serialize)]
pub struct AssetInfoResponse {
    #[serde(flatten)]
    pub assets: HashMap<String, AssetInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetInfo {
    pub aclass: String,
    pub altname: String,
    pub decimals: u32,
    pub display_decimals: u32,
}

/// /0/public/AssetPairs
///
/// The result is a map from pair name (e.g. "XBTUSD") to its info.
#[derive(Debug, Deserialize, Serialize)]
pub struct AssetPairsResponse {
    #[serde(flatten)]
    pub pairs: HashMap<String, AssetPairInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetPairInfo {
    pub altname: Option<String>,
    pub wsname: Option<String>,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,

    /// Usually "lot": "unit"
    pub lot: String,

    /// Number of price decimal places
    pub pair_decimals: u32,

    /// Number of lot decimal places
    pub lot_decimals: u32,

    /// Multiplicator for lot volume. Usually 1
    pub lot_multiplier: u32,

    /// Array of leverage amounts (long side)
    pub leverage_buy: Option<Vec<u32>>,

    /// Array of leverage amounts (short side)
    pub leverage_sell: Option<Vec<u32>>,

    /// Tiers for fees, as arrays of [volume, percentFee]
    pub fees: Vec<Vec<f64>>,

    /// Tiers for maker fees, if any
    pub fees_maker: Option<Vec<Vec<f64>>>,

    /// Volume currency for calculating fees
    pub fee_volume_currency: Option<String>,

    /// Margin call level
    pub margin_call: Option<u32>,

    /// Stop-out level
    pub margin_stop: Option<u32>,

    /// Minimal order volume. Some pairs have "ordermin"
    pub ordermin: Option<String>,

    /// Minimal cost. Some pairs have "costmin"
    pub costmin: Option<String>,

    /// Precision for cost
    pub costprecision: Option<u32>,

    /// Minimal lot. Some pairs have "lotmin"
    pub lotmin: Option<String>,

    /// Tick size. Some pairs have "tick_size"
    pub tick_size: Option<String>,

    /// Pair status: "online", "cancel_only", "post_only", "disabled", or "maintenance"
    pub status: Option<String>,

    /// "true"/"false" or missing
    pub tradable: Option<bool>,
}

/// /0/public/Ticker
///
/// Maps pair name to TickerInfo (which holds: ask, bid, last trade, etc.)
#[derive(Debug, Deserialize, Serialize)]
pub struct TickerResponse {
    #[serde(flatten)]
    pub tickers: HashMap<String, TickerInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TickerInfo {
    /// Ask array: [price, wholeLotVolume, lotVolume]
    pub a: [String; 3],
    /// Bid array: [price, wholeLotVolume, lotVolume]
    pub b: [String; 3],
    /// Last trade array: [price, lotVolume]
    pub c: [String; 2],
    /// Volume array: [todayVolume, last24HoursVolume]
    pub v: [String; 2],
    /// Volume-weighted average price array: [todayVWAP, last24HoursVWAP]
    pub p: [String; 2],
    /// Number of trades array: [todayTrades, last24HoursTrades]
    pub t: [u64; 2],
    /// Low array: [todayLow, last24HoursLow]
    pub l: [String; 2],
    /// High array: [todayHigh, last24HoursHigh]
    pub h: [String; 2],
    /// Today's opening price
    pub o: String,
}

/// /0/public/OHLC
///
/// Kraken returns something like:
/// {
///   "error": [],
///   "result": {
///     "XBTUSD": [
///       [time, open, high, low, close, vwap, volume, count], ...
///     ],
///     "last": 123456789
///   }
/// }
#[derive(Debug, Deserialize, Serialize)]
pub struct OhlcDataResponse {
    #[serde(flatten)]
    pub result: HashMap<String, serde_json::Value>,
}

/// /0/public/Depth
///
/// The result often has the pair name => { "asks": [...], "bids": [...] }
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBookResponse {
    #[serde(flatten)]
    pub orderbook: HashMap<String, OrderBookData>,
}

/// The typical format is:
/// {
///   "asks": [[price, volume, timestamp], ...],
///   "bids": [[price, volume, timestamp], ...]
/// }
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderBookData {
    pub asks: Vec<[String; 3]>,
    pub bids: Vec<[String; 3]>,
}

/// /0/public/Trades
///
/// Maps pair => list of trades, plus "last" => last trade timestamp
#[derive(Debug, Deserialize, Serialize)]
pub struct TradesResponse {
    #[serde(flatten)]
    pub trades: HashMap<String, serde_json::Value>,
}

/// /0/public/Spread
///
/// Maps pair => list of spreads, plus "last" => last timestamp
#[derive(Debug, Deserialize, Serialize)]
pub struct SpreadsResponse {
    #[serde(flatten)]
    pub spreads: HashMap<String, serde_json::Value>,
}

//
// ──────────────────────────────────────────────────────────────────────────────
//   2. PRIVATE ENDPOINTS (AUTH REQUIRED)
//   (Balance, BalanceEx, TradeBalance, OpenOrders, etc.)
// ──────────────────────────────────────────────────────────────────────────────
//

/// /0/private/Balance
///
/// Returns a map from asset code (e.g. "ZUSD", "XXBT") => string representing balance amount
#[derive(Debug, Deserialize, Serialize)]
pub struct AccountBalanceResponse {
    #[serde(flatten)]
    pub balances: HashMap<String, String>,
}

/// /0/private/BalanceEx
///
/// Typically same format as /Balance, but extended. If additional fields appear, you can add them here.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExtendedBalanceResponse {
    #[serde(flatten)]
    pub balances: HashMap<String, String>,
}

/// /0/private/TradeBalance
///
/// Fields documented at:
/// https://docs.kraken.com/rest/#tag/User-Data/operation/getTradeBalance
#[derive(Debug, Deserialize, Serialize)]
pub struct TradeBalanceResponse {
    /// Equivalent balance (combined balance of all currencies)
    pub eb: String,
    /// Trade balance (combined balances of all equity currencies)
    pub tb: String,
    /// Margin amount of open positions
    pub m: String,
    /// Unrealized net profit/loss of open positions
    pub n: String,
    /// Cost basis of open positions
    pub c: String,
    /// Current floating valuation of open positions
    pub v: String,
    /// Equity = trade balance + unrealized net profit/loss
    pub e: String,
    /// Free margin = equity - initial margin (maximum margin available to open new positions)
    pub mf: String,
    /// Margin level = (equity / initial margin) * 100
    pub ml: String,
}

/// /0/private/OpenOrders
///
/// Returns { "open": { "order_txid": { ... }, ... }, "count": optional }
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenOrdersResponse {
    pub open: HashMap<String, OrderInfo>,
    /// Some calls also return "count"
    pub count: Option<u64>,
}

/// /0/private/ClosedOrders
///
/// Returns { "closed": { "order_txid": { ... }, ... }, "count": ... }
#[derive(Debug, Deserialize, Serialize)]
pub struct ClosedOrdersResponse {
    pub closed: HashMap<String, OrderInfo>,
    pub count: Option<u64>,
}

/// Common structure for describing an order in open/closed orders
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderInfo {
    pub refid: Option<String>,
    pub userref: Option<u64>,
    /// "pending", "open", "closed", "canceled", "expired"
    pub status: String,
    /// Unix timestamp when order was placed
    pub opentm: f64,
    /// Unix timestamp for order start time (if set)
    pub starttm: f64,
    /// Unix timestamp for order end time (if set)
    pub expiretm: f64,
    /// The order description
    pub descr: OrderDescription,
    /// Volume of order (base currency)
    pub vol: String,
    /// Volume executed (base currency)
    pub vol_exec: String,
    /// Total cost (quote currency)
    pub cost: String,
    /// Total fee (quote currency)
    pub fee: String,
    /// Average price (quote currency)
    pub price: String,
    /// Stop price (for stop orders)
    pub stopprice: String,
    /// Limit price (for limit orders)
    pub limitprice: String,
    /// Miscellaneous info
    pub misc: String,
    /// "oflags" might include "fciq", "fciq", "post", etc.
    pub oflags: String,
    /// If partial fill occurred, "trades" might exist
    pub trades: Option<Vec<String>>,
    /// Additional fields sometimes appear (e.g. "reason")
    pub reason: Option<String>,
}

/// Detailed order description
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderDescription {
    /// The trading pair (e.g. "XBTUSD")
    pub pair: String,
    /// "buy" or "sell"
    pub side: String,
    /// "market", "limit", "stop-loss", etc.
    pub ordertype: String,
    /// Primary price
    pub price: String,
    /// Secondary price
    pub price2: String,
    /// Leverage. "none" or numeric string
    pub leverage: String,
    /// Plaintext description
    pub order: Option<String>,
    /// Optional stop-related fields
    pub close: Option<String>,
}

/// /0/private/QueryOrders
///
/// Returns a map: { "order_txid": OrderInfo, ... }
#[derive(Debug, Deserialize, Serialize)]
pub struct QueryOrdersResponse {
    #[serde(flatten)]
    pub orders: HashMap<String, OrderInfo>,
}

/// /0/private/TradesHistory
#[derive(Debug, Deserialize, Serialize)]
pub struct TradesHistoryResponse {
    /// Map of trade ID => TradeInfo
    pub trades: HashMap<String, TradeInfo>,
    /// Total number of trades matching request
    pub count: u64,
}

/// /0/private/QueryTrades
///
/// Similar structure but just the map of trades (no count).
#[derive(Debug, Deserialize, Serialize)]
pub struct QueryTradesResponse {
    #[serde(flatten)]
    pub trades: HashMap<String, TradeInfo>,
}

/// Detailed info for a single trade
#[derive(Debug, Deserialize, Serialize)]
pub struct TradeInfo {
    /// Order responsible for execution of this trade
    pub ordertxid: String,
    /// Position ID if it results in a position
    pub postxid: String,
    /// The pair traded (e.g. "XBTUSD")
    pub pair: String,
    /// Unix timestamp of execution
    pub time: f64,
    /// "buy" or "sell"
    #[serde(rename = "type")]
    pub trade_type: String,
    /// "market", "limit", etc.
    pub ordertype: String,
    /// The price (quote currency)
    pub price: String,
    /// The cost (quote currency)
    pub cost: String,
    /// The fee (quote currency)
    pub fee: String,
    /// The volume (base currency)
    pub vol: String,
    /// The margin
    pub margin: String,
    /// Additional info (often empty)
    pub misc: String,
}

/// /0/private/OpenPositions
///
/// Returns a map: { "position_txid": PositionInfo, ... }.
#[derive(Debug, Deserialize, Serialize)]
pub struct OpenPositionsResponse {
    #[serde(flatten)]
    pub positions: HashMap<String, PositionInfo>,
}

/// Detailed info for an open position
#[derive(Debug, Deserialize, Serialize)]
pub struct PositionInfo {
    /// "order_txid" is the order ID that opened the position
    pub ordertxid: String,
    /// "posstatus": e.g. "open"
    pub posstatus: String,
    /// The pair
    pub pair: String,
    /// "buy" or "sell"
    #[serde(rename = "type")]
    pub side: String,
    /// "market", "limit", etc.
    pub ordertype: String,
    /// The average entry price
    pub cost: String,
    /// The total fee
    pub fee: String,
    /// The volume (base currency)
    pub vol: String,
    /// The volume executed
    pub vol_closed: String,
    /// The cost for the closed portion
    pub cost_closed: String,
    /// The fee for the closed portion
    pub fee_closed: String,
    /// The net profit/loss for the closed portion
    pub pl_closed: String,
    /// The margin used
    pub margin: String,
    /// Some positions might include "terms", "rollover_time", "misc", etc.
    pub terms: Option<String>,
    pub rollover_time: Option<f64>,
    pub misc: Option<String>,
}

/// /0/private/Ledgers
#[derive(Debug, Deserialize, Serialize)]
pub struct LedgersResponse {
    pub ledger: HashMap<String, LedgerInfo>,
    pub count: u64,
}

/// /0/private/QueryLedgers
#[derive(Debug, Deserialize, Serialize)]
pub struct QueryLedgersResponse {
    #[serde(flatten)]
    pub ledgers: HashMap<String, LedgerInfo>,
}

/// Detailed ledger info
#[derive(Debug, Deserialize, Serialize)]
pub struct LedgerInfo {
    pub refid: String,
    /// Unix timestamp
    pub time: f64,
    /// e.g. "trade", "withdrawal", "deposit", etc.
    #[serde(rename = "type")]
    pub ledger_type: String,
    /// e.g. "spend", "receive", "rollover", etc.
    pub subtype: Option<String>,
    pub aclass: String,
    /// The asset code
    pub asset: String,
    /// Amount change
    pub amount: String,
    /// Fee (if any)
    pub fee: String,
    /// Resulting balance
    pub balance: String,
}

/// /0/private/TradeVolume
#[derive(Debug, Deserialize, Serialize)]
pub struct TradeVolumeResponse {
    /// The currency used for fee calculations
    pub currency: String,
    /// Volume in the currency
    pub volume: String,
    /// "fees" => map from pair => FeeInfo
    #[serde(default)]
    pub fees: HashMap<String, FeeInfo>,
    /// "fees_maker" => map from pair => FeeInfo
    #[serde(default)]
    pub fees_maker: HashMap<String, FeeInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeeInfo {
    /// Current fee in percent
    pub fee: String,
    /// Minimum fee
    pub minfee: Option<String>,
    /// Maximum fee
    pub maxfee: Option<String>,
    /// Next tier volume
    pub nextfee: Option<String>,
    /// Next tier fee in percent
    pub nextvolume: Option<String>,
    /// Tier volume
    pub tier_volume: Option<String>,
}

/// /0/private/ExportTrades
#[derive(Debug, Deserialize, Serialize)]
pub struct ExportTradesResponse {
    /// ID of the report requested
    pub id: String,
    /// Additional fields
    pub descr: Option<String>,
}

/// /0/private/ExportStatus
#[derive(Debug, Deserialize, Serialize)]
pub struct ExportStatusResponse {
    /// Array of reports
    pub reports: Vec<ExportReportStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportReportStatus {
    pub id: String,
    pub report: String,
    pub format: String,
    pub description: String,
    /// "Queued", "Processing", "Finished", "Error", etc.
    pub status: String,
    /// Unix timestamp
    pub createdtm: f64,
    pub finishtm: Option<f64>,
    pub starttm: Option<f64>,
    pub totalrows: Option<u64>,
    pub refid: Option<String>,
}

/// /0/private/RetrieveExport
#[derive(Debug, Deserialize, Serialize)]
pub struct RetrieveExportResponse {
    /// Potentially base64 or raw data
    pub file: Option<String>,
    /// If there's an error
    pub error: Option<String>,
}

/// /0/private/DeleteExport
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteExportResponse {
    /// Possibly "id", "message", etc.
    pub id: String,
    pub message: Option<String>,
}

/// /0/private/AddOrder
///
/// Returns a descriptor and an array of transaction IDs
#[derive(Debug, Deserialize, Serialize)]
pub struct AddOrderResponse {
    pub descr: AddOrderDescr,
    /// Array of txids created
    pub txid: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddOrderDescr {
    /// Text describing the order (e.g. "buy 0.1 XBT/USD @ limit 30000.0")
    pub order: String,
    /// Possibly a "close" description if a close order was set
    pub close: Option<String>,
}

/// /0/private/AddOrderBatch
///
/// Returns array of results for each order. Each includes "descr", "txid", or error
#[derive(Debug, Deserialize, Serialize)]
pub struct AddOrderBatchResponse {
    pub results: Vec<AddOrderBatchItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddOrderBatchItem {
    pub descr: Option<String>,
    pub txid: Option<Vec<String>>,
    /// If an error occurred
    pub error: Option<String>,
}

/// /0/private/AmendOrder
#[derive(Debug, Deserialize, Serialize)]
pub struct AmendOrderResponse {
    /// Number of orders amended
    pub count: u32,
    /// If pending further actions
    pub pending: bool,
    /// If there's a "description"
    pub descr: Option<AddOrderDescr>,
}

/// /0/private/EditOrder
#[derive(Debug, Deserialize, Serialize)]
pub struct EditOrderResponse {
    /// Number of orders edited
    pub count: u32,
    /// If pending
    pub pending: bool,
    /// Possibly additional fields
    pub descr: Option<AddOrderDescr>,
}

/// /0/private/CancelOrder
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelOrderResponse {
    /// Number of orders canceled
    pub count: u32,
    /// If some orders remain pending
    pub pending: bool,
}

/// /0/private/CancelAll
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelAllOrdersResponse {
    /// Number of orders canceled
    pub count: u32,
}

/// /0/private/CancelAllOrdersAfter
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelAllOrdersAfterResponse {
    /// e.g. "trigger set" or similar
    pub current_time: Option<String>,
    pub trigger_time: Option<String>,
}

/// /0/private/CancelOrderBatch
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelOrderBatchResponse {
    /// Array of results for each order
    pub results: Vec<CancelOrderResponse>,
}

/// /0/private/GetWebSocketsToken
#[derive(Debug, Deserialize, Serialize)]
pub struct GetWebSocketsTokenResponse {
    pub token: String,
    pub expires: u64,
}

//
// ──────────────────────────────────────────────────────────────────────────────
//   3. FUNDING ENDPOINTS
//   (DepositMethods, DepositAddresses, DepositStatus, WithdrawalMethods, etc.)
// ──────────────────────────────────────────────────────────────────────────────
//

/// /0/private/DepositMethods
#[derive(Debug, Deserialize, Serialize)]
pub struct DepositMethodsResponse(pub Vec<DepositMethod>);

#[derive(Debug, Deserialize, Serialize)]
pub struct DepositMethod {
    pub method: String,
    /// True/False as a string or boolean
    pub limit: bool,
    /// Fee (if any)
    pub fee: String,
    /// "AddressSetupOptions" or other
    pub gen_address: bool,
}

/// /0/private/DepositAddresses
#[derive(Debug, Deserialize, Serialize)]
pub struct DepositAddressesResponse(pub Vec<DepositAddressItem>);

#[derive(Debug, Deserialize, Serialize)]
pub struct DepositAddressItem {
    pub address: String,
    pub expiretm: Option<String>,
    pub new: Option<bool>,
    pub qr_code: Option<String>,
}

/// /0/private/DepositStatus
#[derive(Debug, Deserialize, Serialize)]
pub struct DepositStatusResponse(pub Vec<DepositStatusItem>);

#[derive(Debug, Deserialize, Serialize)]
pub struct DepositStatusItem {
    /// "initial", "pending", "success", "failure", etc.
    pub status: String,
    pub txid: Option<String>,
    pub address: Option<String>,
    pub amount: String,
    pub fee: Option<String>,
    pub time: u64,
}

/// /0/private/WithdrawalMethods
#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalMethodsResponse(pub Vec<WithdrawalMethod>);

#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalMethod {
    pub method: String,
    pub limit: bool,
    pub fee: String,
    pub gen_address: bool,
}

/// /0/private/WithdrawalAddresses
#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalAddressesResponse(pub Vec<WithdrawalAddressItem>);

#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalAddressItem {
    pub address: String,
    pub new: Option<bool>,
    pub name: Option<String>,
    pub fee: Option<String>,
}

/// /0/private/WithdrawalInformation
#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalInformationResponse {
    pub method: String,
    /// The limit
    pub limit: String,
    /// The amount
    pub amount: String,
    /// The fee
    pub fee: String,
}

/// /0/private/Withdraw
#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawFundsResponse {
    pub refid: String,
}

/// /0/private/WithdrawStatus
#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawStatusResponse(pub Vec<WithdrawalStatusItem>);

#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawalStatusItem {
    pub method: String,
    pub aclass: Option<String>,
    pub asset: String,
    pub refid: Option<String>,
    pub txid: String,
    pub info: Option<String>,
    pub amount: String,
    /// "Pending", "Success", "Failed", etc.
    pub status: String,
    pub fee: String,
    pub time: u64,
}

/// /0/private/WithdrawCancel
#[derive(Debug, Deserialize, Serialize)]
pub struct WithdrawCancelResponse {
    pub refid: Option<String>,
    /// "true" or "false"
    pub result: bool,
}

/// /0/private/WalletTransfer
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletTransferResponse {
    /// e.g. "some ID" or "null"
    pub refid: Option<String>,
    pub result: Option<String>,
}

//
// ──────────────────────────────────────────────────────────────────────────────
//   4. SUBACCOUNTS
//   (CreateSubaccount, AccountTransfer)
// ──────────────────────────────────────────────────────────────────────────────
//

/// /0/private/CreateSubaccount
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSubaccountResponse {
    /// Subaccount ID
    pub id: String,
    /// Additional fields, e.g. name, success message
    pub name: Option<String>,
}

/// /0/private/AccountTransfer
#[derive(Debug, Deserialize, Serialize)]
pub struct AccountTransferResponse {
    /// Transfer ID
    pub refid: Option<String>,
    /// E.g. success/failure
    pub status: Option<String>,
    pub txid: Option<String>,
}

//
// ──────────────────────────────────────────────────────────────────────────────
//   5. EARN / STAKING
//   (Staking/Stake, Staking/Unstake, GetStakeStatus, GetUnstakeStatus, etc.)
// ──────────────────────────────────────────────────────────────────────────────
//

/// /0/private/Staking/Stake
#[derive(Debug, Deserialize, Serialize)]
pub struct AllocateEarnFundsResponse {
    /// Transaction ID
    pub txid: String,
    /// The asset staked
    pub asset: String,
    /// The amount
    pub amount: String,
    /// The method (e.g. "stake")
    pub method: String,
}

/// /0/private/Staking/Unstake
#[derive(Debug, Deserialize, Serialize)]
pub struct DeallocateEarnFundsResponse {
    /// Transaction ID
    pub txid: String,
    /// The asset
    pub asset: String,
    /// The amount
    pub amount: String,
    /// The method (e.g. "unstake")
    pub method: String,
}

/// /0/private/Staking/GetStakeStatus
#[derive(Debug, Deserialize, Serialize)]
pub struct GetAllocationStatusResponse {
    /// Possibly an array or map with details
    pub status: Vec<StakeStatusItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StakeStatusItem {
    pub txid: String,
    pub asset: String,
    pub amount: String,
    pub status: String,
}

/// /0/private/Staking/GetUnstakeStatus
#[derive(Debug, Deserialize, Serialize)]
pub struct GetDeallocationStatusResponse {
    pub status: Vec<UnstakeStatusItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnstakeStatusItem {
    pub txid: String,
    pub asset: String,
    pub amount: String,
    pub status: String,
}

/// /0/private/Staking/ListStakingProducts
#[derive(Debug, Deserialize, Serialize)]
pub struct ListEarnStrategiesResponse {
    /// Array of available staking products
    pub products: Vec<StakingProduct>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StakingProduct {
    pub asset: String,
    pub title: String,
    pub apy: String,
    pub method: String,
    /// Min / max amounts, locktime, etc.
    pub min_amount: String,
    pub max_amount: Option<String>,
    pub lock_time: Option<u64>,
    pub interval: Option<String>,
}

/// /0/private/Staking/ListStakingTransactions
#[derive(Debug, Deserialize, Serialize)]
pub struct ListEarnAllocationsResponse {
    pub transactions: Vec<StakingTransaction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StakingTransaction {
    pub txid: String,
    pub asset: String,
    pub amount: String,
    pub method: String,
    pub status: String,
    pub time: u64,
    /// Possibly "reward" or other fields
    pub reward: Option<String>,
}

//
// ──────────────────────────────────────────────────────────────────────────────
//   END OF MODELS
// ──────────────────────────────────────────────────────────────────────────────
//
