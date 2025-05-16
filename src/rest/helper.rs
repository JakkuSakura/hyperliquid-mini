use crate::error::Result;
use crate::rest::models::API;
use serde::Serialize;
use serde::de::DeserializeOwned;
use static_assertions::assert_impl_all;

#[derive(Debug, Clone)]
pub struct HyperliquidRestClientHelper {
    host: String,
}

impl HyperliquidRestClientHelper {
    pub fn new(host: String) -> Self {
        Self { host }
    }
}

impl HyperliquidRestClientHelper {
    pub fn build_request(&self, endpoint: API, req: impl Serialize) -> reqwest::Request {
        todo!()
    }
    pub async fn post<T: DeserializeOwned>(&self, endpoint: API, req: impl Serialize) -> Result<T> {
        todo!()
    }
}

assert_impl_all!(HyperliquidRestClientHelper: Send, Sync, Unpin);
