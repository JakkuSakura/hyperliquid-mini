use crate::execution::models::Subscription as HyperliquidSubscription;
use std::time::SystemTimeError;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Reqwest error: {0:?}")]
    Reqwest(reqwest::Error),
    #[error("Timestamp error: {0:?}")]
    TimestampError(SystemTimeError),
    #[error("Wallet error: {0:?}")]
    WalletError(alloy::signers::Error),

    #[error("Not connected")]
    NotConnected,
    #[error("JSON error: {0:?}")]
    Json(serde_json::Error),
    #[error("Not subscribed to channel with id {0}")]
    NotSubscribed(u64),
    #[error("Subscription failed: {0:?}")]
    SubscriptionFailed(HyperliquidSubscription),
    #[error("Missing subscription response: {0:?}")]
    MissingSubscriptionResponse(HyperliquidSubscription),
    #[error("Response error: {0}")]
    ResponseError(String),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<SystemTimeError> for Error {
    fn from(e: SystemTimeError) -> Self {
        Self::TimestampError(e)
    }
}

impl From<alloy::signers::Error> for Error {
    fn from(e: alloy::signers::Error) -> Self {
        Self::WalletError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}
impl Error {
    pub fn response_error(msg: impl Into<String>) -> Self {
        Self::ResponseError(msg.into())
    }
}

static_assertions::assert_impl_all!(Error: Send, Sync);
