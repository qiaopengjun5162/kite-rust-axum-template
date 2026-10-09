/// Kite chain configuration.
///
/// Defines the two Kite networks:
/// - **mainnet** (eip155:2366, USDC.e)
/// - **testnet** (eip155:2368, pieUSD)
use x402_chain_eip155::{KnownNetworkEip155, V1Eip155Exact};
use x402_types::chain::ChainId;
use x402_types::networks::USDC;

/// Kite network configuration.
#[derive(Debug, Clone)]
pub struct KiteChain {
    /// Human-readable name (`mainnet` or `testnet`).
    pub name: String,
    /// CAIP-2 chain identifier.
    #[expect(dead_code)]
    pub chain_id: ChainId,
    /// Stablecoin symbol.
    pub asset_symbol: String,
    /// Chain identifier string (used in x402 price tags).
    pub network: String,
    /// Default facilitator URL for this network.
    pub default_facilitator_url: String,
}

impl KiteChain {
    /// Create a x402 V1 price tag for the given USD price on this network.
    pub fn price_tag(&self, pay_to: &str, usd: &str) -> x402_types::proto::v1::PriceTag {
        use std::str::FromStr;
        let addr =
            x402_chain_eip155::chain::types::ChecksummedAddress::from_str(pay_to).expect("invalid pay_to address");
        V1Eip155Exact::price_tag(
            addr,
            match self.name.as_str() {
                "testnet" => USDC::base_sepolia().parse(usd).expect("invalid price"),
                _ => USDC::base().parse(usd).expect("invalid price"),
            },
        )
    }
}

/// Look up a KiteChain by name.
pub fn kite_chain_by_name(name: &str) -> KiteChain {
    match name.trim().to_lowercase().as_str() {
        "testnet" => KiteChain {
            name: "testnet".into(),
            chain_id: ChainId::new("eip155", "2368"),
            asset_symbol: "pieUSD".into(),
            network: "eip155:2368".into(),
            default_facilitator_url: "https://facilitator.pieverse.io/v2".into(),
        },
        _ => KiteChain {
            name: "mainnet".into(),
            chain_id: ChainId::new("eip155", "2366"),
            asset_symbol: "USDC.e".into(),
            network: "eip155:2366".into(),
            default_facilitator_url: "https://facilitator.pieverse.io/v2".into(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kite_chain_mainnet() {
        let chain = kite_chain_by_name("mainnet");
        assert_eq!(chain.network, "eip155:2366");
        assert_eq!(chain.asset_symbol, "USDC.e");
        assert_eq!(chain.name, "mainnet");
    }

    #[test]
    fn test_kite_chain_testnet() {
        let chain = kite_chain_by_name("testnet");
        assert_eq!(chain.network, "eip155:2368");
        assert_eq!(chain.asset_symbol, "pieUSD");
        assert_eq!(chain.name, "testnet");
    }

    #[test]
    fn test_price_tag_creation() {
        let chain = kite_chain_by_name("mainnet");
        let tag = chain.price_tag("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", "0.001");
        let _ = tag;
    }

    #[test]
    fn test_kite_chain_defaults_to_mainnet() {
        let chain = kite_chain_by_name("unknown");
        assert_eq!(chain.name, "mainnet");
    }
}
