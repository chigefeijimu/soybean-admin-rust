//! K-Line (OHLCV) Data Service
//! Provides candlestick chart data for trading pairs

use serde::{Deserialize, Serialize};

/// OHLCV candlestick
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candlestick {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub quote_volume: f64,
}

/// Time period for candlestick
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimePeriod {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "1w")]
    OneWeek,
}

impl TimePeriod {
    pub fn to_seconds(&self) -> u64 {
        match self {
            Self::OneMinute => 60,
            Self::FiveMinutes => 300,
            Self::FifteenMinutes => 900,
            Self::OneHour => 3600,
            Self::FourHours => 14400,
            Self::OneDay => 86400,
            Self::OneWeek => 604800,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1m" => Some(Self::OneMinute),
            "5m" => Some(Self::FiveMinutes),
            "15m" => Some(Self::FifteenMinutes),
            "1h" => Some(Self::OneHour),
            "4h" => Some(Self::FourHours),
            "1d" => Some(Self::OneDay),
            "1w" => Some(Self::OneWeek),
            _ => None,
        }
    }
}

/// Trading pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingPair {
    pub base: String,
    pub quote: String,
    pub address: Option<String>,
    pub chain: String,
}

/// K-Line service
pub struct KLineService {
    max_candlesticks: usize,
}

impl Default for KLineService {
    fn default() -> Self {
        Self::new()
    }
}

impl KLineService {
    pub fn new() -> Self {
        Self {
            max_candlesticks: 1000,
        }
    }

    /// Get historical candlesticks
    pub fn get_candlesticks(
        &self,
        pair: &TradingPair,
        period: TimePeriod,
        limit: usize,
    ) -> Vec<Candlestick> {
        let limit = limit.min(self.max_candlesticks);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let period_secs = period.to_seconds();
        
        // Generate mock candlestick data
        let base_price = self.get_base_price(&pair.base);
        
        (0..limit)
            .map(|i| {
                let timestamp = now - ((limit - i - 1) as u64 * period_secs);
                self.generate_candlestick(timestamp, period_secs, base_price)
            })
            .collect()
    }

    /// Get latest candlestick
    pub fn get_latest(&self, pair: &TradingPair, period: TimePeriod) -> Option<Candlestick> {
        let candlesticks = self.get_candlesticks(pair, period, 1);
        candlesticks.into_iter().next()
    }

    /// Get real-time price (latest close)
    pub fn get_price(&self, symbol: &str) -> f64 {
        self.get_base_price(symbol)
    }

    /// Get 24h price change
    pub fn get_24h_change(&self, symbol: &str) -> (f64, f64) {
        let price = self.get_base_price(symbol);
        let change_percent = (rand_simple() - 0.5) * 10.0; // -5% to +5%
        let change = price * change_percent / 100.0;
        (change, change_percent)
    }

    /// Get base price for symbol
    fn get_base_price(&self, symbol: &str) -> f64 {
        match symbol.to_uppercase().as_str() {
            "ETH" => 2500.0,
            "BTC" | "WBTC" => 62500.0,
            "USDC" | "USDT" | "DAI" => 1.0,
            "SOL" => 120.0,
            "UNI" => 8.5,
            "AAVE" => 85.0,
            "LINK" => 15.0,
            "MATIC" => 0.85,
            "ARB" => 1.1,
            "OP" => 2.5,
            "BNB" => 580.0,
            _ => 100.0,
        }
    }

    /// Generate a mock candlestick
    fn generate_candlestick(&self, timestamp: u64, _period_secs: u64, base_price: f64) -> Candlestick {
        // Add some randomness to simulate price movement
        let variation = rand_simple() * 0.02 - 0.01; // -1% to +1%
        let open = base_price * (1.0 + variation);
        
        let high_variation = (rand_simple() * 0.02).abs();
        let low_variation = (rand_simple() * 0.02).abs();
        
        let high = open * (1.0 + high_variation);
        let low = open * (1.0 - low_variation);
        
        let close = if rand_simple() > 0.5 {
            low + (high - low) * rand_simple()
        } else {
            open + (open * 0.005 * (rand_simple() - 0.5))
        };
        
        let close = close.max(low).min(high);
        
        let volume = 1000.0 + rand_simple() * 10000.0;
        let quote_volume = volume * (open + close) / 2.0;
        
        Candlestick {
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            quote_volume,
        }
    }
}

/// Simple random 0-1
fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 % 1000.0) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candlesticks() {
        let service = KLineService::new();
        let pair = TradingPair {
            base: "ETH".to_string(),
            quote: "USDC".to_string(),
            address: None,
            chain: "ethereum".to_string(),
        };
        
        let candles = service.get_candlesticks(&pair, TimePeriod::OneHour, 100);
        assert_eq!(candles.len(), 100);
    }
}
