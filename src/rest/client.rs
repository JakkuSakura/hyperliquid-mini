use crate::execution::models::HyperliquidChain;
use crate::rest::info::HyperliquidInfoClient;

pub struct HyperliquidRestClient {
    pub info: HyperliquidInfoClient,
}

impl HyperliquidRestClient {
    pub fn new_with_chain(chain: HyperliquidChain) -> Self {
        Self {
            info: HyperliquidInfoClient::new(chain),
        }
    }
}
