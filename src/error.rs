use thiserror::Error;

/// A specialized error type for Kraken.
#[derive(Error, Debug)]
pub enum KrakenError {
    /// HTTP or lower-level I/O error
    #[error("HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// General "Kraken returned an error" with multiple messages
    /// if we cannot interpret them more specifically.
    #[error("Kraken returned error(s): {0:?}")]
    Kraken(Vec<String>),

    /// Some known categories from Kraken's error docs:
    #[error("Kraken general error: {message}")]
    GeneralError { message: String },

    #[error("Kraken API error: {message}")]
    ApiError { message: String },

    #[error("Kraken service error: {message}")]
    ServiceError { message: String },

    #[error("Kraken order error: {message}")]
    OrderError { message: String },

    #[error("Kraken trading error: {message}")]
    TradingError { message: String },

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {message}")]
    RateLimitExceeded { message: String },

    /// For invalid usage, missing credentials, bad parameters, etc.
    #[error("Invalid usage: {0}")]
    InvalidUsage(String),
}

/// We store `KrakenError::Kraken` for multiple error messages, but
/// parse them to see if they match known codes from Kraken docs.
pub type KrakenResult<T> = Result<T, KrakenError>;

impl KrakenError {
    /// Attempt to interpret the Kraken error array for known error codes:
    ///
    /// Examples from docs:
    /// - EGeneral
    /// - EAPI
    /// - EOrder
    /// - EQuery
    /// - ETrade
    /// - EService
    /// - EMarket
    /// - EData
    /// - EFunding
    pub fn from_kraken_errors(errors: Vec<String>) -> Self {
        if errors.is_empty() {
            return KrakenError::Kraken(vec![]);
        }

        // Look at each error in the array. If one matches a known pattern, return early.
        for e in &errors {
            // Rate limit example often includes "EAPI:Rate limit exceeded"
            if e.contains("Rate limit exceeded") {
                return KrakenError::RateLimitExceeded { message: e.clone() };
            }
            // "EAPI:"
            if e.starts_with("EAPI:") {
                return KrakenError::ApiError { message: e.clone() };
            }
            // "EGeneral:"
            if e.starts_with("EGeneral:") {
                return KrakenError::GeneralError { message: e.clone() };
            }
            // "EService:"
            if e.starts_with("EService:") {
                return KrakenError::ServiceError { message: e.clone() };
            }
            // "EOrder:"
            if e.starts_with("EOrder:") {
                return KrakenError::OrderError { message: e.clone() };
            }
            // "ETrade:"
            if e.starts_with("ETrade:") {
                return KrakenError::TradingError { message: e.clone() };
            }
            // "EQuery:"
            if e.starts_with("EQuery:") {
                return KrakenError::GeneralError { message: e.clone() };
            }
            // "EMarket:"
            if e.starts_with("EMarket:") {
                return KrakenError::GeneralError { message: e.clone() };
            }
            // "EData:"
            if e.starts_with("EData:") {
                return KrakenError::GeneralError { message: e.clone() };
            }
            // "EFunding:"
            if e.starts_with("EFunding:") {
                return KrakenError::GeneralError { message: e.clone() };
            }
        }

        // If none matched, store them collectively
        KrakenError::Kraken(errors)
    }
}
