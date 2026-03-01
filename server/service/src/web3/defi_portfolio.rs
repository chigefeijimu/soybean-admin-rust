//! DeFi Portfolio Service
//! Tracks DeFi positions across multiple protocols

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DeFi position type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefiPosition {
    pub protocol: String,
    pub pool: String,
    pub token_a: String,
    pub token_b: String,
    pub value_usd: f64,
    pub apy: f64,
    pub tokens: Vec<TokenAmount>,
}

/// Token amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAmount {
    pub symbol: String,
    pub address: String,
    pub amount: f64,
    pub value_usd: f64,
}

/// Portfolio summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioSummary {
    pub total_value_usd: f64,
    pub total_apy: f64,
    pub positions: Vec<DefiPosition>,
    pub tokens: Vec<TokenHolding>,
}

/// Token holding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenHolding {
    pub symbol: String,
    pub address: String,
    pub amount: f64,
    pub value_usd: f64,
    pub protocol: String,
}

/// Protocol info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    pub name: String,
    pub logo: String,
    pub category: String,
    pub tvl_usd: f64,
    pub apy_range: (f64, f64),
}

/// DeFi Portfolio Service
pub struct DefiPortfolioService {
    #[allow(dead_code)]
    prices: HashMap<String, f64>,
}

impl DefiPortfolioService {
    pub fn new() -> Self {
        let mut prices = HashMap::new();
        // Mock prices
        prices.insert("ETH".to_string(), 2500.0);
        prices.insert("USDC".to_string(), 1.0);
        prices.insert("USDT".to_string(), 1.0);
        prices.insert("DAI".to_string(), 1.0);
        prices.insert("WBTC".to_string(), 62500.0);
        prices.insert("UNI".to_string(), 8.5);
        prices.insert("AAVE".to_string(), 85.0);
        prices.insert("LINK".to_string(), 15.0);
        Self { prices }
    }

    /// Get portfolio for an address
    pub fn get_portfolio(&self, _address: &str) -> PortfolioSummary {
        // Mock portfolio data
        let positions = vec![
            DefiPosition {
                protocol: "Uniswap V3".to_string(),
                pool: "ETH/USDC".to_string(),
                token_a: "ETH".to_string(),
                token_b: "USDC".to_string(),
                value_usd: 15000.0,
                apy: 12.5,
                tokens: vec![
                    TokenAmount {
                        symbol: "ETH".to_string(),
                        address: "0x0000000000000000000000000000000000000000".to_string(),
                        amount: 4.0,
                        value_usd: 10000.0,
                    },
                    TokenAmount {
                        symbol: "USDC".to_string(),
                        address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
                        amount: 5000.0,
                        value_usd: 5000.0,
                    },
                ],
            },
            DefiPosition {
                protocol: "Aave".to_string(),
                pool: "ETH".to_string(),
                token_a: "ETH".to_string(),
                token_b: "aETH".to_string(),
                value_usd: 8000.0,
                apy: 4.2,
                tokens: vec![TokenAmount {
                    symbol: "aETH".to_string(),
                    address: "0x4d5f542ea7c8c1e31c21e10d7c1c2e4ae5d5f7e".to_string(),
                    amount: 3.2,
                    value_usd: 8000.0,
                }],
            },
            DefiPosition {
                protocol: "Compound".to_string(),
                pool: "USDC".to_string(),
                token_a: "USDC".to_string(),
                token_b: "cUSDC".to_string(),
                value_usd: 5000.0,
                apy: 3.8,
                tokens: vec![TokenAmount {
                    symbol: "cUSDC".to_string(),
                    address: "0x39aa39c6df8c3aa2883a1ae4e4ea5b2d2c6e05e".to_string(),
                    amount: 5000.0,
                    value_usd: 5000.0,
                }],
            },
        ];

        let total_value: f64 = positions.iter().map(|p| p.value_usd).sum();
        let weighted_apy = positions.iter()
            .map(|p| p.apy * p.value_usd)
            .sum::<f64>() / total_value;

        // Flatten tokens
        let mut token_map: HashMap<String, TokenHolding> = HashMap::new();
        for pos in &positions {
            for token in &pos.tokens {
                let key = format!("{}_{}", token.symbol, pos.protocol);
                if let Some(existing) = token_map.get(&key) {
                    token_map.insert(key, TokenHolding {
                        symbol: token.symbol.clone(),
                        address: token.address.clone(),
                        amount: existing.amount + token.amount,
                        value_usd: existing.value_usd + token.value_usd,
                        protocol: pos.protocol.clone(),
                    });
                } else {
                    token_map.insert(key, TokenHolding {
                        symbol: token.symbol.clone(),
                        address: token.address.clone(),
                        amount: token.amount,
                        value_usd: token.value_usd,
                        protocol: pos.protocol.clone(),
                    });
                }
            }
        }

        PortfolioSummary {
            total_value_usd: total_value,
            total_apy: weighted_apy,
            positions,
            tokens: token_map.into_values().collect(),
        }
    }

    /// Get supported protocols
    pub fn get_protocols(&self) -> Vec<ProtocolInfo> {
        vec![
            ProtocolInfo {
                name: "Uniswap".to_string(),
                logo: "🦄".to_string(),
                category: "DEX".to_string(),
                tvl_usd: 4_200_000_000.0,
                apy_range: (2.0, 30.0),
            },
            ProtocolInfo {
                name: "Aave".to_string(),
                logo: "👻".to_string(),
                category: "Lending".to_string(),
                tvl_usd: 12_000_000_000.0,
                apy_range: (2.0, 15.0),
            },
            ProtocolInfo {
                name: "Compound".to_string(),
                logo: "🔷".to_string(),
                category: "Lending".to_string(),
                tvl_usd: 2_100_000_000.0,
                apy_range: (2.0, 8.0),
            },
            ProtocolInfo {
                name: "Curve".to_string(),
                logo: "💚".to_string(),
                category: "Stablecoin DEX".to_string(),
                tvl_usd: 3_800_000_000.0,
                apy_range: (2.0, 20.0),
            },
            ProtocolInfo {
                name: "Lido".to_string(),
                logo: "🌟".to_string(),
                category: "Liquid Staking".to_string(),
                tvl_usd: 15_000_000_000.0,
                apy_range: (3.0, 5.0),
            },
            ProtocolInfo {
                name: "Yearn".to_string(),
                logo: "📈".to_string(),
                category: "Yield".to_string(),
                tvl_usd: 500_000_000.0,
                apy_range: (5.0, 50.0),
            },
        ]
    }

    /// Calculate potential returns
    pub fn calculate_returns(&self, principal: f64, apy: f64, days: u32) -> f64 {
        principal * (1.0 + apy / 100.0).powf(days as f64 / 365.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portfolio() {
        let service = DefiPortfolioService::new();
        let portfolio = service.get_portfolio("0x123");
        
        assert!(portfolio.total_value_usd > 0.0);
        assert!(!portfolio.positions.is_empty());
    }
}
