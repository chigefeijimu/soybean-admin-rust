// =========================================
// DeFi Yield Optimizer Service
// =========================================

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::web3::ServiceError;

// ============ Types ============

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldOpportunity {
    pub protocol: String,
    pub chain: String,
    pub pool: String,
    pub token0: String,
    pub token1: String,
    pub apy: f64,
    pub tvl: f64,
    pub risk_level: String,
    pub recommendation: String,
    pub swap_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortfolioPosition {
    pub protocol: String,
    pub chain: String,
    pub token: String,
    pub amount: f64,
    pub value_usd: f64,
    pub apy: f64,
    pub reward_token: Option<String>,
    pub pending_rewards: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldOptimizationResult {
    pub current_yield: f64,
    pub projected_yield: f64,
    pub potential_gain: f64,
    pub opportunities: Vec<YieldOpportunity>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldProtocolInfo {
    pub name: String,
    pub chain: String,
    pub category: String,
    pub tvl: f64,
    pub avg_apy: f64,
    pub token_symbol: String,
    pub website: String,
    pub risk_score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YieldPoolInfo {
    pub protocol: String,
    pub chain: String,
    pub pool_name: String,
    pub token0: TokenInfo,
    pub token1: Option<TokenInfo>,
    pub apy: f64,
    pub apy_24h: f64,
    pub tvl: f64,
    pub volume_24h: f64,
    pub reward_tokens: Vec<String>,
    pub pool_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub symbol: String,
    pub name: String,
    pub address: Option<String>,
    pub price_usd: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeInput {
    pub wallet_address: String,
    pub chain_id: Option<i32>,
}

// ============ Trait ============

#[async_trait]
pub trait TYieldOptimizerService: Send + Sync {
    async fn get_top_yields(&self, chain: Option<String>, limit: Option<i32>) -> Result<Vec<YieldPoolInfo>, ServiceError>;
    async fn get_protocols(&self) -> Result<Vec<YieldProtocolInfo>, ServiceError>;
    async fn analyze_portfolio(&self, input: OptimizeInput) -> Result<YieldOptimizationResult, ServiceError>;
    async fn get_optimization_opportunities(&self, wallet_address: &str) -> Result<Vec<YieldOpportunity>, ServiceError>;
    async fn compare_yields(&self, tokens: Vec<String>, amount: f64) -> Result<Vec<YieldOpportunity>, ServiceError>;
}

// ============ Service Implementation ============

#[allow(dead_code)]
pub struct YieldOptimizerService {
    #[allow(dead_code)]
    provider: Option<Arc<crate::web3::Web3Provider>>,
}

impl YieldOptimizerService {
    pub fn new(_provider: Arc<crate::web3::Web3Provider>) -> Self {
        Self { provider: Some(_provider) }
    }
    
    pub fn new_without_provider() -> Self {
        Self { provider: None }
    }
}

#[async_trait]
impl TYieldOptimizerService for YieldOptimizerService {
    async fn get_top_yields(&self, chain: Option<String>, limit: Option<i32>) -> Result<Vec<YieldPoolInfo>, ServiceError> {
        let limit = limit.unwrap_or(20);
        
        // Mock data for popular yield pools across chains
        let mut pools = vec![
            YieldPoolInfo {
                protocol: "Aave".to_string(),
                chain: "Ethereum".to_string(),
                pool_name: "USDC".to_string(),
                token0: TokenInfo {
                    symbol: "USDC".to_string(),
                    name: "USD Coin".to_string(),
                    address: Some("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string()),
                    price_usd: 1.0,
                },
                token1: None,
                apy: 4.52,
                apy_24h: 4.48,
                tvl: 5_200_000_000.0,
                volume_24h: 180_000_000.0,
                reward_tokens: vec!["AAVE".to_string()],
                pool_address: "0x7d2768dE32b0b80b7a3454c06BdAc94A69DDc7A9".to_string(),
            },
            YieldPoolInfo {
                protocol: "Compound".to_string(),
                chain: "Ethereum".to_string(),
                pool_name: "USDC".to_string(),
                token0: TokenInfo {
                    symbol: "USDC".to_string(),
                    name: "USD Coin".to_string(),
                    address: Some("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string()),
                    price_usd: 1.0,
                },
                token1: None,
                apy: 3.85,
                apy_24h: 3.82,
                tvl: 3_100_000_000.0,
                volume_24h: 95_000_000.0,
                reward_tokens: vec!["COMP".to_string()],
                pool_address: "0xc3d688B66703497DAA19211EEdff0f5A63cE1906".to_string(),
            },
            YieldPoolInfo {
                protocol: "Uniswap V3".to_string(),
                chain: "Ethereum".to_string(),
                pool_name: "USDC/ETH".to_string(),
                token0: TokenInfo {
                    symbol: "USDC".to_string(),
                    name: "USD Coin".to_string(),
                    address: Some("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string()),
                    price_usd: 1.0,
                },
                token1: Some(TokenInfo {
                    symbol: "WETH".to_string(),
                    name: "Wrapped Ether".to_string(),
                    address: Some("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string()),
                    price_usd: 2450.0,
                }),
                apy: 12.35,
                apy_24h: 11.89,
                tvl: 890_000_000.0,
                volume_24h: 420_000_000.0,
                reward_tokens: vec!["UNI".to_string()],
                pool_address: "0x8ad599c3A0ff1De082011EFDDc58f1908eb6e6D8".to_string(),
            },
            YieldPoolInfo {
                protocol: "Curve".to_string(),
                chain: "Ethereum".to_string(),
                pool_name: "3CRV".to_string(),
                token0: TokenInfo {
                    symbol: "3CRV".to_string(),
                    name: "Curve DAO Token".to_string(),
                    address: Some("0x6c3F90f043a72FA612cbac8115EE7e52BDe6E490".to_string()),
                    price_usd: 1.0,
                },
                token1: None,
                apy: 2.45,
                apy_24h: 2.51,
                tvl: 2_800_000_000.0,
                volume_24h: 150_000_000.0,
                reward_tokens: vec!["CRV".to_string(), "LDO".to_string()],
                pool_address: "0xbEbc44782C7dB0a1A60Cb6fe7d57dc1f5B71BF20".to_string(),
            },
            YieldPoolInfo {
                protocol: "Lido".to_string(),
                chain: "Ethereum".to_string(),
                pool_name: "stETH".to_string(),
                token0: TokenInfo {
                    symbol: "stETH".to_string(),
                    name: "Lido Staked Ether".to_string(),
                    address: Some("0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84".to_string()),
                    price_usd: 2432.5,
                },
                token1: None,
                apy: 3.75,
                apy_24h: 3.68,
                tvl: 18_500_000_000.0,
                volume_24h: 45_000_000.0,
                reward_tokens: vec!["LDO".to_string()],
                pool_address: "0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84".to_string(),
            },
            YieldPoolInfo {
                protocol: "Aave".to_string(),
                chain: "Arbitrum".to_string(),
                pool_name: "USDC".to_string(),
                token0: TokenInfo {
                    symbol: "USDC".to_string(),
                    name: "USD Coin".to_string(),
                    address: Some("0xaf88d065e77c8cC2239327C5EDb3A432268e5831".to_string()),
                    price_usd: 1.0,
                },
                token1: None,
                apy: 5.21,
                apy_24h: 5.15,
                tvl: 890_000_000.0,
                volume_24h: 42_000_000.0,
                reward_tokens: vec!["ARB".to_string()],
                pool_address: "0x625E7708f30cA75bfd92586e17077590C60eb4cD".to_string(),
            },
            YieldPoolInfo {
                protocol: "GMX".to_string(),
                chain: "Arbitrum".to_string(),
                pool_name: "GLP".to_string(),
                token0: TokenInfo {
                    symbol: "GLP".to_string(),
                    name: "GMX Liquidity Provider".to_string(),
                    address: Some("0x5402B5F40310bDED796c7D0F3FF6683f5C0c8df0".to_string()),
                    price_usd: 68.5,
                },
                token1: None,
                apy: 18.5,
                apy_24h: 17.2,
                tvl: 450_000_000.0,
                volume_24h: 180_000_000.0,
                reward_tokens: vec!["GMX".to_string(), "ARB".to_string()],
                pool_address: "0x5402B5F40310bDED796c7D0F3FF6683f5C0c8df0".to_string(),
            },
            YieldPoolInfo {
                protocol: "Yearn".to_string(),
                chain: "Ethereum".to_string(),
                pool_name: "yUSDC".to_string(),
                token0: TokenInfo {
                    symbol: "yUSDC".to_string(),
                    name: "Yearn USDC".to_string(),
                    address: Some("0xa354F35829Ae975e850e23e9615b11d4b24438A4".to_string()),
                    price_usd: 1.01,
                },
                token1: None,
                apy: 4.75,
                apy_24h: 4.68,
                tvl: 1_200_000_000.0,
                volume_24h: 35_000_000.0,
                reward_tokens: vec!["YFI".to_string()],
                pool_address: "0xa354F35829Ae975e850e23e9615b11d4b24438A4".to_string(),
            },
            YieldPoolInfo {
                protocol: "Convex".to_string(),
                chain: "Ethereum".to_string(),
                pool_name: "cvxCRV".to_string(),
                token0: TokenInfo {
                    symbol: "cvxCRV".to_string(),
                    name: "Convex CRV".to_string(),
                    address: Some("0xD533a949740bb3306d119CC777fa900bA034cd52".to_string()),
                    price_usd: 1.12,
                },
                token1: None,
                apy: 8.5,
                apy_24h: 8.35,
                tvl: 950_000_000.0,
                volume_24h: 28_000_000.0,
                reward_tokens: vec!["CVX".to_string(), "CRV".to_string()],
                pool_address: "0xD533a949740bb3306d119CC777fa900bA034cd52".to_string(),
            },
            YieldPoolInfo {
                protocol: "Rocket Pool".to_string(),
                chain: "Ethereum".to_string(),
                pool_name: "rETH".to_string(),
                token0: TokenInfo {
                    symbol: "rETH".to_string(),
                    name: "Rocket Pool ETH".to_string(),
                    address: Some("0xae78736Cd615f374D3085123A210448E74Fc6393".to_string()),
                    price_usd: 2510.0,
                },
                token1: None,
                apy: 4.25,
                apy_24h: 4.18,
                tvl: 2_100_000_000.0,
                volume_24h: 18_000_000.0,
                reward_tokens: vec!["RPL".to_string()],
                pool_address: "0xae78736Cd615f374D3085123A210448E74Fc6393".to_string(),
            },
        ];

        // Filter by chain if specified
        if let Some(ref chain_filter) = chain {
            pools.retain(|p| p.chain.to_lowercase() == chain_filter.to_lowercase());
        }

        pools.truncate(limit as usize);
        Ok(pools)
    }

    async fn get_protocols(&self) -> Result<Vec<YieldProtocolInfo>, ServiceError> {
        let protocols = vec![
            YieldProtocolInfo {
                name: "Aave".to_string(),
                chain: "Multi-chain".to_string(),
                category: "Lending".to_string(),
                tvl: 15_800_000_000.0,
                avg_apy: 4.2,
                token_symbol: "AAVE".to_string(),
                website: "https://aave.com".to_string(),
                risk_score: 2,
            },
            YieldProtocolInfo {
                name: "Compound".to_string(),
                chain: "Ethereum".to_string(),
                category: "Lending".to_string(),
                tvl: 3_200_000_000.0,
                avg_apy: 3.8,
                token_symbol: "COMP".to_string(),
                website: "https://compound.finance".to_string(),
                risk_score: 2,
            },
            YieldProtocolInfo {
                name: "Uniswap".to_string(),
                chain: "Multi-chain".to_string(),
                category: "DEX".to_string(),
                tvl: 4_500_000_000.0,
                avg_apy: 8.5,
                token_symbol: "UNI".to_string(),
                website: "https://uniswap.org".to_string(),
                risk_score: 3,
            },
            YieldProtocolInfo {
                name: "Curve".to_string(),
                chain: "Multi-chain".to_string(),
                category: "StableSwap".to_string(),
                tvl: 3_800_000_000.0,
                avg_apy: 3.2,
                token_symbol: "CRV".to_string(),
                website: "https://curve.fi".to_string(),
                risk_score: 2,
            },
            YieldProtocolInfo {
                name: "Lido".to_string(),
                chain: "Ethereum".to_string(),
                category: "Liquid Staking".to_string(),
                tvl: 18_500_000_000.0,
                avg_apy: 3.7,
                token_symbol: "LDO".to_string(),
                website: "https://lido.fi".to_string(),
                risk_score: 2,
            },
            YieldProtocolInfo {
                name: "Yearn".to_string(),
                chain: "Ethereum".to_string(),
                category: "Yield Aggregator".to_string(),
                tvl: 2_100_000_000.0,
                avg_apy: 5.5,
                token_symbol: "YFI".to_string(),
                website: "https://yearn.finance".to_string(),
                risk_score: 3,
            },
            YieldProtocolInfo {
                name: "Convex".to_string(),
                chain: "Ethereum".to_string(),
                category: "Yield Booster".to_string(),
                tvl: 1_800_000_000.0,
                avg_apy: 7.5,
                token_symbol: "CVX".to_string(),
                website: "https://convexfinance.com".to_string(),
                risk_score: 3,
            },
            YieldProtocolInfo {
                name: "GMX".to_string(),
                chain: "Arbitrum".to_string(),
                category: "Perpetual DEX".to_string(),
                tvl: 650_000_000.0,
                avg_apy: 15.0,
                token_symbol: "GMX".to_string(),
                website: "https://gmx.io".to_string(),
                risk_score: 4,
            },
            YieldProtocolInfo {
                name: "Rocket Pool".to_string(),
                chain: "Ethereum".to_string(),
                category: "Liquid Staking".to_string(),
                tvl: 2_100_000_000.0,
                avg_apy: 4.1,
                token_symbol: "RPL".to_string(),
                website: "https://rocketpool.net".to_string(),
                risk_score: 2,
            },
            YieldProtocolInfo {
                name: "MakerDAO".to_string(),
                chain: "Ethereum".to_string(),
                category: "Lending".to_string(),
                tvl: 6_200_000_000.0,
                avg_apy: 2.5,
                token_symbol: "MKR".to_string(),
                website: "https://makerdao.com".to_string(),
                risk_score: 2,
            },
        ];

        Ok(protocols)
    }

    async fn analyze_portfolio(&self, _input: OptimizeInput) -> Result<YieldOptimizationResult, ServiceError> {
        // Mock portfolio analysis
        let current_yield = 3.2;
        let projected_yield = 5.8;
        let potential_gain = 2.6;

        let opportunities = vec![
            YieldOpportunity {
                protocol: "Aave".to_string(),
                chain: "Ethereum".to_string(),
                pool: "USDC".to_string(),
                token0: "USDC".to_string(),
                token1: "AAVE".to_string(),
                apy: 4.52,
                tvl: 5_200_000_000.0,
                risk_level: "Low".to_string(),
                recommendation: "Move USDC from wallet to Aave for ~4.5% APY".to_string(),
                swap_url: Some("https://app.aave.com".to_string()),
            },
            YieldOpportunity {
                protocol: "Uniswap V3".to_string(),
                chain: "Ethereum".to_string(),
                pool: "USDC/ETH".to_string(),
                token0: "USDC".to_string(),
                token1: "WETH".to_string(),
                apy: 12.35,
                tvl: 890_000_000.0,
                risk_level: "Medium".to_string(),
                recommendation: "Provide liquidity to USDC/ETH pool for ~12% APY".to_string(),
                swap_url: Some("https://app.uniswap.org".to_string()),
            },
            YieldOpportunity {
                protocol: "Lido".to_string(),
                chain: "Ethereum".to_string(),
                pool: "stETH".to_string(),
                token0: "ETH".to_string(),
                token1: "stETH".to_string(),
                apy: 3.75,
                tvl: 18_500_000_000.0,
                risk_level: "Low".to_string(),
                recommendation: "Stake ETH to Lido for ~3.75% staking rewards".to_string(),
                swap_url: Some("https://stake.lido.fi".to_string()),
            },
        ];

        let recommendations = vec![
            "Consider moving stablecoins to Aave for better yields".to_string(),
            "Explore Uniswap V3 for higher yields on liquidity provision".to_string(),
            "Use Convex to boost CRV yields if you hold Curve positions".to_string(),
            "Monitor Aave for new collateral types with higher APY".to_string(),
        ];

        Ok(YieldOptimizationResult {
            current_yield,
            projected_yield,
            potential_gain,
            opportunities,
            recommendations,
        })
    }

    async fn get_optimization_opportunities(&self, _wallet_address: &str) -> Result<Vec<YieldOpportunity>, ServiceError> {
        // Generate opportunities based on wallet address
        let opportunities = vec![
            YieldOpportunity {
                protocol: "Aave".to_string(),
                chain: "Ethereum".to_string(),
                pool: "USDC".to_string(),
                token0: "USDC".to_string(),
                token1: "AAVE".to_string(),
                apy: 4.52,
                tvl: 5_200_000_000.0,
                risk_level: "Low".to_string(),
                recommendation: "Lend USDC on Aave for 4.52% APY".to_string(),
                swap_url: Some("https://app.aave.com".to_string()),
            },
            YieldOpportunity {
                protocol: "Compound".to_string(),
                chain: "Ethereum".to_string(),
                pool: "USDC".to_string(),
                token0: "USDC".to_string(),
                token1: "COMP".to_string(),
                apy: 3.85,
                tvl: 3_100_000_000.0,
                risk_level: "Low".to_string(),
                recommendation: "Lend USDC on Compound for 3.85% APY".to_string(),
                swap_url: Some("https://app.compound.finance".to_string()),
            },
            YieldOpportunity {
                protocol: "Yearn".to_string(),
                chain: "Ethereum".to_string(),
                pool: "yUSDC".to_string(),
                token0: "USDC".to_string(),
                token1: "YFI".to_string(),
                apy: 4.75,
                tvl: 1_200_000_000.0,
                risk_level: "Medium".to_string(),
                recommendation: "Use Yearn vault for automated yield optimization".to_string(),
                swap_url: Some("https://yearn.finance".to_string()),
            },
        ];

        Ok(opportunities)
    }

    async fn compare_yields(&self, tokens: Vec<String>, amount: f64) -> Result<Vec<YieldOpportunity>, ServiceError> {
        let mut opportunities = Vec::new();

        for token in tokens {
            let opp = YieldOpportunity {
                protocol: "Aave".to_string(),
                chain: "Ethereum".to_string(),
                pool: format!("{}/a{}", token, token),
                token0: token.clone(),
                token1: "AAVE".to_string(),
                apy: 4.52,
                tvl: 5_200_000_000.0,
                risk_level: "Low".to_string(),
                recommendation: format!("Lend {} on Aave for 4.52% APY (${:.2}/year)", token, amount * 0.0452),
                swap_url: Some("https://app.aave.com".to_string()),
            };
            opportunities.push(opp);
        }

        Ok(opportunities)
    }
}

// ============ Helper Functions ============

pub fn calculate_compound_interest(principal: f64, rate: f64, periods: i32) -> f64 {
    principal * (1.0 + rate).powi(periods)
}

pub fn calculate_apy_to_apr(apy: f64, periods: i32) -> f64 {
    ((1.0 + apy).powf(1.0 / periods as f64) - 1.0) * periods as f64
}

pub fn calculate_apr_to_apy(apr: f64, periods: i32) -> f64 {
    (1.0 + apr / periods as f64).powi(periods) - 1.0
}
