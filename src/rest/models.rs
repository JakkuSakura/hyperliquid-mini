// REST-related model definitions
use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::serde_as;

// Enum for API endpoints
pub enum API {
    Info,
    Exchange,
}

impl API {
    pub fn as_str(&self) -> &str {
        match self {
            API::Info => "/info",
            API::Exchange => "/exchange",
        }
    }
}

// Info request models
#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum InfoRequest {
    #[serde(rename = "meta")]
    Meta,
    #[serde(rename = "spotMeta")]
    SpotMeta,
    #[serde(rename = "metaAndAssetCtxs")]
    MetaAndAssetCtxs,
    #[serde(rename = "clearinghouseState")]
    ClearinghouseState { user: Address },
    #[serde(rename = "userFills")]
    UserFills {
        user: Address,
        startTime: Option<u64>,
    },
    #[serde(rename = "userFunding")]
    UserFunding {
        user: Address,
        start_time: u64,
        end_time: Option<u64>,
    },
    #[serde(rename = "candles")]
    Candles {
        coin: String,
        interval: String,
        startTime: Option<u64>,
        endTime: Option<u64>,
    },
    #[serde(rename = "openOrders")]
    OpenOrders { user: Address },
    #[serde(rename = "assetCtx")]
    AssetCtx { asset: String },
    #[serde(rename = "userPoints")]
    UserPoints { user: Address },
    #[serde(rename = "allMids")]
    AllMids,
    #[serde(rename = "recentTrades")]
    RecentTrades {
        coin: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        startTime: Option<u64>,
    },
    #[serde(rename = "fundingHistory")]
    FundingHistory {
        coin: String,
        start_time: u64,
        end_time: Option<u64>,
    },
    #[serde(rename = "l2Book")]
    L2Book { coin: String },
    #[serde(rename = "candleSnapshot")]
    CandleSnapshot { req: CandleSnapshotRequest },
    #[serde(rename = "orderStatus")]
    OrderStatus { user: Address, oid: u64 },
    #[serde(rename = "orderStatusByCloid")]
    OrderStatusByCloid { user: Address, cloid: String },
}

// Map old Request type to new InfoRequest type for compatibility
pub type Request = InfoRequest;

#[derive(Debug, Serialize)]
pub struct CandleSnapshotRequest {
    pub coin: String,
    pub interval: String,
    pub start_time: u64,
    pub end_time: u64,
}

// Response models
#[derive(Debug, Deserialize)]
pub struct Universe {
    pub universe: Vec<Asset>,
    pub timestamp: Option<u64>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub szDecimals: u32,
}

#[derive(Debug, Deserialize)]
pub struct SpotMetaTokenUniverse {
    pub tokens: Vec<SpotToken>,
}
#[allow(non_snake_case)]
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct SpotToken {
    pub name: String,
    pub szDecimals: u32,
    pub weiDecimals: u32,
    pub index: u32,
    pub tokenId: String,
    pub isCanonical: bool,
    pub evmContract: Option<Value>,
    pub fullName: Option<String>,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub deployerTradingFeeShare: f64,
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct AssetContext {
    pub asset: Asset,
    pub frozen: bool,
    pub openInterest: f64,
    pub midPrice: f64,
    pub markPrice: f64,
    pub fundingRate: f64,
    pub oraclePx: f64,
}

#[derive(Debug, Deserialize)]
pub struct UserState {
    pub withdrawable: f64,
    pub margin_summary: MarginSummary,
    pub time: u64,
    pub asset_positions: Vec<AssetPosition>,
}

#[derive(Debug, Deserialize)]
pub struct MarginSummary {
    pub account_value: f64,
    pub total_margin_used: f64,
    pub total_ntl: f64,
    pub total_margin_ratio: f64,
}

#[derive(Debug, Deserialize)]
pub struct AssetPosition {
    pub position: Position,
}

#[derive(Debug, Deserialize)]
pub struct Position {
    pub coin: String,
    pub szi: f64,
    pub entry_px: Option<f64>,
    pub unrealized_pnl: f64,
}

#[derive(Debug, Deserialize)]
pub struct OpenOrder {
    pub coin: String,
    pub side: String,
    pub sz: String,
    pub limit_px: String,
    pub reduce_only: bool,
    pub order_type: OrderType,
    pub oid: u64,
    pub timestamp: u64,
    pub cloid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit { tif: String },
    #[serde(rename = "trigger")]
    Trigger {
        is_market: bool,
        trigger_px: String,
        tpsl: String,
    },
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct UserFill {
    pub coin: String,
    pub px: f64,
    pub sz: f64,
    pub side: String,
    pub time: u64,
    pub startPosition: String,
    pub dir: String,
    pub closed_pnl: String,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    pub fee: String,
    pub tid: u64,
    pub fee_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UserFunding {
    pub coin: String,
    pub usdc: f64,
    pub time: u64,
}

#[derive(Debug, Deserialize)]
pub struct FundingHistory {
    pub coin: String,
    pub funding: f64,
    pub time: u64,
}

#[derive(Debug, Deserialize)]
pub struct L2Book {
    pub coin: String,
    pub levels: (Vec<PriceLevel>, Vec<PriceLevel>),
}

#[derive(Debug, Deserialize)]
pub struct PriceLevel {
    pub px: f64,
    pub sz: f64,
    pub n: Option<u32>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct UserPoints {
    pub user: String,
    pub name: Option<String>,
    pub eligible: bool,
    pub points: Vec<UserPoint>,
}

#[derive(Debug, Deserialize)]
pub struct UserPoint {
    pub name: String,
    pub score: f64,
}

#[derive(Debug, Deserialize)]
pub struct OrderStatusResponse {
    pub status: String,
    pub coin: String,
    pub side: String,
    pub sz: String,
    pub limit_px: String,
    pub reduce_only: bool,
    pub order_type: OrderType,
    pub timestamp: u64,
    pub oid: u64,
    pub cloid: Option<String>,
}
