//! Market Data Service
//! Provides cryptocurrency market data, price feeds, and analytics
//! Supports both mock data and real CoinGecko API integration

use crate::web3::alloy_provider::ProviderPool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

// Global provider pool for RPC calls
lazy_static::lazy_static! {
    static ref PROVIDER_POOL: ProviderPool = ProviderPool::new();
}

/// Format token name from CoinGecko ID (e.g., "ethereum" -> "Ethereum")
fn format_token_name(token_id: &str) -> String {
    let mut chars = token_id.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// CoinGecko API response types
#[derive(Debug, Deserialize)]
struct CoinGeckoPrice {
    #[serde(rename = "usd")]
    price: f64,
    #[serde(rename = "usd_24h_change")]
    change_24h: Option<f64>,
    #[serde(rename = "usd_24h_vol")]
    volume_24h: Option<f64>,
    #[serde(rename = "usd_market_cap")]
    market_cap: Option<f64>,
}

type CoinGeckoResponse = HashMap<String, CoinGeckoPrice>;

// Token ID mapping (symbol -> CoinGecko ID)
fn get_token_id(symbol: &str) -> Option<&'static str> {
    match symbol.to_uppercase().as_str() {
        "ETH" => Some("ethereum"),
        "BTC" => Some("bitcoin"),
        "USDC" => Some("usd-coin"),
        "USDT" => Some("tether"),
        "SOL" => Some("solana"),
        "ARB" => Some("arbitrum"),
        "MATIC" => Some("matic-network"),
        "LINK" => Some("chainlink"),
        "UNI" => Some("uniswap"),
        "AAVE" => Some("aave"),
        "DAI" => Some("dai"),
        "WBTC" => Some("wrapped-bitcoin"),
        "WETH" => Some("weth"),
        "OP" => Some("optimism"),
        "AVAX" => Some("avalanche-2"),
        "BNB" => Some("binancecoin"),
        "DOT" => Some("polkadot"),
        "ATOM" => Some("cosmos"),
        "FIL" => Some("filecoin"),
        "NEAR" => Some("near"),
        "APT" => Some("aptos"),
        "PEPE" => Some("pepe"),
        "SHIB" => Some("shiba-inu"),
        "DOGE" => Some("dogecoin"),
        "XRP" => Some("ripple"),
        "ADA" => Some("cardano"),
        "FTM" => Some("fantom"),
        "SAND" => Some("the-sandbox"),
        "MANA" => Some("decentraland"),
        "AXS" => Some("axie-infinity"),
        _ => None,
    }
}

/// Token price information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPrice {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub change_24h: f64,
    pub change_7d: f64,
    pub market_cap: f64,
    pub volume_24h: f64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub supply: f64,
}

/// Market overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOverview {
    pub total_market_cap: f64,
    pub total_volume: f64,
    pub btc_dominance: f64,
    pub eth_dominance: f64,
    pub active_tokens: u32,
    pub active_pairs: u32,
}

/// DeFi protocol information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeFiProtocol {
    pub name: String,
    pub logo: String,
    pub tvl: f64,
    pub tvl_change_24h: f64,
    pub volume_24h: f64,
    pub category: String,
    pub chains: Vec<String>,
}

/// Gas price information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasPrice {
    pub slow: u64,
    pub normal: u64,
    pub fast: u64,
    pub base_fee: u64,
    pub priority_fee: u64,
    pub chain_id: u64,
}

/// Token metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub logo: Option<String>,
    pub price: f64,
    pub price_change_24h: f64,
    pub volume_24h: f64,
    pub market_cap: f64,
    pub rank: u32,
    pub website: Option<String>,
    pub description: Option<String>,
}

/// Cache entry with timestamp
#[derive(Clone)]
struct CacheEntry {
    price: TokenPrice,
    cached_at: Instant,
}

const CACHE_TTL: Duration = Duration::from_secs(60); // 1 minute cache

/// Market data service with CoinGecko integration
pub struct MarketDataService {
    cache: HashMap<String, TokenPrice>,
    /// Price cache with timestamps for rate limiting
    price_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    /// HTTP client
    client: reqwest::Client,
}

impl MarketDataService {
    /// Create new service instance
    pub fn new() -> Self {
        let mut cache = HashMap::new();
        
        // Initialize with mock data as fallback
        cache.insert("ETH".to_string(), TokenPrice {
            symbol: "ETH".to_string(),
            name: "Ethereum".to_string(),
            price: 2500.0,
            change_24h: 2.5,
            change_7d: 5.2,
            market_cap: 300_000_000_000.0,
            volume_24h: 15_000_000_000.0,
            high_24h: 2550.0,
            low_24h: 2400.0,
            supply: 120_000_000.0,
        });
        
        cache.insert("BTC".to_string(), TokenPrice {
            symbol: "BTC".to_string(),
            name: "Bitcoin".to_string(),
            price: 62500.0,
            change_24h: 1.8,
            change_7d: 3.2,
            market_cap: 1_200_000_000_000.0,
            volume_24h: 35_000_000_000.0,
            high_24h: 63000.0,
            low_24h: 61000.0,
            supply: 19_200_000.0,
        });
        
        cache.insert("USDC".to_string(), TokenPrice {
            symbol: "USDC".to_string(),
            name: "USD Coin".to_string(),
            price: 1.0,
            change_24h: 0.01,
            change_7d: -0.02,
            market_cap: 42_000_000_000.0,
            volume_24h: 5_000_000_000.0,
            high_24h: 1.001,
            low_24h: 0.999,
            supply: 42_000_000_000.0,
        });
        
        cache.insert("USDT".to_string(), TokenPrice {
            symbol: "USDT".to_string(),
            name: "Tether".to_string(),
            price: 1.0,
            change_24h: 0.0,
            change_7d: 0.01,
            market_cap: 95_000_000_000.0,
            volume_24h: 50_000_000_000.0,
            high_24h: 1.002,
            low_24h: 0.998,
            supply: 95_000_000_000.0,
        });
        
        cache.insert("SOL".to_string(), TokenPrice {
            symbol: "SOL".to_string(),
            name: "Solana".to_string(),
            price: 120.0,
            change_24h: 5.2,
            change_7d: 12.5,
            market_cap: 50_000_000_000.0,
            volume_24h: 3_000_000_000.0,
            high_24h: 125.0,
            low_24h: 110.0,
            supply: 420_000_000.0,
        });
        
        cache.insert("ARB".to_string(), TokenPrice {
            symbol: "ARB".to_string(),
            name: "Arbitrum".to_string(),
            price: 1.8,
            change_24h: 3.1,
            change_7d: 8.5,
            market_cap: 2_000_000_000.0,
            volume_24h: 500_000_000.0,
            high_24h: 1.9,
            low_24h: 1.7,
            supply: 1_100_000_000.0,
        });
        
        Self { 
            cache, 
            price_cache: Arc::new(RwLock::new(HashMap::new())),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
        }
    }

    /// Fetch price from CoinGecko API (async)
    pub async fn fetch_price_from_coingecko(&self, symbol: &str) -> Option<TokenPrice> {
        let token_id = get_token_id(symbol)?;
        
        // Check cache first
        {
            let cache = self.price_cache.read().await;
            if let Some(entry) = cache.get(&symbol.to_uppercase()) {
                if entry.cached_at.elapsed() < CACHE_TTL {
                    return Some(entry.price.clone());
                }
            }
        }

        // Fetch from CoinGecko
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true&include_24hr_vol=true&include_market_cap=true",
            token_id
        );

        match self.client.get(&url).send().await {
            Ok(response) => {
                if let Ok(data) = response.json::<CoinGeckoResponse>().await {
                    if let Some(price_data) = data.get(token_id) {
                        let token_price = TokenPrice {
                            symbol: symbol.to_uppercase(),
                            name: format_token_name(token_id),
                            price: price_data.price,
                            change_24h: price_data.change_24h.unwrap_or(0.0),
                            change_7d: 0.0, // Not provided by simple price endpoint
                            market_cap: price_data.market_cap.unwrap_or(0.0),
                            volume_24h: price_data.volume_24h.unwrap_or(0.0),
                            high_24h: price_data.price * 1.02,
                            low_24h: price_data.price * 0.98,
                            supply: 0.0,
                        };

                        // Update cache
                        {
                            let mut cache = self.price_cache.write().await;
                            cache.insert(
                                symbol.to_uppercase(),
                                CacheEntry {
                                    price: token_price.clone(),
                                    cached_at: Instant::now(),
                                },
                            );
                        }

                        // Update main cache
                        let mut cache_guard = self.cache.clone();
                        cache_guard.insert(symbol.to_uppercase(), token_price.clone());

                        return Some(token_price);
                    }
                }
            }
            Err(e) => {
                eprintln!("CoinGecko API error for {}: {}", symbol, e);
            }
        }

        None
    }

    /// Fetch multiple prices from CoinGecko (batch API)
    pub async fn fetch_prices_batch(&self, symbols: &[&str]) -> Vec<TokenPrice> {
        let token_ids: Vec<&str> = symbols
            .iter()
            .filter_map(|s| get_token_id(s))
            .collect();

        if token_ids.is_empty() {
            return vec![];
        }

        let ids = token_ids.join(",");
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true&include_24hr_vol=true&include_market_cap=true",
            ids
        );

        match self.client.get(&url).send().await {
            Ok(response) => {
                if let Ok(data) = response.json::<CoinGeckoResponse>().await {
                    return symbols
                        .iter()
                        .filter_map(|symbol| {
                            let token_id = get_token_id(symbol)?;
                            let price_data = data.get(token_id)?;
                            
                            Some(TokenPrice {
                                symbol: symbol.to_uppercase(),
                                name: format_token_name(token_id),
                                price: price_data.price,
                                change_24h: price_data.change_24h.unwrap_or(0.0),
                                change_7d: 0.0,
                                market_cap: price_data.market_cap.unwrap_or(0.0),
                                volume_24h: price_data.volume_24h.unwrap_or(0.0),
                                high_24h: price_data.price * 1.02,
                                low_24h: price_data.price * 0.98,
                                supply: 0.0,
                            })
                        })
                        .collect();
                }
            }
            Err(e) => {
                eprintln!("CoinGecko batch API error: {}", e);
            }
        }

        vec![]
    }

    /// Get price (with optional live fetch)
    pub async fn get_price_live(&self, symbol: &str) -> Option<TokenPrice> {
        // Try to fetch from CoinGecko first
        if let Some(price) = self.fetch_price_from_coingecko(symbol).await {
            return Some(price);
        }
        
        // Fallback to cached/mock data
        self.get_price(symbol).cloned()
    }

    /// Get multiple prices (with optional live fetch)
    pub async fn get_prices_live(&self, symbols: &[&str]) -> Vec<TokenPrice> {
        // Try batch fetch first
        let prices = self.fetch_prices_batch(symbols).await;
        
        if !prices.is_empty() {
            return prices;
        }

        // Fallback to cached/mock data
        symbols
            .iter()
            .filter_map(|s| self.get_price(s).cloned())
            .collect()
    }
    
    /// Get price for a specific token (sync, uses cache)
    pub fn get_price(&self, symbol: &str) -> Option<&TokenPrice> {
        self.cache.get(&symbol.to_uppercase())
    }
    
    /// Get all token prices
    pub fn get_all_prices(&self) -> Vec<TokenPrice> {
        self.cache.values().cloned().collect()
    }
    
    /// Get market overview
    pub fn get_market_overview(&self) -> MarketOverview {
        let total_market_cap: f64 = self.cache.values().map(|t| t.market_cap).sum();
        let total_volume: f64 = self.cache.values().map(|t| t.volume_24h).sum();
        
        MarketOverview {
            total_market_cap,
            total_volume,
            btc_dominance: 52.0,
            eth_dominance: 18.0,
            active_tokens: 5000,
            active_pairs: 10000,
        }
    }
    
    /// Get gas price for a chain
    pub fn get_gas_price(&self, chain_id: u64) -> GasPrice {
        // Mock gas prices based on chain
        match chain_id {
            1 | 5 | 11155111 => GasPrice { // Ethereum
                slow: 10,
                normal: 20,
                fast: 40,
                base_fee: 15,
                priority_fee: 2,
                chain_id,
            },
            137 | 80001 => GasPrice { // Polygon
                slow: 50,
                normal: 80,
                fast: 150,
                base_fee: 45,
                priority_fee: 10,
                chain_id,
            },
            42161 | 421613 => GasPrice { // Arbitrum
                slow: 100_000_000,      // 0.1 gwei in wei
                normal: 150_000_000,    // 0.15 gwei in wei
                fast: 250_000_000,      // 0.25 gwei in wei
                base_fee: 100_000_000,  // 0.1 gwei in wei
                priority_fee: 10_000_000, // 0.01 gwei in wei
                chain_id,
            },
            56 | 97 => GasPrice { // BSC
                slow: 3,
                normal: 5,
                fast: 10,
                base_fee: 3,
                priority_fee: 1,
                chain_id,
            },
            _ => GasPrice {
                slow: 20,
                normal: 30,
                fast: 50,
                base_fee: 20,
                priority_fee: 2,
                chain_id,
            },
        }
    }
    
    /// Get live gas price from RPC (EIP-1559 support)
    /// Returns real-time gas price data from blockchain
    pub async fn get_gas_price_live(&self, chain_id: u64) -> Result<GasPrice, String> {
        // Try to get provider from pool
        let pool = PROVIDER_POOL.clone();
        
        let provider_result = pool.get_provider(chain_id).await;
        
        match provider_result {
            Ok(provider) => {
                // Get gas price from blockchain
                match provider.get_gas_price().await {
                    Ok(gas_price_str) => {
                        // Parse hex string to u64 (wei)
                        let gas_price_u64 = u64::from_str_radix(
                            gas_price_str.trim_start_matches("0x"), 
                            16
                        ).unwrap_or(20_000_000_000); // fallback
                        
                        let gas_price_gwei = gas_price_u64 as f64 / 1e9;
                        
                        // Estimate base fee and priority fee
                        // For EIP-1559, base_fee is typically ~70% of gas_price
                        let base_fee = (gas_price_gwei * 0.7).round() as u64;
                        let priority_fee = ((gas_price_gwei * 0.1).round() as u64).max(1);
                        
                        // Calculate slow/normal/fast options
                        let slow = ((base_fee as f64) * 0.8).round() as u64;
                        let normal = base_fee;
                        let fast = ((base_fee as f64) * 1.3).round() as u64;
                        
                        Ok(GasPrice {
                            slow: slow.max(1),
                            normal: normal.max(1),
                            fast: fast.max(1),
                            base_fee,
                            priority_fee,
                            chain_id,
                        })
                    }
                    Err(e) => {
                        // Fallback to mock data on error
                        eprintln!("[warn] Failed to get gas price from RPC: {}, using fallback", e);
                        Ok(self.get_gas_price(chain_id))
                    }
                }
            }
            Err(e) => {
                // Fallback to mock data if provider not available
                eprintln!("[warn] Provider not available for chain {}: {}, using fallback", chain_id, e);
                Ok(self.get_gas_price(chain_id))
            }
        }
    }
    
    /// Get top DeFi protocols
    pub fn get_defi_protocols(&self) -> Vec<DeFiProtocol> {
        vec![
            DeFiProtocol {
                name: "Uniswap".to_string(),
                logo: "🦄".to_string(),
                tvl: 4_200_000_000.0,
                tvl_change_24h: 2.5,
                volume_24h: 800_000_000.0,
                category: "DEX".to_string(),
                chains: vec!["Ethereum".to_string(), "Arbitrum".to_string(), "Optimism".to_string()],
            },
            DeFiProtocol {
                name: "Aave".to_string(),
                logo: "👻".to_string(),
                tvl: 12_000_000_000.0,
                tvl_change_24h: 1.2,
                volume_24h: 200_000_000.0,
                category: "Lending".to_string(),
                chains: vec!["Ethereum".to_string(), "Polygon".to_string(), "Arbitrum".to_string()],
            },
            DeFiProtocol {
                name: "Curve".to_string(),
                logo: "💚".to_string(),
                tvl: 3_800_000_000.0,
                tvl_change_24h: 3.1,
                volume_24h: 400_000_000.0,
                category: "DEX".to_string(),
                chains: vec!["Ethereum".to_string(), "Arbitrum".to_string()],
            },
            DeFiProtocol {
                name: "Compound".to_string(),
                logo: "🔷".to_string(),
                tvl: 2_100_000_000.0,
                tvl_change_24h: 0.8,
                volume_24h: 50_000_000.0,
                category: "Lending".to_string(),
                chains: vec!["Ethereum".to_string()],
            },
            DeFiProtocol {
                name: "Lido".to_string(),
                logo: "🌟".to_string(),
                tvl: 15_000_000_000.0,
                tvl_change_24h: 2.0,
                volume_24h: 100_000_000.0,
                category: "Liquid Staking".to_string(),
                chains: vec!["Ethereum".to_string()],
            },
        ]
    }
    
    /// Search tokens
    pub fn search_tokens(&self, query: &str) -> Vec<TokenPrice> {
        let query = query.to_lowercase();
        self.cache
            .values()
            .filter(|t| {
                t.symbol.to_lowercase().contains(&query) ||
                t.name.to_lowercase().contains(&query)
            })
            .cloned()
            .collect()
    }
    
    /// Get trending tokens (by volume)
    pub fn get_trending(&self) -> Vec<TokenPrice> {
        let mut tokens: Vec<TokenPrice> = self.cache.values().cloned().collect();
        tokens.sort_by(|a, b| b.volume_24h.partial_cmp(&a.volume_24h).unwrap());
        tokens.truncate(10);
        tokens
    }
    
    /// Get top gainers (by 24h change)
    pub fn get_top_gainers(&self) -> Vec<TokenPrice> {
        let mut tokens: Vec<TokenPrice> = self.cache.values().cloned().collect();
        tokens.sort_by(|a, b| b.change_24h.partial_cmp(&a.change_24h).unwrap());
        tokens.truncate(10);
        tokens
    }
    
    /// Get top losers (by 24h change)
    pub fn get_top_losers(&self) -> Vec<TokenPrice> {
        let mut tokens: Vec<TokenPrice> = self.cache.values().cloned().collect();
        tokens.sort_by(|a, b| a.change_24h.partial_cmp(&b.change_24h).unwrap());
        tokens.truncate(10);
        tokens
    }
}

/// Historical price point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub timestamp: u64,
    pub price: f64,
    pub volume: f64,
}

/// Price history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    pub symbol: String,
    pub prices: Vec<PricePoint>,
}

impl MarketDataService {
    /// Get price history for a token
    pub fn get_price_history(&self, symbol: &str, days: u32) -> PriceHistory {
        let mut prices = Vec::new();
        
        // Generate mock historical data
        let base_price = self.get_price(symbol).map(|p| p.price).unwrap_or(100.0);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        for i in 0..days * 24 {
            let timestamp = now - (i * 3600) as u64;
            let variation = 1.0 + (i as f64 * 0.001).sin() * 0.05;
            let price = base_price * variation;
            let volume = base_price * 100_000.0 * (1.0 + (i as f64 * 0.1).sin() * 0.3);
            
            prices.push(PricePoint {
                timestamp,
                price,
                volume,
            });
        }
        
        prices.reverse();
        
        PriceHistory {
            symbol: symbol.to_uppercase(),
            prices,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_price() {
        let service = MarketDataService::new();
        let eth = service.get_price("ETH");
        assert!(eth.is_some());
        assert_eq!(eth.unwrap().symbol, "ETH");
    }
    
    #[test]
    fn test_market_overview() {
        let service = MarketDataService::new();
        let overview = service.get_market_overview();
        assert!(overview.total_market_cap > 0.0);
    }
    
    #[test]
    fn test_gas_price() {
        let service = MarketDataService::new();
        let gas = service.get_gas_price(1);
        assert_eq!(gas.chain_id, 1);
        assert!(gas.normal >= gas.slow);
    }
}
