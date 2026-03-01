//! Real Price Fetcher Service
//! Fetches real prices from CoinGecko API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Real price data from API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealPriceData {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub change_24h: f64,
    pub change_percent_24h: f64,
    pub market_cap: f64,
    pub volume_24h: f64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub last_updated: String,
}

/// Price API response
#[derive(Debug, Deserialize)]
struct CoinGeckoPrice {
    #[serde(rename = "usd")]
    price: f64,
    #[serde(rename = "usd_24h_change")]
    change_24h: Option<f64>,
    #[serde(rename = "usd_market_cap")]
    market_cap: Option<f64>,
    #[serde(rename = "usd_24h_vol")]
    volume_24h: Option<f64>,
}

/// Coin info
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct CoinGeckoCoin {
    id: String,
    symbol: String,
    name: String,
    image: Option<HashMap<String, String>>,
}

/// Price Service with real API
pub struct PriceService {
    base_url: String,
    coin_map: HashMap<String, String>, // symbol -> coinGecko id
}

impl PriceService {
    pub fn new() -> Self {
        let mut coin_map = HashMap::new();
        coin_map.insert("ETH".to_string(), "ethereum".to_string());
        coin_map.insert("BTC".to_string(), "bitcoin".to_string());
        coin_map.insert("SOL".to_string(), "solana".to_string());
        coin_map.insert("BNB".to_string(), "binancecoin".to_string());
        coin_map.insert("XRP".to_string(), "ripple".to_string());
        coin_map.insert("ADA".to_string(), "cardano".to_string());
        coin_map.insert("DOGE".to_string(), "dogecoin".to_string());
        coin_map.insert("DOT".to_string(), "polkadot".to_string());
        coin_map.insert("MATIC".to_string(), "matic-network".to_string());
        coin_map.insert("LINK".to_string(), "chainlink".to_string());
        coin_map.insert("UNI".to_string(), "uniswap".to_string());
        coin_map.insert("AAVE".to_string(), "aave".to_string());
        coin_map.insert("MKR".to_string(), "maker".to_string());
        coin_map.insert("CRV".to_string(), "curve-dao-token".to_string());
        coin_map.insert("LDO".to_string(), "lido-dao".to_string());
        
        Self {
            base_url: "https://api.coingecko.com/api/v3".to_string(),
            coin_map,
        }
    }

    /// Get real price from CoinGecko
    pub async fn get_price(&self, symbol: &str) -> Result<RealPriceData, String> {
        let coin_id = self.coin_map.get(&symbol.to_uppercase())
            .ok_or_else(|| format!("Unsupported symbol: {}", symbol))?;
        
        let url = format!(
            "{}/simple/price?ids={}&vs_currencies=usd&include_24hr_change=true&include_24hr_vol=true&include_market_cap=true",
            self.base_url,
            coin_id
        );
        
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()));
        }
        
        let data: HashMap<String, CoinGeckoPrice> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let coin_data = data.get(coin_id)
            .ok_or_else(|| format!("No data for coin: {}", coin_id))?;
        
        Ok(RealPriceData {
            symbol: symbol.to_uppercase(),
            name: coin_id.replace('-', " ").to_title_case(),
            price: coin_data.price,
            change_24h: coin_data.change_24h.unwrap_or(0.0) * coin_data.price / 100.0,
            change_percent_24h: coin_data.change_24h.unwrap_or(0.0),
            market_cap: coin_data.market_cap.unwrap_or(0.0),
            volume_24h: coin_data.volume_24h.unwrap_or(0.0),
            high_24h: coin_data.price * 1.02, // Estimate
            low_24h: coin_data.price * 0.98,   // Estimate
            last_updated: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// Get multiple prices
    pub async fn get_prices(&self, symbols: &[&str]) -> HashMap<String, Result<RealPriceData, String>> {
        let mut results = HashMap::new();
        
        for symbol in symbols {
            results.insert(symbol.to_string(), self.get_price(symbol).await);
        }
        
        results
    }

    /// Get top coins by market cap
    pub async fn get_top_coins(&self, limit: usize) -> Result<Vec<RealPriceData>, String> {
        let url = format!(
            "{}/coins/markets?vs_currency=usd&order=market_cap_desc&per_page={}&page=1&sparkline=false",
            self.base_url,
            limit
        );
        
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()));
        }
        
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct MarketCoin {
            #[allow(dead_code)]
            id: String,
            symbol: String,
            name: String,
            current_price: f64,
            price_change_24h: Option<f64>,
            price_change_percentage_24h: Option<f64>,
            market_cap: Option<f64>,
            total_volume: Option<f64>,
            high_24h: Option<f64>,
            low_24h: Option<f64>,
        }
        
        let data: Vec<MarketCoin> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        Ok(data.into_iter().map(|coin| RealPriceData {
            symbol: coin.symbol.to_uppercase(),
            name: coin.name,
            price: coin.current_price,
            change_24h: coin.price_change_24h.unwrap_or(0.0),
            change_percent_24h: coin.price_change_percentage_24h.unwrap_or(0.0),
            market_cap: coin.market_cap.unwrap_or(0.0),
            volume_24h: coin.total_volume.unwrap_or(0.0),
            high_24h: coin.high_24h.unwrap_or(coin.current_price),
            low_24h: coin.low_24h.unwrap_or(coin.current_price),
            last_updated: chrono::Utc::now().to_rfc3339(),
        }).collect())
    }

    /// Search coins
    pub async fn search_coins(&self, query: &str) -> Result<Vec<CoinSearchResult>, String> {
        let url = format!("{}/search?query={}", self.base_url, query);
        
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Accept", "application/json")
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("API returned status: {}", response.status()));
        }
        
        #[derive(Deserialize)]
        struct SearchResponse {
            coins: Vec<SearchCoin>,
        }
        
        #[derive(Deserialize)]
        struct SearchCoin {
            id: String,
            name: String,
            symbol: String,
            thumb: String,
            market_cap_rank: Option<i32>,
        }
        
        let data: SearchResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        Ok(data.coins.into_iter().map(|coin| CoinSearchResult {
            id: coin.id,
            name: coin.name,
            symbol: coin.symbol.to_uppercase(),
            thumb: coin.thumb,
            rank: coin.market_cap_rank.unwrap_or(0),
        }).collect())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinSearchResult {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub thumb: String,
    pub rank: i32,
}

// Helper for title case
trait ToTitleCase {
    fn to_title_case(&self) -> String;
}

impl ToTitleCase for String {
    fn to_title_case(&self) -> String {
        let mut result = String::new();
        for (i, word) in self.split('-').enumerate() {
            if i > 0 {
                result.push(' ');
            }
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                result.push(first.to_ascii_uppercase());
                result.extend(chars);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_map() {
        let service = PriceService::new();
        assert!(service.coin_map.contains_key("ETH"));
        assert!(service.coin_map.contains_key("BTC"));
    }

    #[tokio::test]
    #[ignore] // Requires network
    async fn test_get_price() {
        let service = PriceService::new();
        let result = service.get_price("ETH").await;
        println!("Result: {:?}", result);
        assert!(result.is_ok());
    }
}
