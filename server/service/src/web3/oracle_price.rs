// =========================================
// Oracle Price Comparison Service
// Compares prices from multiple sources: Chainlink, Uniswap, CoinGecko, Binance
// =========================================

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use reqwest::Client;

use super::ServiceError;

// ============ Data Types ============

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OraclePrice {
    pub source: String,
    pub symbol: String,
    pub price: f64,
    pub timestamp: i64,
    pub confidence: Option<f64>,
    pub change_24h: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceComparison {
    pub symbol: String,
    pub prices: Vec<OraclePrice>,
    pub average_price: f64,
    pub price_diff_percent: f64,
    pub highest_price: OraclePrice,
    pub lowest_price: OraclePrice,
    pub arbitrage_opportunity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleConfig {
    pub chainlink_feeds: Vec<ChainlinkFeed>,
    pub supported_symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainlinkFeed {
    pub symbol: String,
    pub proxy_address: String,
    pub chain_id: u64,
}

// ============ Service Trait ============

#[async_trait]
pub trait TOraclePriceService: Send + Sync {
    async fn get_price_from_chainlink(&self, symbol: &str, _chain_id: u64) -> Result<OraclePrice, ServiceError>;
    async fn get_price_from_coingecko(&self, symbol: &str) -> Result<OraclePrice, ServiceError>;
    async fn get_price_from_binance(&self, symbol: &str) -> Result<OraclePrice, ServiceError>;
    async fn get_price_from_uniswap(&self, symbol: &str, _chain_id: u64) -> Result<OraclePrice, ServiceError>;
    async fn compare_prices(&self, symbol: &str) -> Result<PriceComparison, ServiceError>;
    async fn get_multi_symbol_comparison(&self, symbols: &[&str]) -> Result<Vec<PriceComparison>, ServiceError>;
    async fn get_oracle_status(&self) -> Result<Vec<OracleStatus>, ServiceError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OracleStatus {
    pub source: String,
    pub status: String,
    pub latency_ms: i64,
    pub last_update: i64,
}

// ============ Service Implementation ============

#[allow(dead_code)]
pub struct OraclePriceService {
    http_client: Client,
    chainlink_feeds: Vec<ChainlinkFeed>,
    supported_tokens: Vec<TokenConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub symbol: String,
    pub name: String,
    pub coingecko_id: String,
    pub binance_symbol: String,
    pub chainlink_feed: Option<String>,
    pub decimals: u8,
}

impl OraclePriceService {
    pub fn new() -> Self {
        let chainlink_feeds = vec![
            ChainlinkFeed { symbol: "BTC".to_string(), proxy_address: "0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c".to_string(), chain_id: 1 },
            ChainlinkFeed { symbol: "ETH".to_string(), proxy_address: "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string(), chain_id: 1 },
            ChainlinkFeed { symbol: "LINK".to_string(), proxy_address: "0x2c1d072e956AFFC0D435Cb7AC38EF18d24d9127c".to_string(), chain_id: 1 },
            ChainlinkFeed { symbol: "USDC".to_string(), proxy_address: "0x8fFFfA2dD15d1aAa3E1d3B13D5fC1f2A2dB2d400".to_string(), chain_id: 1 },
            ChainlinkFeed { symbol: "USDT".to_string(), proxy_address: "0xEe9F39fE4Ab621585dD01b2a1f7Da1dBF9f5dA1c".to_string(), chain_id: 1 },
            ChainlinkFeed { symbol: "BNB".to_string(), proxy_address: "0x14e613AC84a31f709eadbdF89c6C9e7615Ce8314".to_string(), chain_id: 56 },
            ChainlinkFeed { symbol: "SOL".to_string(), proxy_address: "0x4e8dF434483B989C761BE91F2B139a355B5c36d1".to_string(), chain_id: 1 },
            ChainlinkFeed { symbol: "AVAX".to_string(), proxy_address: "0x0A9823D5a4d4f6a3d72b53f1B4f8cB3d1a2E5b6c".to_string(), chain_id: 43114 },
            ChainlinkFeed { symbol: "MATIC".to_string(), proxy_address: "0x7AcD4E65fE2a8BfD7A5c7c8fE3dB8c9a2d4f6e8c".to_string(), chain_id: 137 },
        ];

        let supported_tokens = vec![
            TokenConfig { symbol: "BTC".to_string(), name: "Bitcoin".to_string(), coingecko_id: "bitcoin".to_string(), binance_symbol: "BTCUSDT".to_string(), chainlink_feed: Some("0xF4030086522a5bEEa4988F8cA5B36dbC97BeE88c".to_string()), decimals: 8 },
            TokenConfig { symbol: "ETH".to_string(), name: "Ethereum".to_string(), coingecko_id: "ethereum".to_string(), binance_symbol: "ETHUSDT".to_string(), chainlink_feed: Some("0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419".to_string()), decimals: 18 },
            TokenConfig { symbol: "LINK".to_string(), name: "Chainlink".to_string(), coingecko_id: "chainlink".to_string(), binance_symbol: "LINKUSDT".to_string(), chainlink_feed: Some("0x2c1d072e956AFFC0D435Cb7AC38EF18d24d9127c".to_string()), decimals: 18 },
            TokenConfig { symbol: "BNB".to_string(), name: "BNB".to_string(), coingecko_id: "binancecoin".to_string(), binance_symbol: "BNBUSDT".to_string(), chainlink_feed: Some("0x14e613AC84a31f709eadbdF89c6C9e7615Ce8314".to_string()), decimals: 18 },
            TokenConfig { symbol: "SOL".to_string(), name: "Solana".to_string(), coingecko_id: "solana".to_string(), binance_symbol: "SOLUSDT".to_string(), chainlink_feed: Some("0x4e8dF434483B989C761BE91F2B139a355B5c36d1".to_string()), decimals: 9 },
            TokenConfig { symbol: "XRP".to_string(), name: "XRP".to_string(), coingecko_id: "ripple".to_string(), binance_symbol: "XRPUSDT".to_string(), chainlink_feed: None, decimals: 6 },
            TokenConfig { symbol: "ADA".to_string(), name: "Cardano".to_string(), coingecko_id: "cardano".to_string(), binance_symbol: "ADAUSDT".to_string(), chainlink_feed: None, decimals: 6 },
            TokenConfig { symbol: "DOGE".to_string(), name: "Dogecoin".to_string(), coingecko_id: "dogecoin".to_string(), binance_symbol: "DOGEUSDT".to_string(), chainlink_feed: None, decimals: 8 },
            TokenConfig { symbol: "AVAX".to_string(), name: "Avalanche".to_string(), coingecko_id: "avalanche-2".to_string(), binance_symbol: "AVAXUSDT".to_string(), chainlink_feed: Some("0x0A9823D5a4d4f6a3d72b53f1B4f8cB3d1a2E5b6c".to_string()), decimals: 18 },
            TokenConfig { symbol: "DOT".to_string(), name: "Polkadot".to_string(), coingecko_id: "polkadot".to_string(), binance_symbol: "DOTUSDT".to_string(), chainlink_feed: None, decimals: 10 },
            TokenConfig { symbol: "MATIC".to_string(), name: "Polygon".to_string(), coingecko_id: "matic-network".to_string(), binance_symbol: "MATICUSDT".to_string(), chainlink_feed: Some("0x7AcD4E65fE2a8BfD7A5c7c8fE3dB8c9a2d4f6e8c".to_string()), decimals: 18 },
            TokenConfig { symbol: "UNI".to_string(), name: "Uniswap".to_string(), coingecko_id: "uniswap".to_string(), binance_symbol: "UNIUSDT".to_string(), chainlink_feed: None, decimals: 18 },
            TokenConfig { symbol: "AAVE".to_string(), name: "Aave".to_string(), coingecko_id: "aave".to_string(), binance_symbol: "AAVEUSDT".to_string(), chainlink_feed: None, decimals: 18 },
            TokenConfig { symbol: "MKR".to_string(), name: "Maker".to_string(), coingecko_id: "maker".to_string(), binance_symbol: "MKRUSDT".to_string(), chainlink_feed: None, decimals: 18 },
            TokenConfig { symbol: "CRV".to_string(), name: "Curve DAO".to_string(), coingecko_id: "curve-dao-token".to_string(), binance_symbol: "CRVUSDT".to_string(), chainlink_feed: None, decimals: 18 },
        ];

        Self {
            http_client: Client::new(),
            chainlink_feeds,
            supported_tokens,
        }
    }

    fn get_token_config(&self, symbol: &str) -> Option<&TokenConfig> {
        self.supported_tokens.iter().find(|t| t.symbol.eq_ignore_ascii_case(symbol))
    }

    // Get price from CoinGecko
    async fn get_price_from_coingecko(&self, symbol: &str) -> Result<OraclePrice, ServiceError> {
        let token = self.get_token_config(symbol).ok_or_else(|| {
            ServiceError::new(&format!("Token {} not supported", symbol))
        })?;

        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true",
            token.coingecko_id
        );

        let start = std::time::Instant::now();
        let response = self.http_client
            .get(&url)
            .header("Accept", "application/json")
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| ServiceError::new(&format!("CoinGecko request failed: {}", e)))?;

        let _latency = start.elapsed().as_millis() as i64;

        if !response.status().is_success() {
            return Err(ServiceError::new(&format!("CoinGecko API error: {}", response.status())));
        }

        let data: serde_json::Value = response.json().await
            .map_err(|e| ServiceError::new(&format!("Failed to parse CoinGecko response: {}", e)))?;

        let price = data[&token.coingecko_id]["usd"]
            .as_f64()
            .ok_or_else(|| ServiceError::new("Invalid price data from CoinGecko"))?;

        let change_24h = data[&token.coingecko_id]["usd_24h_change"]
            .as_f64();

        Ok(OraclePrice {
            source: "CoinGecko".to_string(),
            symbol: token.symbol.clone(),
            price,
            timestamp: chrono::Utc::now().timestamp(),
            confidence: Some(0.95),
            change_24h,
        })
    }

    // Get price from Binance
    async fn get_price_from_binance(&self, symbol: &str) -> Result<OraclePrice, ServiceError> {
        let token = self.get_token_config(symbol).ok_or_else(|| {
            ServiceError::new(&format!("Token {} not supported", symbol))
        })?;

        let url = format!(
            "https://api.binance.com/api/v3/ticker/24hr?symbol={}",
            token.binance_symbol
        );

        let start = std::time::Instant::now();
        let response = self.http_client
            .get(&url)
            .header("Accept", "application/json")
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| ServiceError::new(&format!("Binance request failed: {}", e)))?;

        let _latency = start.elapsed().as_millis() as i64;

        if !response.status().is_success() {
            return Err(ServiceError::new(&format!("Binance API error: {}", response.status())));
        }

        let data: serde_json::Value = response.json().await
            .map_err(|e| ServiceError::new(&format!("Failed to parse Binance response: {}", e)))?;

        let price = data["lastPrice"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| ServiceError::new("Invalid price data from Binance"))?;

        let change_24h = data["priceChangePercent"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok());

        Ok(OraclePrice {
            source: "Binance".to_string(),
            symbol: token.symbol.clone(),
            price,
            timestamp: chrono::Utc::now().timestamp(),
            confidence: Some(0.98),
            change_24h,
        })
    }

    // Get price from Chainlink (simulated - would need actual RPC in production)
    async fn get_price_from_chainlink(&self, symbol: &str, _chain_id: u64) -> Result<OraclePrice, ServiceError> {
        let token = self.get_token_config(symbol).ok_or_else(|| {
            ServiceError::new(&format!("Token {} not supported", symbol))
        })?;

        if token.chainlink_feed.is_none() {
            return Err(ServiceError::new(&format!("No Chainlink feed for {}", symbol)));
        }

        // In production, this would query the actual Chainlink price feed contract
        // For demo, we'll fetch from CoinGecko and simulate Chainlink's price
        let cg_price = self.get_price_from_coingecko(symbol).await?;
        
        // Add small variance to simulate Chainlink's price feed
        let variance = cg_price.price * 0.001; // 0.1% variance
        let chainlink_price = cg_price.price + variance;

        Ok(OraclePrice {
            source: "Chainlink".to_string(),
            symbol: token.symbol.clone(),
            price: chainlink_price,
            timestamp: chrono::Utc::now().timestamp(),
            confidence: Some(0.99),
            change_24h: cg_price.change_24h,
        })
    }

    // Get price from Uniswap (simulated TWAP)
    async fn get_price_from_uniswap(&self, symbol: &str, _chain_id: u64) -> Result<OraclePrice, ServiceError> {
        let token = self.get_token_config(symbol).ok_or_else(|| {
            ServiceError::new(&format!("Token {} not supported", symbol))
        })?;

        // In production, this would query Uniswap V3 TWAP
        // For demo, we'll use CoinGecko as base and add variance
        let cg_price = self.get_price_from_coingecko(symbol).await?;
        
        // Add small variance to simulate Uniswap TWAP
        let variance = cg_price.price * 0.002; // 0.2% variance
        let uniswap_price = cg_price.price - variance;

        Ok(OraclePrice {
            source: "Uniswap".to_string(),
            symbol: token.symbol.clone(),
            price: uniswap_price,
            timestamp: chrono::Utc::now().timestamp(),
            confidence: Some(0.90),
            change_24h: cg_price.change_24h,
        })
    }

    // Compare prices from all sources
    async fn compare_prices(&self, symbol: &str) -> Result<PriceComparison, ServiceError> {
        let mut prices = Vec::new();

        // Fetch from all sources in parallel
        let cg_result = self.get_price_from_coingecko(symbol).await;
        let binance_result = self.get_price_from_binance(symbol).await;
        let chainlink_result = self.get_price_from_chainlink(symbol, 1).await;
        let uniswap_result = self.get_price_from_uniswap(symbol, 1).await;

        if let Ok(price) = cg_result {
            prices.push(price);
        }
        if let Ok(price) = binance_result {
            prices.push(price);
        }
        if let Ok(price) = chainlink_result {
            prices.push(price);
        }
        if let Ok(price) = uniswap_result {
            prices.push(price);
        }

        if prices.is_empty() {
            return Err(ServiceError::new(&format!("Failed to fetch prices for {}", symbol)));
        }

        let total_price: f64 = prices.iter().map(|p| p.price).sum();
        let average_price = total_price / prices.len() as f64;

        let highest_price = prices.iter().cloned().reduce(|a, b| if a.price > b.price { a } else { b }).unwrap();
        let lowest_price = prices.iter().cloned().reduce(|a, b| if a.price < b.price { a } else { b }).unwrap();

        let price_diff_percent = if lowest_price.price > 0.0 {
            ((highest_price.price - lowest_price.price) / lowest_price.price) * 100.0
        } else {
            0.0
        };

        // Consider arbitrage opportunity if price diff > 0.5%
        let arbitrage_opportunity = price_diff_percent > 0.5;

        Ok(PriceComparison {
            symbol: symbol.to_uppercase(),
            prices,
            average_price,
            price_diff_percent,
            highest_price,
            lowest_price,
            arbitrage_opportunity,
        })
    }

    // Get comparison for multiple symbols
    async fn get_multi_symbol_comparison(&self, symbols: &[&str]) -> Result<Vec<PriceComparison>, ServiceError> {
        let mut results = Vec::new();

        for symbol in symbols {
            match self.compare_prices(symbol).await {
                Ok(comparison) => results.push(comparison),
                Err(e) => {
                    // Log error but continue with other symbols
                    tracing::warn!("Failed to get comparison for {}: {}", symbol, e.message);
                }
            }
        }

        if results.is_empty() {
            return Err(ServiceError::new("Failed to fetch any price comparisons"));
        }

        Ok(results)
    }

    // Get oracle status
    async fn get_oracle_status(&self) -> Result<Vec<OracleStatus>, ServiceError> {
        let mut statuses = Vec::new();

        // Test CoinGecko
        let start = std::time::Instant::now();
        let cg_status = match self.http_client
            .get("https://api.coingecko.com/api/v3/ping")
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => "online",
            _ => "offline",
        };
        let cg_latency = start.elapsed().as_millis() as i64;
        statuses.push(OracleStatus {
            source: "CoinGecko".to_string(),
            status: cg_status.to_string(),
            latency_ms: cg_latency,
            last_update: chrono::Utc::now().timestamp(),
        });

        // Test Binance
        let start = std::time::Instant::now();
        let binance_status = match self.http_client
            .get("https://api.binance.com/api/v3/ping")
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => "online",
            _ => "offline",
        };
        let binance_latency = start.elapsed().as_millis() as i64;
        statuses.push(OracleStatus {
            source: "Binance".to_string(),
            status: binance_status.to_string(),
            latency_ms: binance_latency,
            last_update: chrono::Utc::now().timestamp(),
        });

        // Chainlink and Uniswap are simulated
        statuses.push(OracleStatus {
            source: "Chainlink".to_string(),
            status: "online".to_string(),
            latency_ms: 50,
            last_update: chrono::Utc::now().timestamp(),
        });

        statuses.push(OracleStatus {
            source: "Uniswap".to_string(),
            status: "online".to_string(),
            latency_ms: 100,
            last_update: chrono::Utc::now().timestamp(),
        });

        Ok(statuses)
    }
}

#[async_trait]
impl TOraclePriceService for OraclePriceService {
    async fn get_price_from_chainlink(&self, symbol: &str, chain_id: u64) -> Result<OraclePrice, ServiceError> {
        self.get_price_from_chainlink(symbol, chain_id).await
    }

    async fn get_price_from_coingecko(&self, symbol: &str) -> Result<OraclePrice, ServiceError> {
        self.get_price_from_coingecko(symbol).await
    }

    async fn get_price_from_binance(&self, symbol: &str) -> Result<OraclePrice, ServiceError> {
        self.get_price_from_binance(symbol).await
    }

    async fn get_price_from_uniswap(&self, symbol: &str, chain_id: u64) -> Result<OraclePrice, ServiceError> {
        self.get_price_from_uniswap(symbol, chain_id).await
    }

    async fn compare_prices(&self, symbol: &str) -> Result<PriceComparison, ServiceError> {
        self.compare_prices(symbol).await
    }

    async fn get_multi_symbol_comparison(&self, symbols: &[&str]) -> Result<Vec<PriceComparison>, ServiceError> {
        self.get_multi_symbol_comparison(symbols).await
    }

    async fn get_oracle_status(&self) -> Result<Vec<OracleStatus>, ServiceError> {
        self.get_oracle_status().await
    }
}

// ============ Helper Functions ============

pub fn create_oracle_price_service() -> Arc<dyn TOraclePriceService> {
    Arc::new(OraclePriceService::new())
}
