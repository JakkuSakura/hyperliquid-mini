#[cfg(test)]
mod tests {
    use crate::execution::models::agent::agent_sol;
    use crate::execution::models::{
        Action, CancelRequest, Grouping, HyperliquidChain, HyperliquidOrderRequest,
        HyperliquidOrderType, HyperliquidTif, RequestCancelByClientId, TpSl,
    };
    use crate::sign::tests::get_wallet;
    use crate::sign::{HyperliquidSignature, L1_DOMAIN, sign_l1_action};
    use crate::utils::uuid_to_hex_string;
    use alloy::dyn_abi::Eip712Domain;
    use alloy::primitives::{Address, B256, keccak256};
    use alloy::signers::Signer;
    use alloy::sol_types::{SolStruct, SolValue};
    use malachite::base::strings::{ToDebugString, ToLowerHexString};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_limit_order_action_hashing() -> eyre::Result<()> {
        let wallet = get_wallet();
        let action = Action::Order {
            orders: vec![HyperliquidOrderRequest {
                asset: 1,
                is_buy: true,
                limit_px: "2000.0".to_string(),
                sz: "3.5".to_string(),
                reduce_only: false,
                order_type: HyperliquidOrderType::Limit {
                    tif: HyperliquidTif::Ioc,
                },
                cloid: None,
            }],
            grouping: Grouping::Na,
        };
        assert_eq!(
            serde_json::to_string(&action)?,
            r#"{"type":"order","orders":[{"a":1,"b":true,"p":"2000.0","s":"3.5","r":false,"t":{"limit":{"tif":"Ioc"}}}],"grouping":"na"}"#
        );
        let connection_id = action.hash(1583838, Address::new([0; 20]))?;
        assert_eq!(
            connection_id.to_debug_string(),
            "0x5983a9453b8d32668daefa9310e1a81bc1f4d7da50a9ad8869a4011d12068ea0"
        );

        let signature = sign_l1_action(HyperliquidChain::Arbitrum, &wallet, connection_id).await?;
        assert_eq!(
            hex::encode(signature.as_primitive_signature().as_bytes()),
            "77957e58e70f43b6b68581f2dc42011fc384538a2e5b7bf42d5b936f19fbb67360721a8598727230f67080efee48c812a6a4442013fd3b0eed509171bef9f23f1c"
        );

        let signature =
            sign_l1_action(HyperliquidChain::ArbitrumGoerli, &wallet, connection_id).await?;
        assert_eq!(
            hex::encode(signature.as_primitive_signature().as_bytes()),
            "cd0925372ff1ed499e54883e9a6205ecfadec748f80ec463fe2f84f1209648776377961965cb7b12414186b1ea291e95fd512722427efcbcfb3b0b2bcd4d79d01c"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_limit_order_action_hashing_with_cloid() -> eyre::Result<()> {
        let cloid = uuid::Uuid::from_str("1e60610f-0b3d-4205-97c8-8c1fed2ad5ee")?;
        let wallet = get_wallet();
        let action = Action::Order {
            orders: vec![HyperliquidOrderRequest {
                asset: 1,
                is_buy: true,
                limit_px: "2000.0".to_string(),
                sz: "3.5".to_string(),
                reduce_only: false,
                order_type: HyperliquidOrderType::Limit {
                    tif: HyperliquidTif::Ioc,
                },
                cloid: Some(uuid_to_hex_string(cloid)),
            }],
            grouping: Grouping::Na,
        };
        let connection_id = action.hash(1583838, Address::new([0; 20]))?;

        let signature = sign_l1_action(HyperliquidChain::Arbitrum, &wallet, connection_id).await?;
        assert_eq!(
            signature.to_string(),
            "d3e894092eb27098077145714630a77bbe3836120ee29df7d935d8510b03a08f456de5ec1be82aa65fc6ecda9ef928b0445e212517a98858cfaa251c4cd7552b1c"
        );

        let signature =
            sign_l1_action(HyperliquidChain::ArbitrumGoerli, &wallet, connection_id).await?;
        assert_eq!(
            signature.to_string(),
            "3768349dbb22a7fd770fc9fc50c7b5124a7da342ea579b309f58002ceae49b4357badc7909770919c45d850aabb08474ff2b7b3204ae5b66d9f7375582981f111c"
        );

        Ok(())
    }
    #[tokio::test]
    async fn test_cancel_order_by_oid_conn_id() -> eyre::Result<()> {
        let action = Action::Cancel {
            cancels: vec![CancelRequest { asset: 0, oid: 123 }],
        };
        assert_eq!(
            serde_json::to_string(&action)?,
            r#"{"type":"cancel","cancels":[{"a":0,"o":123}]}"#
        );
        let connection_id = action.hash(1741110304133, Address::new([0; 20]))?;
        assert_eq!(
            connection_id.to_debug_string(),
            "0xe9972af71a50d95fee642a02c87ba87d7c10ceb0e6cc56dec09739f187d7053c"
        );
        Ok(())
    }
    #[tokio::test]
    async fn test_cancel_order_by_cloid_conn_id() -> eyre::Result<()> {
        let action = Action::CancelByCloid {
            cancels: vec![RequestCancelByClientId {
                asset: 0,
                cloid: "0x9c09a42dede9495ea86bb4bc3888cc2d".to_string(),
            }],
        };
        assert_eq!(
            serde_json::to_string(&action)?,
            r#"{"type":"cancelByCloid","cancels":[{"asset":0,"cloid":"0x9c09a42dede9495ea86bb4bc3888cc2d"}]}"#
        );
        let connection_id = action.hash(1741110304133, Address::new([0; 20]))?;
        assert_eq!(
            connection_id.to_debug_string(),
            "0xb58a154f39d58a2c45cc8444ab7be455c6e4864990ba995c882c950e844f2d43"
        );
        Ok(())
    }
    #[tokio::test]
    async fn test_sign_actual_order_trigger() -> eyre::Result<()> {
        // {"action":{"type":"order","orders":[{"a":13,"b":true,"p":"16","s":"0.5","r":false,"t":{"trigger":{"isMarket":false,"tpsl":"sl","triggerPx":"16"}},"c":"0x172f684508034185932d9165bca4bc0b"}],"grouping":"na"},"nonce":1741110304133,"signature":{"r":"0xa105bcd3054eb61c0e8c8b26601133568d441e2891737cf30ed1547b3c99fe18","s":"0x1ea54db7953408299f595ca7e4963f57b53aa8b43d095cc8a67b1eead7ab48a","v":27}}
        let wallet = get_wallet();
        let action = Action::Order {
            orders: vec![HyperliquidOrderRequest {
                asset: 13,
                is_buy: true,
                limit_px: "16".to_string(),
                sz: "0.5".to_string(),
                reduce_only: false,
                order_type: HyperliquidOrderType::Trigger {
                    is_market: false,
                    tpsl: TpSl::Sl,
                    trigger_px: "16".to_string(),
                },
                cloid: Some("0x172f684508034185932d9165bca4bc0b".to_string()),
            }],
            grouping: Grouping::Na,
        };
        let connection_id = action.hash(1741110304133, Address::new([0; 20]))?;
        assert_eq!(
            connection_id.to_debug_string(),
            "0x01ad643448b644430978c220e87e6bfcf45232f4d18517e10b7f3affe5d778ce"
        );
        let signature = sign_l1_action(HyperliquidChain::Arbitrum, &wallet, connection_id).await?;
        assert_eq!(
            signature.r.to_lower_hex_string(),
            "fa5732a68b2a7a5e2b1db5c886dae83825a29cedcbe92ae836bd1d157504e3bb"
        );
        assert_eq!(
            signature.s.to_lower_hex_string(),
            "1d60facc47cc071c1687c3a0f5e5a8c728fbf197d83871b6c4af918b6c64714"
        );
        assert_eq!(signature.v, 28);

        Ok(())
    }

    #[tokio::test]
    async fn test_sign_actual_order_limit() -> eyre::Result<()> {
        //   {"action":{"type":"order","orders":[{"a":1,"b":true,"p":"2700","s":"0.0031","r":false,"t":{"limit":{"tif":"Gtc"}},"c":"0x9c09a42dede9495ea86bb4bc3888cc2d"}],"grouping":"na"},"nonce":1741146714088,"signature":{"r":"0x748bad003987a6f5b77e4d288420511ba7bdd3217e6ed3a625df7385d583acee","s":"0x11f817ae197a88148eb76b60626ca01523f1aaa39ef59629b162f570abb68045","v":28}}
        let wallet = get_wallet();
        let action = Action::Order {
            orders: vec![HyperliquidOrderRequest {
                asset: 1,
                is_buy: true,
                limit_px: "2700".to_string(),
                sz: "0.0031".to_string(),
                reduce_only: false,
                order_type: HyperliquidOrderType::Limit {
                    tif: HyperliquidTif::Gtc,
                },
                cloid: Some("0x9c09a42dede9495ea86bb4bc3888cc2d".to_string()),
            }],
            grouping: Grouping::Na,
        };
        let connection_id = action.hash(1741146714088, Address::new([0; 20]))?;
        let signature = sign_l1_action(HyperliquidChain::Arbitrum, &wallet, connection_id).await?;
        assert_eq!(
            signature.r.to_lower_hex_string(),
            "748bad003987a6f5b77e4d288420511ba7bdd3217e6ed3a625df7385d583acee"
        );
        assert_eq!(
            signature.s.to_lower_hex_string(),
            "11f817ae197a88148eb76b60626ca01523f1aaa39ef59629b162f570abb68045"
        );
        assert_eq!(signature.v, 28);
        Ok(())
    }

    #[tokio::test]
    async fn test_sign_actual_order_limit_manual() -> eyre::Result<()> {
        //   {"action":{"type":"order","orders":[{"a":1,"b":true,"p":"2700","s":"0.0031","r":false,"t":{"limit":{"tif":"Gtc"}},"c":"0x9c09a42dede9495ea86bb4bc3888cc2d"}],"grouping":"na"},"nonce":1741146714088,"signature":{"r":"0x748bad003987a6f5b77e4d288420511ba7bdd3217e6ed3a625df7385d583acee","s":"0x11f817ae197a88148eb76b60626ca01523f1aaa39ef59629b162f570abb68045","v":28}}
        let wallet = get_wallet();
        let action = Action::Order {
            orders: vec![HyperliquidOrderRequest {
                asset: 1,
                is_buy: true,
                limit_px: "2700".to_string(),
                sz: "0.0031".to_string(),
                reduce_only: false,
                order_type: HyperliquidOrderType::Limit {
                    tif: HyperliquidTif::Gtc,
                },
                cloid: Some("0x9c09a42dede9495ea86bb4bc3888cc2d".to_string()),
                // expire_at: Some(1672531199),
            }],
            grouping: Grouping::Na,
        };
        let connection_id = action.hash(1741146714088, Address::new([0; 20]))?;
        let payload = agent_sol::Agent {
            source: "a".to_string(),
            connectionId: connection_id,
        };
        fn eip712_hash_struct(this: &agent_sol::Agent) -> B256 {
            let mut data = vec![];
            data.extend_from_slice(&this.eip712_type_hash()[..]);
            data.extend_from_slice(&this.source.eip712_data_word().0);
            data.extend_from_slice(&this.connectionId.eip712_data_word().0);
            // println!("data_hash_input: {}", hex::encode(&data));
            keccak256(&data)
        }
        fn eip712_signing_hash(this: &agent_sol::Agent, domain: &Eip712Domain) -> B256 {
            let mut digest_input = [0u8; 2 + 32 + 32];
            digest_input[0] = 0x19;
            digest_input[1] = 0x01;
            digest_input[2..34].copy_from_slice(&domain.hash_struct()[..]);
            // println!("domain hash: {:?}", &digest_input[0..34]);
            let hash_struct = eip712_hash_struct(this);
            // println!("hash_struct: {}", hex::encode(&hash_struct[..]));
            digest_input[34..66].copy_from_slice(&hash_struct.0);
            // println!("digest_input: {}", hex::encode(&digest_input[..]));
            keccak256(digest_input)
        }
        let signing_hash = eip712_signing_hash(&payload, L1_DOMAIN);
        assert_eq!(
            signing_hash.to_lower_hex_string(),
            "57895f56942aa15ae2a80da640ba59a97762b64ad91fa68dbcc5d06fa6534bc5"
        );
        let sig: HyperliquidSignature = wallet.sign_hash(&signing_hash).await?.into();
        assert_eq!(
            sig.r.to_lower_hex_string(),
            "748bad003987a6f5b77e4d288420511ba7bdd3217e6ed3a625df7385d583acee"
        );
        assert_eq!(
            sig.s.to_lower_hex_string(),
            "11f817ae197a88148eb76b60626ca01523f1aaa39ef59629b162f570abb68045"
        );
        assert_eq!(sig.v, 28);

        Ok(())
    }
}
