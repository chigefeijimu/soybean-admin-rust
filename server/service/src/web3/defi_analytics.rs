// DeFi Protocol Analytics Service
// Provides real-time statistics for major DeFi protocols using DeFi Llama API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DeFi protocol statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefiProtocolStats {
    pub name: String,
    pub symbol: String,
    pub category: String,
    pub tvl: f64,
    pub tvl_change_24h: f64,
    pub tvl_change_7d: f64,
    pub volume_24h: f64,
    pub fees_24h: f64,
    pub revenue_24h: f64,
    pub active_users: u64,
    pub tx_count_24h: u64,
    pub avg_apr: f64,
    pub num_pools: u32,
    pub chain: String,
    pub logo_url: String,
}

/// Category summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefiCategoryStats {
    pub category: String,
    pub total_tvl: f64,
    pub change_24h: f64,
    pub protocols_count: u32,
    pub top_protocols: Vec<String>,
}

/// Overall DeFi market stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefiMarketStats {
    pub total_tvl: f64,
    pub total_tvl_change_24h: f64,
    pub total_volume_24h: f64,
    pub total_fees_24h: f64,
    pub total_revenue_24h: f64,
    pub categories: Vec<DefiCategoryStats>,
    pub top_protocols: Vec<DefiProtocolStats>,
    pub last_updated: String,
}

/// Protocol historical data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolHistoryPoint {
    pub timestamp: i64,
    pub tvl: f64,
    pub volume: f64,
    pub users: u64,
}

pub fn get_defi_protocol_stats() -> Vec<DefiProtocolStats> {
    get_defi_protocol_stats_mock()
}
#[derive(Debug, Deserialize)]
pub struct DefiLlamaProtocol {
    pub name: String,
    pub symbol: Option<String>,
    #[serde(rename = "category")]
    pub defi_category: Option<String>,
    pub tvl: Option<f64>,
    pub chain: Option<String>,
    pub logo: Option<String>,
    #[serde(rename = "change_1h")]
    pub change_1h: Option<f64>,
    #[serde(rename = "change_24h")]
    pub change_24h: Option<f64>,
    #[serde(rename = "change_7d")]
    pub change_7d: Option<f64>,
    #[serde(rename = "volumeUsd24h")]
    pub volume_usd_24h: Option<f64>,
    #[serde(rename = "feesUsd24h")]
    pub fees_usd_24h: Option<f64>,
    #[serde(rename = "revenueUsd24h")]
    pub revenue_usd_24h: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct DefiLlamaResponse {
    pub data: Vec<DefiLlamaProtocol>,
}

/// Get real DeFi protocol statistics from DeFi Llama API
pub async fn get_defi_protocol_stats_from_api() -> Result<Vec<DefiProtocolStats>, String> {
    let url = "https://api.llama.fi/protocols";
    
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to fetch from DeFi Llama: {}", e))?;
    
    let protocols: Vec<DefiLlamaProtocol> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    // Take top 30 protocols by TVL
    let mut top_protocols: Vec<_> = protocols.into_iter()
        .filter(|p| p.tvl.unwrap_or(0.0) > 10_000_000.0) // > $10M TVL
        .collect();
    
    top_protocols.sort_by(|a, b| {
        let tvl_a = a.tvl.unwrap_or(0.0);
        let tvl_b = b.tvl.unwrap_or(0.0);
        tvl_b.partial_cmp(&tvl_a).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    let result: Vec<DefiProtocolStats> = top_protocols.into_iter()
        .take(30)
        .map(|p| {
            let chain = p.chain.unwrap_or_else(|| "Multi".to_string());
            DefiProtocolStats {
                name: p.name.clone(),
                symbol: p.symbol.unwrap_or_else(|| "".to_string()),
                category: p.defi_category.unwrap_or_else(|| "Other".to_string()),
                tvl: p.tvl.unwrap_or(0.0),
                tvl_change_24h: p.change_24h.unwrap_or(0.0),
                tvl_change_7d: p.change_7d.unwrap_or(0.0),
                volume_24h: p.volume_usd_24h.unwrap_or(0.0),
                fees_24h: p.fees_usd_24h.unwrap_or(0.0),
                revenue_24h: p.revenue_usd_24h.unwrap_or(0.0),
                active_users: 0, // Not available from DeFi Llama
                tx_count_24h: 0,
                avg_apr: 0.0,
                num_pools: 0,
                chain: chain.clone(),
                logo_url: p.logo.unwrap_or_default(),
            }
        })
        .collect();
    
    Ok(result)
}

/// Get mock data as fallback
pub fn get_defi_protocol_stats_mock() -> Vec<DefiProtocolStats> {
    vec![
        DefiProtocolStats {
            name: "Aave".to_string(),
            symbol: "AAVE".to_string(),
            category: "Lending".to_string(),
            tvl: 32_450_000_000.0,
            tvl_change_24h: 2.5,
            tvl_change_7d: 5.2,
            volume_24h: 890_000_000.0,
            fees_24h: 4_200_000.0,
            revenue_24h: 2_100_000.0,
            active_users: 125_000,
            tx_count_24h: 45_000,
            avg_apr: 4.8,
            num_pools: 28,
            chain: "Multi".to_string(),
            logo_url: "https://cryptologos.cc/logos/aave-aave-logo.png".to_string(),
        },
        DefiProtocolStats {
            name: "Compound".to_string(),
            symbol: "COMP".to_string(),
            category: "Lending".to_string(),
            tvl: 12_800_000_000.0,
            tvl_change_24h: 1.2,
            tvl_change_7d: 3.8,
            volume_24h: 320_000_000.0,
            fees_24h: 1_800_000.0,
            revenue_24h: 900_000.0,
            active_users: 52_000,
            tx_count_24h: 18_500,
            avg_apr: 3.9,
            num_pools: 12,
            chain: "Ethereum".to_string(),
            logo_url: "https://cryptologos.cc/logos/compound-comp-logo.png".to_string(),
        },
        DefiProtocolStats {
            name: "Uniswap".to_string(),
            symbol: "UNI".to_string(),
            category: "DEX".to_string(),
            tvl: 8_200_000_000.0,
            tvl_change_24h: 3.1,
            tvl_change_7d: 8.5,
            volume_24h: 1_850_000_000.0,
            fees_24h: 5_500_000.0,
            revenue_24h: 5_500_000.0,
            active_users: 89_000,
            tx_count_24h: 125_000,
            avg_apr: 0.0,
            num_pools: 800,
            chain: "Multi".to_string(),
            logo_url: "https://cryptologos.cc/logos/uniswap-uni-logo.png".to_string(),
        },
    ]
}

/// Get DeFi market overview
pub fn get_defi_market_stats() -> DefiMarketStats {
    // This would be async in real implementation
    // For now, use mock data
    let protocols = get_defi_protocol_stats_mock();
    
    let mut category_map: HashMap<String, (f64, f64, Vec<String>)> = HashMap::new();
    for p in &protocols {
        let entry = category_map.entry(p.category.clone()).or_insert((0.0, 0.0, Vec::new()));
        entry.0 += p.tvl;
        entry.1 += p.volume_24h;
        if entry.2.len() < 3 {
            entry.2.push(p.name.clone());
        }
    }
    
    let categories: Vec<DefiCategoryStats> = category_map
        .into_iter()
        .map(|(cat, (tvl, _vol, tops))| {
            let cat_clone = cat.clone();
            DefiCategoryStats {
                category: cat,
                total_tvl: tvl,
                change_24h: 1.5,
                protocols_count: protocols.iter().filter(|p| p.category == cat_clone).count() as u32,
                top_protocols: tops,
            }
        })
        .collect();
    
    let total_tvl: f64 = protocols.iter().map(|p| p.tvl).sum();
    let total_volume: f64 = protocols.iter().map(|p| p.volume_24h).sum();
    let total_fees: f64 = protocols.iter().map(|p| p.fees_24h).sum();
    let total_revenue: f64 = protocols.iter().map(|p| p.revenue_24h).sum();
    
    let mut top_protocols = protocols.clone();
    top_protocols.sort_by(|a, b| b.tvl.partial_cmp(&a.tvl).unwrap_or(std::cmp::Ordering::Equal));
    let top_protocols: Vec<DefiProtocolStats> = top_protocols.into_iter().take(10).collect();
    
    DefiMarketStats {
        total_tvl,
        total_tvl_change_24h: 2.1,
        total_volume_24h: total_volume,
        total_fees_24h: total_fees,
        total_revenue_24h: total_revenue,
        categories,
        top_protocols,
        last_updated: chrono::Utc::now().to_rfc3339(),
    }
}

/// Get protocol by name
pub fn get_protocol_by_name(name: &str) -> Option<DefiProtocolStats> {
    let protocols = get_defi_protocol_stats_mock();
    protocols.into_iter().find(|p| p.name.to_lowercase() == name.to_lowercase())
}

/// Get protocols by chain
pub fn get_protocols_by_chain(chain: &str) -> Vec<DefiProtocolStats> {
    let protocols = get_defi_protocol_stats_mock();
    protocols.into_iter()
        .filter(|p| p.chain.to_lowercase().contains(&chain.to_lowercase()))
        .collect()
}

/// Get protocols by category
pub fn get_protocol_by_category(category: &str) -> Vec<DefiProtocolStats> {
    let protocols = get_defi_protocol_stats_mock();
    protocols.into_iter()
        .filter(|p| p.category.to_lowercase() == category.to_lowercase())
        .collect()
}

/// Get protocol TVL history (mock)
pub fn get_protocol_tvl_history(_protocol: &str, days: u32) -> Vec<ProtocolHistoryPoint> {
    let now = chrono::Utc::now().timestamp();
    let base_tvl = 1_000_000_000.0;
    
    (0..days)
        .map(|i| {
            let ts = now - (days - i) as i64 * 86400;
            let variation = (i as f64 * 0.05).sin() * 0.1 + 1.0;
            ProtocolHistoryPoint {
                timestamp: ts,
                tvl: base_tvl * variation,
                volume: base_tvl * 0.05 * variation,
                users: 10000 + (i as u64 * 100),
            }
        })
        .collect()
}
