use crate::execution::models::HyperliquidChain;
use crate::execution::models::agent::agent_sol;
use alloy::dyn_abi::Eip712Domain;
use alloy::primitives::{B256, PrimitiveSignature, U256, address};
use alloy::signers::Signer;
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::eip712_domain;
use serde::Serialize;
use std::fmt::{Debug, Formatter};

pub const MAINNET_DOMAIN: &Eip712Domain = &eip712_domain! {
    name: "Exchange",
    version: "1",
    chain_id: 42161,
    verifying_contract: address!("0x0000000000000000000000000000000000000000") ,
};
pub const TESTNET_DOMAIN: &Eip712Domain = &eip712_domain! {
    name: "Exchange",
    version: "1",
    chain_id: 421613,
    verifying_contract: address!("0x0000000000000000000000000000000000000000") ,
};

pub const L1_DOMAIN: &Eip712Domain = &eip712_domain! {
    name: "Exchange",
    version: "1",
    chain_id: 1337,
    verifying_contract: address!("0x0000000000000000000000000000000000000000") ,
};
pub const fn get_domain(chain: HyperliquidChain) -> &'static Eip712Domain {
    match chain {
        HyperliquidChain::Arbitrum => MAINNET_DOMAIN,
        HyperliquidChain::ArbitrumGoerli => TESTNET_DOMAIN,
        HyperliquidChain::Dev => L1_DOMAIN,
    }
}
#[derive(Serialize)]
pub struct HyperliquidSignature {
    pub r: U256,
    pub s: U256,
    pub v: u8,
}
impl Debug for HyperliquidSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HyperliquidSignature")
            .field("r", &hex::encode(&self.r.to_be_bytes::<32>()))
            .field("s", &hex::encode(&self.s.to_be_bytes::<32>()))
            .field("v", &self.v)
            .finish()
    }
}
#[allow(dead_code)]
impl HyperliquidSignature {
    pub fn to_string(&self) -> String {
        hex::encode(self.as_primitive_signature().as_bytes())
    }
    pub fn as_primitive_signature(&self) -> PrimitiveSignature {
        PrimitiveSignature::new(self.r, self.s, self.v != 27)
    }
}
impl From<PrimitiveSignature> for HyperliquidSignature {
    fn from(sig: PrimitiveSignature) -> Self {
        Self {
            r: sig.r(),
            s: sig.s(),
            v: 27 + (sig.v() as u8),
        }
    }
}

/// Create a signature for the given connection id
pub async fn sign_l1_action(
    chain: HyperliquidChain,
    wallet: &PrivateKeySigner,
    connection_id: B256,
) -> crate::error::Result<HyperliquidSignature> {
    // This is weird, but it's running ok
    let (chain, source) = match chain {
        HyperliquidChain::Arbitrum => (HyperliquidChain::Dev, "a".to_string()),
        HyperliquidChain::Dev | HyperliquidChain::ArbitrumGoerli => {
            (HyperliquidChain::Dev, "b".to_string())
        }
    };
    sign_l1_action_inner(chain, source, wallet, connection_id).await
}
pub async fn sign_l1_action_inner(
    chain: HyperliquidChain,
    source: String,
    wallet: &PrivateKeySigner,
    connection_id: B256,
) -> crate::error::Result<HyperliquidSignature> {
    let payload = agent_sol::Agent {
        source,
        connectionId: connection_id,
    };

    let domain = get_domain(chain);
    // println!("type {}", solidity::Agent::eip712_encode_type(),);
    // println!("type hash {}", hex::encode(payload.eip712_type_hash()),);
    // println!("encode data {}", hex::encode(payload.eip712_encode_data()),);

    let sig = wallet.sign_typed_data(&payload, domain).await?;
    Ok(sig.into())
}

#[cfg(test)]
pub(crate) mod tests {
    use alloy::primitives::keccak256;
    use alloy::signers::local::PrivateKeySigner;

    const PRIVATE_KEY: &str = "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e";
    pub fn get_wallet() -> PrivateKeySigner {
        PRIVATE_KEY.parse::<PrivateKeySigner>().unwrap()
    }
    #[test]
    fn test_keccak256() {
        let data = "1234";
        let hash = keccak256(data);
        assert_eq!(
            hex::encode(hash),
            "387a8233c96e1fc0ad5e284353276177af2186e7afa85296f106336e376669f7"
        );
    }
}
