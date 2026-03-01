use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported blockchain networks for bridging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainInfo {
    pub id: u64,
    pub name: String,
    pub symbol: String,
    pub logo_url: String,
    pub explorer: String,
    pub rpc_url: String,
    pub is_testnet: bool,
}

/// Bridge quote response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeQuote {
    pub from_chain: ChainInfo,
    pub to_chain: ChainInfo,
    pub from_token: TokenInfo,
    pub to_token: TokenInfo,
    pub from_amount: String,
    pub to_amount: String,
    pub exchange_rate: String,
    pub estimated_time: String,
    pub estimated_gas: String,
    pub bridge_fee: String,
    pub protocol: String,
    pub route: Vec<BridgeStep>,
}

/// Bridge step information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeStep {
    pub step_type: String,
    pub protocol: String,
    pub description: String,
}

/// Token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: Option<String>,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub logo_url: String,
    pub chain_id: u64,
}

/// Bridge transaction request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransactionRequest {
    pub from_chain_id: u64,
    pub to_chain_id: u64,
    pub from_token: String,
    pub to_token: String,
    pub amount: String,
    pub from_address: String,
    pub slippage: Option<f64>,
}

/// Bridge transaction response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransaction {
    pub quote: BridgeQuote,
    pub tx_data: TransactionData,
    pub approval_address: Option<String>,
}

/// Transaction data for bridging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub to: String,
    pub data: String,
    pub value: String,
    pub gas_limit: String,
    pub gas_price: Option<String>,
}

/// Bridge history record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeHistory {
    pub id: String,
    pub from_chain: u64,
    pub to_chain: u64,
    pub from_token: String,
    pub to_token: String,
    pub from_amount: String,
    pub to_amount: String,
    pub status: String,
    pub tx_hash: Option<String>,
    pubbridge_tx_hash: Option<String>,
    pub timestamp: i64,
}

/// Supported bridges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeProtocol {
    pub id: String,
    pub name: String,
    pub logo_url: String,
    pub chains: Vec<u64>,
    pub supported_tokens: Vec<String>,
}

/// Get supported chains
pub fn get_supported_chains() -> Vec<ChainInfo> {
    vec![
        ChainInfo {
            id: 1,
            name: "Ethereum".to_string(),
            symbol: "ETH".to_string(),
            logo_url: "https://assets.coingecko.com/coins/images/279/small/ethereum-logo.png".to_string(),
            explorer: "https://etherscan.io".to_string(),
            rpc_url: "".to_string(),
            is_testnet: false,
        },
        ChainInfo {
            id: 56,
            name: "BNB Chain".to_string(),
            symbol: "BNB".to_string(),
            logo_url: "https://assets.coingecko.com/coins/images/825/small/bnb-icon2_2x.png".to_string(),
            explorer: "https://bscscan.com".to_string(),
            rpc_url: "".to_string(),
            is_testnet: false,
        },
        ChainInfo {
            id: 137,
            name: "Polygon".to_string(),
            symbol: "MATIC".to_string(),
            logo_url: "https://assets.coingecko.com/coins/images/4713/small/matic-token-icon.png".to_string(),
            explorer: "https://polygonscan.com".to_string(),
            rpc_url: "".to_string(),
            is_testnet: false,
        },
        ChainInfo {
            id: 42161,
            name: "Arbitrum".to_string(),
            symbol: "ETH".to_string(),
            logo_url: "https://assets.coingecko.com/chains/images/16547/small/photo_2023-03-29_21.47.00.jpeg".to_string(),
            explorer: "https://arbiscan.io".to_string(),
            rpc_url: "".to_string(),
            is_testnet: false,
        },
        ChainInfo {
            id: 10,
            name: "Optimism".to_string(),
            symbol: "ETH".to_string(),
            logo_url: "https://assets.coingecko.com/chains/images/25277/small/Optimism.png".to_string(),
            explorer: "https://optimistic.etherscan.io".to_string(),
            rpc_url: "".to_string(),
            is_testnet: false,
        },
        ChainInfo {
            id: 8453,
            name: "Base".to_string(),
            symbol: "ETH".to_string(),
            logo_url: "https://assets.coingecko.com/chains/images/31075/small/base-logo.png".to_string(),
            explorer: "https://basescan.org".to_string(),
            rpc_url: "".to_string(),
            is_testnet: false,
        },
        ChainInfo {
            id: 43114,
            name: "Avalanche".to_string(),
            symbol: "AVAX".to_string(),
            logo_url: "https://assets.coingecko.com/coins/images/12559/small/Avalanche_Circle_RedWhite_Trans.png".to_string(),
            explorer: "https://snowtrace.io".to_string(),
            rpc_url: "".to_string(),
            is_testnet: false,
        },
    ]
}

/// Get bridge tokens for a specific chain
pub fn get_bridge_tokens(chain_id: u64) -> Vec<TokenInfo> {
    let native_tokens: HashMap<u64, TokenInfo> = [
        (1, TokenInfo {
            address: None,
            symbol: "ETH".to_string(),
            name: "Ethereum".to_string(),
            decimals: 18,
            logo_url: "https://assets.coingecko.com/coins/images/279/small/ethereum-logo.png".to_string(),
            chain_id: 1,
        }),
        (56, TokenInfo {
            address: None,
            symbol: "BNB".to_string(),
            name: "BNB".to_string(),
            decimals: 18,
            logo_url: "https://assets.coingecko.com/coins/images/825/small/bnb-icon2_2x.png".to_string(),
            chain_id: 56,
        }),
        (137, TokenInfo {
            address: None,
            symbol: "MATIC".to_string(),
            name: "Polygon".to_string(),
            decimals: 18,
            logo_url: "https://assets.coingecko.com/coins/images/4713/small/matic-token-icon.png".to_string(),
            chain_id: 137,
        }),
        (42161, TokenInfo {
            address: None,
            symbol: "ETH".to_string(),
            name: "Ethereum".to_string(),
            decimals: 18,
            logo_url: "https://assets.coingecko.com/coins/images/279/small/ethereum-logo.png".to_string(),
            chain_id: 42161,
        }),
        (10, TokenInfo {
            address: None,
            symbol: "ETH".to_string(),
            name: "Ethereum".to_string(),
            decimals: 18,
            logo_url: "https://assets.coingecko.com/coins/images/279/small/ethereum-logo.png".to_string(),
            chain_id: 10,
        }),
        (8453, TokenInfo {
            address: None,
            symbol: "ETH".to_string(),
            name: "Ethereum".to_string(),
            decimals: 18,
            logo_url: "https://assets.coingecko.com/coins/images/279/small/ethereum-logo.png".to_string(),
            chain_id: 8453,
        }),
        (43114, TokenInfo {
            address: None,
            symbol: "AVAX".to_string(),
            name: "Avalanche".to_string(),
            decimals: 18,
            logo_url: "https://assets.coingecko.com/coins/images/12559/small/Avalanche_Circle_RedWhite_Trans.png".to_string(),
            chain_id: 43114,
        }),
    ].into_iter().collect();

    // Common ERC20 tokens across chains
    let common_tokens = vec![
        ("USDC", "USD Coin", 6, "https://assets.coingecko.com/coins/images/6319/small/USD_Coin_icon.png"),
        ("USDT", "Tether", 6, "https://assets.coingecko.com/coins/images/325/small/Tether.png"),
        ("WBTC", "Wrapped Bitcoin", 8, "https://assets.coingecko.com/coins/images/7598/small/wrapped_bitcoin_wbtc.png"),
        ("DAI", "Dai", 18, "https://assets.coingecko.com/coins/images/9956/small/4943.png"),
    ];

    let mut tokens = Vec::new();
    
    // Add native token
    if let Some(native) = native_tokens.get(&chain_id).cloned() {
        tokens.push(native);
    }
    
    // Add common tokens
    for (symbol, name, decimals, logo) in common_tokens {
        tokens.push(TokenInfo {
            address: Some(format!("0x0000000000000000000000000000000000000000")),
            symbol: symbol.to_string(),
            name: name.to_string(),
            decimals,
            logo_url: logo.to_string(),
            chain_id,
        });
    }
    
    tokens
}

/// Get supported bridge protocols
pub fn get_bridge_protocols() -> Vec<BridgeProtocol> {
    vec![
        BridgeProtocol {
            id: "layerzero".to_string(),
            name: "LayerZero".to_string(),
            logo_url: "https://assets.coingecko.com/chains/images/24288/small/LayerZero_Logos_2023_Blacks_BG.png".to_string(),
            chains: vec![1, 56, 137, 42161, 10, 8453, 43114],
            supported_tokens: vec!["ETH".to_string(), "USDC".to_string(), "USDT".to_string()],
        },
        BridgeProtocol {
            id: "axelar".to_string(),
            name: "Axelar".to_string(),
            logo_url: "https://assets.coingecko.com/coins/images/19783/small/axelar.png".to_string(),
            chains: vec![1, 56, 137, 42161, 10, 43114],
            supported_tokens: vec!["ETH".to_string(), "USDC".to_string(), "USDT".to_string(), "DAI".to_string()],
        },
        BridgeProtocol {
            id: "wormhole".to_string(),
            name: "Wormhole".to_string(),
            logo_url: "https://assets.coingecko.com/coins/images/35087/small/womrhole_logo_full_color_rgb_2000px_72ppi_fb766ac85a.png".to_string(),
            chains: vec![1, 56, 137, 42161, 10, 8453, 43114],
            supported_tokens: vec!["ETH".to_string(), "USDC".to_string(), "USDT".to_string(), "WBTC".to_string()],
        },
        BridgeProtocol {
            id: "stargate".to_string(),
            name: "Stargate".to_string(),
            logo_url: "https://assets.coingecko.com/coins/images/25413/small/stargate_logo_no_text.png".to_string(),
            chains: vec![1, 56, 137, 42161, 10],
            supported_tokens: vec!["ETH".to_string(), "USDC".to_string(), "USDT".to_string()],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_supported_chains() {
        let chains = get_supported_chains();
        assert!(!chains.is_empty());
        assert!(chains.iter().any(|c| c.id == 1));
    }

    #[test]
    fn test_get_bridge_tokens() {
        let tokens = get_bridge_tokens(1);
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_get_bridge_protocols() {
        let protocols = get_bridge_protocols();
        assert!(!protocols.is_empty());
    }
}
