//! Technical Indicators for Trading
//! Implements common indicators: MA, EMA, RSI, MACD, Bollinger Bands

use serde::{Deserialize, Serialize};
use crate::web3::kline::Candlestick;

/// Moving Average
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingAverage {
    pub period: u32,
    pub value: f64,
    pub timestamp: u64,
}

/// RSI (Relative Strength Index)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RsiData {
    pub value: f64,
    pub overbought: bool,
    pub oversold: bool,
}

/// MACD (Moving Average Convergence Divergence)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacdData {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

/// Bollinger Bands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BollingerBands {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    pub bandwidth: f64,
}

/// VWAP (Volume Weighted Average Price)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VwapData {
    pub value: f64,
    pub typical_price: f64,
    pub volume: f64,
}

/// ATR (Average True Range)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtrData {
    pub value: f64,
    pub high: f64,
    pub low: f64,
    pub volatility: String,
}

/// Technical Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalAnalysis {
    pub ma: Vec<MovingAverage>,
    pub rsi: Option<RsiData>,
    pub macd: Option<MacdData>,
    pub bollinger: Option<BollingerBands>,
    pub vwap: Option<VwapData>,
    pub atr: Option<AtrData>,
    pub trend: String,
    pub signal: String,
}

/// Technical Indicator Service
pub struct IndicatorService;

impl Default for IndicatorService {
    fn default() -> Self {
        Self::new()
    }
}

impl IndicatorService {
    pub fn new() -> Self {
        Self
    }

    /// Calculate Simple Moving Average (SMA)
    pub fn sma(candles: &[Candlestick], period: u32) -> Vec<MovingAverage> {
        if candles.len() < period as usize {
            return vec![];
        }
        
        candles
            .windows(period as usize)
            .map(|window| {
                let sum: f64 = window.iter().map(|c| c.close).sum();
                let value = sum / period as f64;
                MovingAverage {
                    period,
                    value,
                    timestamp: window.last().map(|c| c.timestamp).unwrap_or(0),
                }
            })
            .collect()
    }

    /// Calculate Exponential Moving Average (EMA)
    pub fn ema(candles: &[Candlestick], period: u32) -> Vec<MovingAverage> {
        if candles.len() < period as usize {
            return vec![];
        }
        
        let multiplier = 2.0 / (period as f64 + 1.0);
        let mut ema_values = Vec::new();
        
        // First EMA is SMA
        let first_sma: f64 = candles.iter().take(period as usize)
            .map(|c| c.close)
            .sum::<f64>() / period as f64;
        
        ema_values.push(MovingAverage {
            period,
            value: first_sma,
            timestamp: candles[(period - 1) as usize].timestamp,
        });
        
        // Calculate subsequent EMAs
        for candle in candles.iter().skip(period as usize) {
            let close = candle.close;
            let prev_ema = ema_values.last().unwrap().value;
            let ema = (close - prev_ema) * multiplier + prev_ema;
            
            ema_values.push(MovingAverage {
                period,
                value: ema,
                timestamp: candle.timestamp,
            });
        }
        
        ema_values
    }

    /// Calculate RSI
    pub fn rsi(candles: &[Candlestick], period: u32) -> Option<RsiData> {
        if candles.len() < (period + 1) as usize {
            return None;
        }
        
        let mut gains = 0.0;
        let mut losses = 0.0;
        
        for i in 1..candles.len() {
            let change = candles[i].close - candles[i - 1].close;
            if change > 0.0 {
                gains += change;
            } else {
                losses -= change;
            }
        }
        
        let avg_gain = gains / period as f64;
        let avg_loss = losses / period as f64;
        
        if avg_loss == 0.0 {
            return Some(RsiData {
                value: 100.0,
                overbought: true,
                oversold: false,
            });
        }
        
        let rs = avg_gain / avg_loss;
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        
        Some(RsiData {
            value: rsi,
            overbought: rsi >= 70.0,
            oversold: rsi <= 30.0,
        })
    }

    /// Calculate MACD
    pub fn macd(candles: &[Candlestick]) -> Option<MacdData> {
        if candles.len() < 34 {
            return None;
        }
        
        let ema_12 = Self::ema(candles, 12);
        let ema_26 = Self::ema(candles, 26);
        
        if ema_12.is_empty() || ema_26.is_empty() {
            return None;
        }
        
        let macd_line: Vec<f64> = ema_12.iter()
            .zip(ema_26.iter())
            .map(|(e12, e26)| e12.value - e26.value)
            .collect();
        
        // Signal line is 9-period EMA of MACD
        let signal_start = macd_line.len().saturating_sub(9);
        if signal_start == 0 {
            return None;
        }
        
        let signal_values = &macd_line[signal_start..];
        let signal_sum: f64 = signal_values.iter().sum();
        let signal = signal_sum / 9.0;
        
        let macd = *macd_line.last().unwrap_or(&0.0);
        let histogram = macd - signal;
        
        Some(MacdData {
            macd,
            signal,
            histogram,
        })
    }

    /// Calculate Bollinger Bands
    pub fn bollinger_bands(candles: &[Candlestick], period: u32, std_dev: f64) -> Option<BollingerBands> {
        if candles.len() < period as usize {
            return None;
        }
        
        let sma = Self::sma(candles, period);
        let middle = sma.last()?.value;
        
        let window = &candles[candles.len() - period as usize..];
        let variance: f64 = window.iter()
            .map(|c| (c.close - middle).powi(2))
            .sum::<f64>() / period as f64;
        
        let std = variance.sqrt();
        
        let upper = middle + std_dev * std;
        let lower = middle - std_dev * std;
        let bandwidth = (upper - lower) / middle * 100.0;
        
        Some(BollingerBands {
            upper,
            middle,
            lower,
            bandwidth,
        })
    }

    /// Calculate VWAP (Volume Weighted Average Price)
    pub fn vwap(candles: &[Candlestick]) -> Option<VwapData> {
        if candles.is_empty() {
            return None;
        }

        let mut cumulative_tpv = 0.0; // cumulative typical price * volume
        let mut cumulative_volume = 0.0;

        for candle in candles {
            let typical_price = (candle.high + candle.low + candle.close) / 3.0;
            cumulative_tpv += typical_price * candle.volume;
            cumulative_volume += candle.volume;
        }

        if cumulative_volume == 0.0 {
            return None;
        }

        let vwap_value = cumulative_tpv / cumulative_volume;
        let last_candle = candles.last().unwrap();
        let typical_price = (last_candle.high + last_candle.low + last_candle.close) / 3.0;

        Some(VwapData {
            value: vwap_value,
            typical_price,
            volume: cumulative_volume,
        })
    }

    /// Calculate ATR (Average True Range)
    pub fn atr(candles: &[Candlestick], period: u32) -> Option<AtrData> {
        if candles.len() < (period + 1) as usize {
            return None;
        }

        let mut true_ranges: Vec<f64> = Vec::new();

        for i in 1..candles.len() {
            let high = candles[i].high;
            let low = candles[i].low;
            let prev_close = candles[i - 1].close;

            let tr = (high - low).max((high - prev_close).abs()).max((low - prev_close).abs());
            true_ranges.push(tr);
        }

        if true_ranges.len() < period as usize {
            return None;
        }

        // Calculate ATR using Wilder's smoothing method
        let mut atr_value = true_ranges.iter().take(period as usize).sum::<f64>() / period as f64;

        for tr in true_ranges.iter().skip(period as usize) {
            atr_value = ((period as f64 - 1.0) * atr_value + tr) / period as f64;
        }

        let last_candle = candles.last().unwrap();
        let volatility = if atr_value > last_candle.close * 0.03 {
            "high".to_string()
        } else if atr_value > last_candle.close * 0.01 {
            "medium".to_string()
        } else {
            "low".to_string()
        };

        Some(AtrData {
            value: atr_value,
            high: last_candle.high,
            low: last_candle.low,
            volatility,
        })
    }

    /// Full technical analysis
    pub fn analyze(&self, candles: &[Candlestick]) -> TechnicalAnalysis {
        let ma5 = Self::sma(candles, 5);
        let ma20 = Self::sma(candles, 20);
        let ma50 = Self::sma(candles, 50);
        
        // Determine trend before moving ma5 and ma20
        let trend = if let (Some(ma5_last), Some(ma20_last)) = (ma5.last(), ma20.last()) {
            if ma5_last.value > ma20_last.value {
                "bullish".to_string()
            } else {
                "bearish".to_string()
            }
        } else {
            "neutral".to_string()
        };
        
        let mut ma = Vec::new();
        ma.extend(ma5);
        ma.extend(ma20);
        ma.extend(ma50);
        
        let rsi = Self::rsi(candles, 14);
        let macd = Self::macd(candles);
        let bollinger = Self::bollinger_bands(candles, 20, 2.0);
        let vwap = Self::vwap(candles);
        let atr = Self::atr(candles, 14);

        // Determine signal
        let mut signal = "neutral".to_string();
        if let Some(rsi_data) = &rsi {
            if rsi_data.oversold {
                signal = "strong_buy".to_string();
            } else if rsi_data.overbought {
                signal = "strong_sell".to_string();
            }
        }
        
        if let Some(macd_data) = &macd {
            if macd_data.histogram > 0.0 && signal == "neutral" {
                signal = "buy".to_string();
            } else if macd_data.histogram < 0.0 && signal == "neutral" {
                signal = "sell".to_string();
            }
        }
        
        TechnicalAnalysis {
            ma,
            rsi,
            macd,
            bollinger,
            vwap,
            atr,
            trend,
            signal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web3::kline::{KLineService, TradingPair, TimePeriod};

    #[test]
    fn test_sma() {
        let service = KLineService::new();
        let pair = TradingPair {
            base: "ETH".to_string(),
            quote: "USDC".to_string(),
            address: None,
            chain: "ethereum".to_string(),
        };
        
        let candles = service.get_candlesticks(&pair, TimePeriod::OneHour, 50);
        let sma = IndicatorService::sma(&candles, 5);
        
        assert!(!sma.is_empty());
    }

    #[test]
    fn test_rsi() {
        let service = KLineService::new();
        let pair = TradingPair {
            base: "ETH".to_string(),
            quote: "USDC".to_string(),
            address: None,
            chain: "ethereum".to_string(),
        };
        
        let candles = service.get_candlesticks(&pair, TimePeriod::OneHour, 50);
        let rsi = IndicatorService::rsi(&candles, 14);
        
        assert!(rsi.is_some());
    }
}
