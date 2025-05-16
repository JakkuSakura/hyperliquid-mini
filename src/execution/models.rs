// Execution-related model definitions
use alloy::primitives::{Address, B256, keccak256};
use eyre::Result;
use serde::{Deserialize, Serialize};

// Re-export from the original agent module
pub mod agent {
    use alloy::primitives::B256;
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Agent {
        pub source: String,
        pub connection_id: B256,
    }

    pub mod agent_sol {
        use alloy::core::sol;

        sol! {
            struct Agent {
                string source;
                bytes32 connectionId;
            }
        }
    }
}

// Chain and request definitions
#[derive(Clone, Copy, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum HyperliquidChain {
    Dev,
    Arbitrum,
    ArbitrumGoerli,
}

// Order and execution types
#[derive(Serialize, Debug, Clone)]
pub enum HyperliquidTif {
    Gtc,
    Ioc,
    Alo,
    FrontendMarket = 8,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TpSl {
    Tp,
    Sl,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum HyperliquidOrderType {
    Limit {
        tif: HyperliquidTif,
    },
    #[serde(rename_all = "camelCase")]
    Trigger {
        is_market: bool,
        trigger_px: String,
        tpsl: TpSl,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidOrderRequest {
    #[serde(rename = "a", alias = "asset")]
    pub asset: u32,
    #[serde(rename = "b", alias = "isBuy")]
    pub is_buy: bool,
    #[serde(rename = "p", alias = "limitPx")]
    pub limit_px: String,
    #[serde(rename = "s", alias = "sz")]
    pub sz: String,
    #[serde(rename = "r", alias = "reduceOnly", default)]
    pub reduce_only: bool,
    #[serde(rename = "t", alias = "orderType")]
    pub order_type: HyperliquidOrderType,
    #[serde(rename = "c", alias = "cloid", skip_serializing_if = "Option::is_none")]
    pub cloid: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Grouping {
    Na,
}

impl Grouping {
    pub fn to_i32(&self) -> i32 {
        match self {
            Grouping::Na => 0,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelRequest {
    #[serde(rename = "a", alias = "asset")]
    pub asset: u32,
    #[serde(rename = "o", alias = "oid")]
    pub oid: u64,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestCancelByClientId {
    pub asset: u32,
    pub cloid: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

pub mod exchange_sol {
    use alloy::core::sol;
    sol! {
        struct UsdTransferSignPayload {
            string destination;
            string amount;
            uint64 time;
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Action {
    Order {
        orders: Vec<HyperliquidOrderRequest>,
        grouping: Grouping,
    },
    Cancel {
        cancels: Vec<CancelRequest>,
    },
    CancelByCloid {
        cancels: Vec<RequestCancelByClientId>,
    },
    UsdTransfer {
        chain: HyperliquidChain,
        payload: TransferRequest,
    },
    Withdraw {
        usd: String,
        nonce: u64,
    },
    #[serde(rename_all = "camelCase")]
    UpdateLeverage {
        asset: u32,
        is_cross: bool,
        leverage: u32,
    },
    #[serde(rename_all = "camelCase")]
    UpdateIsolatedMargin {
        asset: u32,
        is_buy: bool,
        ntli: i64,
    },
    #[serde(rename_all = "camelCase", rename = "connect")]
    ApproveAgent {
        chain: HyperliquidChain,
        agent: agent::Agent,
        agent_address: Address,
    },
    // But it belongs to info
    UserPoints {
        user: Address,
    },
}

impl Action {
    pub fn hash(&self, timestamp: u64, vault_address: Address) -> Result<B256> {
        // MsgPack
        let mut bytes = rmp_serde::to_vec_named(self)?;
        bytes.extend(timestamp.to_be_bytes());
        if !vault_address.is_zero() {
            bytes.push(1);
            bytes.extend(vault_address);
        } else {
            bytes.push(0);
        }

        Ok(keccak256(bytes))
    }
}

// WebSocket models for execution-related events
#[derive(Debug, Deserialize)]
#[serde(tag = "channel", content = "data")]
pub enum WsExecutionResponse {
    #[serde(rename = "orderUpdates")]
    OrderUpdates(Vec<WsOrderUpdate>),
    #[serde(rename = "user")]
    User(WsUserEvent),
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "notification")]
    Notification(String),
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct WsOrderUpdate {
    pub status: String,
    pub status_timestamp: u64,
    pub order: WsOrder,
}

#[derive(Debug, Deserialize)]
pub struct WsOrder {
    pub coin: String,
    pub side: String,
    #[serde(default)]
    pub sz: f64,
    #[serde(default)]
    pub orig_sz: f64,
    pub limit_px: Option<String>,
    pub oid: String,
    pub timestamp: u64,
    pub cloid: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WsUserEvent {
    #[serde(rename = "fills")]
    Fills { fills: Vec<WsUserFill> },
    #[serde(rename = "funding")]
    Funding { funding: WsUserFunding },
    #[serde(rename = "liquidation")]
    Liquidation { liquidation: WsUserLiquidation },
    #[serde(rename = "nonUserCancel")]
    NonUserCancel { nonUserCancel: WsUserNonUserCancel },
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
pub struct WsUserFill {
    pub coin: String,
    pub px: f64,
    pub sz: f64,
    pub side: String,
    pub time: u64,
    pub start_position: String,
    pub dir: String,
    pub closed_pnl: String,
    pub hash: String,
    pub oid: u64,
    pub crossed: bool,
    pub fee: String,
    pub tid: u64,
    pub cloid: String,
    pub fee_token: String,
}

impl WsUserFill {
    pub fn dir(&self) -> (String, String) {
        let parts: Vec<&str> = self.dir.split(' ').collect();
        if parts.len() == 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            (self.dir.clone(), "".to_string())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WsUserFunding {
    pub coin: String,
    pub usdc: f64,
    pub time: u64,
}

#[derive(Debug, Deserialize)]
pub struct WsUserLiquidation {
    pub time: u64,
}

#[derive(Debug, Deserialize)]
pub struct WsUserNonUserCancel {
    pub coin: String,
    pub oid: u64,
    pub reason: String,
    pub time: u64,
}

// Request structs
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HyperliquidRequest {
    pub action: Action,
    pub nonce: u64,
    pub signature: crate::sign::HyperliquidSignature,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_address: Option<Address>,
}

#[derive(Debug, Serialize)]
pub struct HyperliquidRequestUserPoints {
    pub signature: crate::sign::HyperliquidSignature,
    pub timestamp: u64,
    #[serde(flatten)]
    pub action: Action, // UserPoints only
}

// WebSocket Request for Execution
#[derive(Debug, Serialize)]
pub struct WsRequest {
    pub method: Method,
    #[serde(flatten)]
    pub subscription: Subscription,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Method {
    Subscribe,
    Unsubscribe,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "channel")]
pub enum Subscription {
    #[serde(rename = "orderUpdates")]
    OrderUpdates { user: Address },
    #[serde(rename = "user")]
    User { user: Address },
}

// Response models
#[derive(Debug, Deserialize)]
pub enum Response {
    Ok(OkResponse),
    Err(String),
}

#[derive(Debug, Deserialize)]
pub struct OkResponse {
    pub status: String,
    pub data: Option<StatusData>,
}

#[derive(Debug, Deserialize)]
pub struct StatusData {
    pub statuses: Vec<Status>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Status {
    #[serde(rename = "filled")]
    Filled(Filled),
    #[serde(rename = "resting")]
    Resting(Resting),
    #[serde(rename = "error")]
    Error(String),
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "triggered")]
    Triggered,
    #[serde(rename = "marginCanceled")]
    MarginCanceled,
    #[serde(rename = "liquidation")]
    Liquidation,
    #[serde(rename = "waitingForFill")]
    WaitingForFill,
    #[serde(rename = "waitingForTrigger")]
    WaitingForTrigger,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Filled {
    pub oid: u64,
    pub total_sz: String,
    pub avg_px: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Resting {
    pub oid: u64,
}
