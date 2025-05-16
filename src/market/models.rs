use alloy::primitives::Address;
// Market-related model definitions
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// Re-export components from the original websocket and info modules related to market data

// WebSocket Request Components for Market Data
#[derive(Debug, Serialize)]
pub struct WsRequest {
    pub method: Method,
    pub subscription: Subscription,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Method {
    Subscribe,
    Unsubscribe,
}
/*

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum HyperliquidSubscription {
    AllMids,
    Notification {
        user: Address,
    },
    OrderUpdates {
        user: Address,
    },
    User {
        user: Address,
    },
    L2Book {
        coin: String,
    },
    Trades {
        coin: String,
    },
    Candle {
        coin: String,
        interval: HyperliquidCandleInterval,
    },
    UserNonFundingLedgerUpdates {
        user: Address,
    },
    UserFundings {
        user: Address,
    },
    UserFills {
        user: Address,
    },
    Bbo {
        coin: String,
    },
    Liquidation {
        user: Address,
    },
    NonUserCancel {
        coin: String,
    },
    UserFunding {
        user: Address,
    },
}
 */
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Subscription {
    AllMids,
    Notification {
        user: Address,
    },
    OrderUpdates {
        user: Address,
    },
    User {
        user: Address,
    },
    L2Book {
        coin: String,
    },
    Trades {
        coin: String,
    },
    Candle {
        coin: String,
        interval: CandleInterval,
    },
    UserNonFundingLedgerUpdates {
        user: Address,
    },
    UserFundings {
        user: Address,
    },
    UserFills {
        user: Address,
    },
    Bbo {
        coin: String,
    },
}

// Define candle interval enum in market model
#[derive(Debug, Serialize, Clone)]
pub enum CandleInterval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "1d")]
    OneDay,
}

// WebSocket Response Components for Market Data
#[derive(Debug, Deserialize)]
#[serde(tag = "channel", content = "data")]
pub enum WsResponse {
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "subscriptionResponse")]
    SubscriptionResponse(SubscriptionResponse),
    #[serde(rename = "l2Book")]
    L2Book(WsBook),
    #[serde(rename = "trades")]
    Trades(Vec<WsTrade>),
    #[serde(rename = "allMids")]
    AllMids(Vec<WsMid>),
    #[serde(rename = "candle")]
    Candle(CandleSnapshot),
    #[serde(rename = "bbo")]
    Bbo(WsBbo),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionResponse {
    pub method: String,
    pub subscription: serde_json::Value,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct PriceLevel {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub px: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub sz: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct WsBook {
    pub coin: String,
    pub time: u64,
    pub levels: (Vec<PriceLevel>, Vec<PriceLevel>),
}

#[derive(Debug, Deserialize)]
pub struct WsBbo {
    pub coin: String,
    pub time: u64,
    pub bbo: (Option<PriceLevel>, Option<PriceLevel>),
}

#[derive(Debug, Deserialize)]
pub struct WsTrade {
    pub coin: String,
    pub side: String,
    pub px: f64,
    pub sz: f64,
    pub time: u64,
    pub hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WsMid {
    pub coin: String,
    pub mid: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CandleSnapshot {
    pub symbol: String,
    pub interval: String,
    pub time_start: u64,
    pub time_end: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}
