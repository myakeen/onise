mod error;
mod models;
mod rate_limiter;

use sha2::Digest;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::Client as HttpClient;
use serde::Deserialize;

use base64::{decode, encode};
use hmac::{Hmac, Mac};
use sha2::Sha512;

use crate::error::{KrakenError, KrakenResult};
use crate::models::*;

/// The standard format from Kraken: if `error` is empty, `result` is the data. Otherwise, we parse the errors.
#[derive(Debug, Deserialize)]
struct KrakenResponse<T> {
    error: Vec<String>,
    result: T,
}

/// A minimal client for **all** Kraken Spot REST endpoints.
#[derive(Clone, Debug)]
pub struct KrakenClient {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub base_url: String,
    http: HttpClient,
}

impl KrakenClient {
    /// Create a new KrakenClient.
    /// - `api_key` and `api_secret` are optional. If they are `None`, private endpoints will fail.
    /// - `base_url` defaults to `https://api.kraken.com` if you don’t override it.
    pub fn new(
        api_key: Option<String>,
        api_secret: Option<String>,
        base_url: Option<String>,
    ) -> Self {
        Self {
            api_key,
            api_secret,
            base_url: base_url.unwrap_or_else(|| "https://api.kraken.com".to_string()),
            http: HttpClient::new(),
        }
    }

    // ─────────────────────────────────────────────────────────────
    // PUBLIC ENDPOINTS (Market Data)
    // ─────────────────────────────────────────────────────────────

    // GET /0/public/Time
    pub async fn get_server_time(&self) -> KrakenResult<ServerTimeResponse> {
        self.public_get("/0/public/Time").await
    }

    // GET /0/public/SystemStatus
    pub async fn get_system_status(&self) -> KrakenResult<SystemStatusResponse> {
        self.public_get("/0/public/SystemStatus").await
    }

    // GET /0/public/Assets
    pub async fn get_asset_info(&self, params: &[(&str, &str)]) -> KrakenResult<AssetInfoResponse> {
        self.public_get_with_params("/0/public/Assets", params)
            .await
    }

    // GET /0/public/AssetPairs
    pub async fn get_asset_pairs(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<AssetPairsResponse> {
        self.public_get_with_params("/0/public/AssetPairs", params)
            .await
    }

    // GET /0/public/Ticker
    pub async fn get_ticker_information(&self, pair: &str) -> KrakenResult<TickerResponse> {
        let p = [("pair", pair)];
        self.public_get_with_params("/0/public/Ticker", &p).await
    }

    // GET /0/public/OHLC
    pub async fn get_ohlc_data(&self, params: &[(&str, &str)]) -> KrakenResult<OhlcDataResponse> {
        self.public_get_with_params("/0/public/OHLC", params).await
    }

    // GET /0/public/Depth
    pub async fn get_order_book(&self, params: &[(&str, &str)]) -> KrakenResult<OrderBookResponse> {
        self.public_get_with_params("/0/public/Depth", params).await
    }

    // GET /0/public/Trades
    pub async fn get_recent_trades(&self, params: &[(&str, &str)]) -> KrakenResult<TradesResponse> {
        self.public_get_with_params("/0/public/Trades", params)
            .await
    }

    // GET /0/public/Spread
    pub async fn get_recent_spreads(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<SpreadsResponse> {
        self.public_get_with_params("/0/public/Spread", params)
            .await
    }

    // ─────────────────────────────────────────────────────────────
    // PRIVATE ENDPOINTS (User Data)
    // ─────────────────────────────────────────────────────────────

    // POST /0/private/Balance
    pub async fn get_balance(&self) -> KrakenResult<AccountBalanceResponse> {
        self.private_post("/0/private/Balance", &[]).await
    }

    // POST /0/private/BalanceEx
    pub async fn get_extended_balance(&self) -> KrakenResult<ExtendedBalanceResponse> {
        self.private_post("/0/private/BalanceEx", &[]).await
    }

    // POST /0/private/TradeBalance
    pub async fn get_trade_balance(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<TradeBalanceResponse> {
        self.private_post("/0/private/TradeBalance", params).await
    }

    // POST /0/private/OpenOrders
    pub async fn get_open_orders(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<OpenOrdersResponse> {
        self.private_post("/0/private/OpenOrders", params).await
    }

    // POST /0/private/ClosedOrders
    pub async fn get_closed_orders(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<ClosedOrdersResponse> {
        self.private_post("/0/private/ClosedOrders", params).await
    }

    // POST /0/private/QueryOrders
    pub async fn query_orders_info(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<QueryOrdersResponse> {
        self.private_post("/0/private/QueryOrders", params).await
    }

    // POST /0/private/TradesHistory
    pub async fn get_trades_history(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<TradesHistoryResponse> {
        self.private_post("/0/private/TradesHistory", params).await
    }

    // POST /0/private/QueryTrades
    pub async fn query_trades_info(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<QueryTradesResponse> {
        self.private_post("/0/private/QueryTrades", params).await
    }

    // POST /0/private/OpenPositions
    pub async fn get_open_positions(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<OpenPositionsResponse> {
        self.private_post("/0/private/OpenPositions", params).await
    }

    // POST /0/private/Ledgers
    pub async fn get_ledgers(&self, params: &[(&str, &str)]) -> KrakenResult<LedgersResponse> {
        self.private_post("/0/private/Ledgers", params).await
    }

    // POST /0/private/QueryLedgers
    pub async fn query_ledgers(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<QueryLedgersResponse> {
        self.private_post("/0/private/QueryLedgers", params).await
    }

    // POST /0/private/TradeVolume
    pub async fn get_trade_volume(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<TradeVolumeResponse> {
        self.private_post("/0/private/TradeVolume", params).await
    }

    // POST /0/private/ExportTrades
    pub async fn request_export_report(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<ExportTradesResponse> {
        self.private_post("/0/private/ExportTrades", params).await
    }

    // POST /0/private/ExportStatus
    pub async fn get_export_report_status(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<ExportStatusResponse> {
        self.private_post("/0/private/ExportStatus", params).await
    }

    // POST /0/private/RetrieveExport
    pub async fn retrieve_export(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<RetrieveExportResponse> {
        self.private_post("/0/private/RetrieveExport", params).await
    }

    // POST /0/private/DeleteExport
    pub async fn delete_export(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<DeleteExportResponse> {
        self.private_post("/0/private/DeleteExport", params).await
    }

    // ─────────────────────────────────────────────────────────────
    // TRADING
    // ─────────────────────────────────────────────────────────────

    // POST /0/private/AddOrder
    pub async fn add_order(&self, params: &[(&str, &str)]) -> KrakenResult<AddOrderResponse> {
        self.private_post("/0/private/AddOrder", params).await
    }

    // POST /0/private/AddOrderBatch
    pub async fn add_order_batch(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<AddOrderBatchResponse> {
        self.private_post("/0/private/AddOrderBatch", params).await
    }

    // POST /0/private/AmendOrder
    pub async fn amend_order(&self, params: &[(&str, &str)]) -> KrakenResult<AmendOrderResponse> {
        self.private_post("/0/private/AmendOrder", params).await
    }

    // POST /0/private/EditOrder
    pub async fn edit_order(&self, params: &[(&str, &str)]) -> KrakenResult<EditOrderResponse> {
        self.private_post("/0/private/EditOrder", params).await
    }

    // POST /0/private/CancelOrder
    pub async fn cancel_order(&self, params: &[(&str, &str)]) -> KrakenResult<CancelOrderResponse> {
        self.private_post("/0/private/CancelOrder", params).await
    }

    // POST /0/private/CancelAll
    pub async fn cancel_all_orders(&self) -> KrakenResult<CancelAllOrdersResponse> {
        self.private_post("/0/private/CancelAll", &[]).await
    }

    // POST /0/private/CancelAllOrdersAfter
    pub async fn cancel_all_orders_after(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<CancelAllOrdersAfterResponse> {
        self.private_post("/0/private/CancelAllOrdersAfter", params)
            .await
    }

    // POST /0/private/CancelOrderBatch
    pub async fn cancel_order_batch(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<CancelOrderBatchResponse> {
        self.private_post("/0/private/CancelOrderBatch", params)
            .await
    }

    // POST /0/private/GetWebSocketsToken
    pub async fn get_websockets_token(&self) -> KrakenResult<GetWebSocketsTokenResponse> {
        self.private_post("/0/private/GetWebSocketsToken", &[])
            .await
    }

    // ─────────────────────────────────────────────────────────────
    // FUNDING
    // ─────────────────────────────────────────────────────────────

    // POST /0/private/DepositMethods
    pub async fn get_deposit_methods(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<DepositMethodsResponse> {
        self.private_post("/0/private/DepositMethods", params).await
    }

    // POST /0/private/DepositAddresses
    pub async fn get_deposit_addresses(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<DepositAddressesResponse> {
        self.private_post("/0/private/DepositAddresses", params)
            .await
    }

    // POST /0/private/DepositStatus
    pub async fn get_deposit_status(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<DepositStatusResponse> {
        self.private_post("/0/private/DepositStatus", params).await
    }

    // POST /0/private/WithdrawalMethods
    pub async fn get_withdrawal_methods(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<WithdrawalMethodsResponse> {
        self.private_post("/0/private/WithdrawalMethods", params)
            .await
    }

    // POST /0/private/WithdrawalAddresses
    pub async fn get_withdrawal_addresses(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<WithdrawalAddressesResponse> {
        self.private_post("/0/private/WithdrawalAddresses", params)
            .await
    }

    // POST /0/private/WithdrawalInformation
    pub async fn get_withdrawal_information(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<WithdrawalInformationResponse> {
        self.private_post("/0/private/WithdrawalInformation", params)
            .await
    }

    // POST /0/private/Withdraw
    pub async fn withdraw_funds(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<WithdrawFundsResponse> {
        self.private_post("/0/private/Withdraw", params).await
    }

    // POST /0/private/WithdrawStatus
    pub async fn get_withdraw_status(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<WithdrawStatusResponse> {
        self.private_post("/0/private/WithdrawStatus", params).await
    }

    // POST /0/private/WithdrawCancel
    pub async fn request_withdrawal_cancellation(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<WithdrawCancelResponse> {
        self.private_post("/0/private/WithdrawCancel", params).await
    }

    // POST /0/private/WalletTransfer
    pub async fn request_wallet_transfer(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<WalletTransferResponse> {
        self.private_post("/0/private/WalletTransfer", params).await
    }

    // ─────────────────────────────────────────────────────────────
    // SUBACCOUNTS
    // ─────────────────────────────────────────────────────────────

    // POST /0/private/CreateSubaccount
    pub async fn create_subaccount(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<CreateSubaccountResponse> {
        self.private_post("/0/private/CreateSubaccount", params)
            .await
    }

    // POST /0/private/AccountTransfer
    pub async fn account_transfer(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<AccountTransferResponse> {
        self.private_post("/0/private/AccountTransfer", params)
            .await
    }

    // ─────────────────────────────────────────────────────────────
    // EARN / STAKING
    // ─────────────────────────────────────────────────────────────

    // POST /0/private/Staking/Stake
    pub async fn allocate_earn_funds(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<AllocateEarnFundsResponse> {
        self.private_post("/0/private/Staking/Stake", params).await
    }

    // POST /0/private/Staking/Unstake
    pub async fn deallocate_earn_funds(
        &self,
        params: &[(&str, &str)],
    ) -> KrakenResult<DeallocateEarnFundsResponse> {
        self.private_post("/0/private/Staking/Unstake", params)
            .await
    }

    // POST /0/private/Staking/GetStakeStatus
    pub async fn get_allocation_status(&self) -> KrakenResult<GetAllocationStatusResponse> {
        self.private_post("/0/private/Staking/GetStakeStatus", &[])
            .await
    }

    // POST /0/private/Staking/GetUnstakeStatus
    pub async fn get_deallocation_status(&self) -> KrakenResult<GetDeallocationStatusResponse> {
        self.private_post("/0/private/Staking/GetUnstakeStatus", &[])
            .await
    }

    // POST /0/private/Staking/ListStakingProducts
    pub async fn list_earn_strategies(&self) -> KrakenResult<ListEarnStrategiesResponse> {
        self.private_post("/0/private/Staking/ListStakingProducts", &[])
            .await
    }

    // POST /0/private/Staking/ListStakingTransactions
    pub async fn list_earn_allocations(&self) -> KrakenResult<ListEarnAllocationsResponse> {
        self.private_post("/0/private/Staking/ListStakingTransactions", &[])
            .await
    }

    // ─────────────────────────────────────────────────────────────
    // HELPER METHODS
    // ─────────────────────────────────────────────────────────────

    /// General public GET helper without query parameters
    async fn public_get<T>(&self, path: &str) -> KrakenResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;

        let parsed = resp.json::<KrakenResponse<T>>().await?;
        if parsed.error.is_empty() {
            Ok(parsed.result)
        } else {
            Err(KrakenError::from_kraken_errors(parsed.error))
        }
    }

    /// General public GET helper with query parameters
    async fn public_get_with_params<T>(
        &self,
        path: &str,
        params: &[(&str, &str)],
    ) -> KrakenResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).query(params).send().await?;

        let parsed = resp.json::<KrakenResponse<T>>().await?;
        if parsed.error.is_empty() {
            Ok(parsed.result)
        } else {
            Err(KrakenError::from_kraken_errors(parsed.error))
        }
    }

    /// Generic private POST call with form parameters
    async fn private_post<T>(&self, path: &str, params: &[(&str, &str)]) -> KrakenResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // Require key/secret to be set
        let api_key = self
            .api_key
            .as_ref()
            .ok_or_else(|| KrakenError::InvalidUsage("API key not set".into()))?;
        let secret = self
            .api_secret
            .as_ref()
            .ok_or_else(|| KrakenError::InvalidUsage("API secret not set".into()))?;

        // Nonce
        let nonce = Self::get_nonce();

        // Build the form data
        let mut form_data = vec![("nonce".to_string(), nonce.to_string())];
        for (k, v) in params {
            form_data.push((k.to_string(), v.to_string()));
        }

        let signature = Self::sign(secret, path, &form_data, nonce)?;

        let url = format!("{}{}", self.base_url, path);
        let resp = self
            .http
            .post(&url)
            .form(&form_data)
            .header("API-Key", api_key)
            .header("API-Sign", signature)
            .send()
            .await?;

        let parsed = resp.json::<KrakenResponse<T>>().await?;
        if parsed.error.is_empty() {
            Ok(parsed.result)
        } else {
            Err(KrakenError::from_kraken_errors(parsed.error))
        }
    }

    /// Create a nonce as microseconds since epoch
    fn get_nonce() -> u64 {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        start.as_micros() as u64
    }

    /// Sign the request for private endpoint
    fn sign(
        secret: &str,
        path: &str,
        form_data: &[(String, String)],
        nonce: u64,
    ) -> KrakenResult<String> {
        // 1) Build the post data string
        let post_data_str = form_data
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");

        // 2) sha256 of (nonce + post_data)
        let mut sha256 = sha2::Sha256::new();
        sha256.update(format!("{}{}", nonce, post_data_str));
        let sha256_bytes = sha256.finalize();

        // 3) path + sha256
        let mut to_sign = Vec::new();
        to_sign.extend_from_slice(path.as_bytes());
        to_sign.extend_from_slice(&sha256_bytes);

        // 4) decode base64 secret
        let decoded_secret = decode(secret).map_err(|_| {
            KrakenError::InvalidUsage("Could not decode API secret from base64".into())
        })?;

        // 5) hmac-sha512
        let mut mac = Hmac::<Sha512>::new_from_slice(&decoded_secret)
            .map_err(|e| KrakenError::InvalidUsage(format!("HMAC error: {e}")))?;
        mac.update(&to_sign);
        let mac_bytes = mac.finalize().into_bytes();

        Ok(encode(mac_bytes))
    }
}
