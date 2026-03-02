// DeFi Protocol Analytics Service
// Provides real-time statistics for major DeFi protocols

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

/// Get mock DeFi protocol statistics
pub fn get_defi_protocol_stats() -> Vec<DefiProtocolStats> {
    let now = chrono::Utc::now().timestamp();
    
    vec![
        // Lending
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
        // DEX
        DefiProtocolStats {
            name: "Uniswap".to_string(),
            symbol: "UNI".to_string(),
            category: "DEX".to_string(),
            tvl: 8_750_000_000.0,
            tvl_change_24h: -1.5,
            tvl_change_7d: 2.1,
            volume_24h: 2_150_000_000.0,
            fees_24h: 6_450_000.0,
            revenue_24h: 6_450_000.0,
            active_users: 189_000,
            tx_count_24h: 320_000,
            avg_apr: 0.0,
            num_pools: 850,
            chain: "Multi".to_string(),
            logo_url: "https://cryptologos.cc/logos/uniswap-uni-logo.png".to_string(),
        },
        DefiProtocolStats {
            name: "Curve".to_string(),
            symbol: "CRV".to_string(),
            category: "DEX".to_string(),
            tvl: 4_250_000_000.0,
            tvl_change_24h: 0.8,
            tvl_change_7d: 1.9,
            volume_24h: 890_000_000.0,
            fees_24h: 2_670_000.0,
            revenue_24h: 2_670_000.0,
            active_users: 45_000,
            tx_count_24h: 85_000,
            avg_apr: 0.0,
            num_pools: 450,
            chain: "Multi".to_string(),
            logo_url: "https://cryptologos.cc/logos/curve-dao-token-crv-logo.png".to_string(),
        },
        DefiProtocolStats {
            name: "SushiSwap".to_string(),
            symbol: "SUSHI".to_string(),
            category: "DEX".to_string(),
            tvl: 1_850_000_000.0,
            tvl_change_24h: -2.3,
            tvl_change_7d: -4.5,
            volume_24h: 280_000_000.0,
            fees_24h: 840_000.0,
            revenue_24h: 840_000.0,
            active_users: 28_000,
            tx_count_24h: 42_000,
            avg_apr: 0.0,
            num_pools: 380,
            chain: "Multi".to_string(),
            logo_url: "https://cryptologos.cc/logos/sushi-sushi-logo.png".to_string(),
        },
        // Liquid Staking
        DefiProtocolStats {
            name: "Lido".to_string(),
            symbol: "LDO".to_string(),
            category: "Liquid Staking".to_string(),
            tvl: 42_800_000_000.0,
            tvl_change_24h: 3.2,
            tvl_change_7d: 8.5,
            volume_24h: 185_000_000.0,
            fees_24h: 3_700_000.0,
            revenue_24h: 1_850_000.0,
            active_users: 95_000,
            tx_count_24h: 28_000,
            avg_apr: 3.2,
            num_pools: 5,
            chain: "Multi".to_string(),
            logo_url: "https://cryptologos.cc/logos/lido-dao-ldo-logo.png".to_string(),
        },
        DefiProtocolStats {
            name: "Rocket Pool".to_string(),
            symbol: "RPL".to_string(),
            category: "Liquid Staking".to_string(),
            tvl: 2_850_000_000.0,
            tvl_change_24h: 4.1,
            tvl_change_7d: 10.2,
            volume_24h: 45_000_000.0,
            fees_24h: 450_000.0,
            revenue_24h: 225_000.0,
            active_users: 12_000,
            tx_count_24h: 4_200,
            avg_apr: 3.8,
            num_pools: 3,
            chain: "Ethereum".to_string(),
            logo_url: "https://cryptologos.cc/logos/rocket-pool-rpl-logo.png".to_string(),
        },
        // Yield
        DefiProtocolStats {
            name: "Yearn".to_string(),
            symbol: "YFI".to_string(),
            category: "Yield".to_string(),
            tvl: 5_200_000_000.0,
            tvl_change_24h: 1.8,
            tvl_change_7d: 4.2,
            volume_24h: 120_000_000.0,
            fees_24h: 1_200_000.0,
            revenue_24h: 600_000.0,
            active_users: 18_000,
            tx_count_24h: 8_500,
            avg_apr: 8.5,
            num_pools: 120,
            chain: "Multi".to_string(),
            logo_url: "https://cryptologos.cc/logos/yearn-finance-yfi-logo.png".to_string(),
        },
        // Bridge
        DefiProtocolStats {
            name: "LayerZero".to_string(),
            symbol: "OFT".to_string(),
            category: "Bridge".to_string(),
            tvl: 8_500_000_000.0,
            tvl_change_24h: 5.2,
            tvl_change_7d: 12.8,
            volume_24h: 450_000_000.0,
            fees_24h: 2_250_000.0,
            revenue_24h: 1_125_000.0,
            active_users: 156_000,
            tx_count_24h: 185_000,
            avg_apr: 0.0,
            num_pools: 0,
            chain: "Multi".to_string(),
            logo_url: "https://cryptologos.cc/logos/layer-zero-logo.png".to_string(),
        },
        DefiProtocolStats {
            name: "Axelar".to_string(),
            symbol: "AXL".to_string(),
            category: "Bridge".to_string(),
            tvl: 1_250_000_000.0,
            tvl_change_24h: 2.8,
            tvl_change_7d: 6.5,
            volume_24h: 85_000_000.0,
            fees_24h: 425_000.0,
            revenue_24h: 212_000.0,
            active_users: 32_000,
            tx_count_24h: 42_000,
            avg_apr: 0.0,
            num_pools: 0,
            chain: "Multi".to_string(),
            logo_url: "https://cryptologos.cc/logos/axelar-axl-logo.png".to_string(),
        },
        // Derivatives
        DefiProtocolStats {
            name: "GMX".to_string(),
            symbol: "GMX".to_string(),
            category: "Derivatives".to_string(),
            tvl: 680_000_000.0,
            tvl_change_24h: -3.5,
            tvl_change_7d: -8.2,
            volume_24h: 1_250_000_000.0,
            fees_24h: 3_750_000.0,
            revenue_24h: 3_750_000.0,
            active_users: 42_000,
            tx_count_24h: 125_000,
            avg_apr: 0.0,
            num_pools: 8,
            chain: "Multi".to_string(),
            logo_url: "https://cryptologos.cc/logos/gmx-gmx-logo.png".to_string(),
        },
        // Insurance
        DefiProtocolStats {
            name: "Nexus Mutual".to_string(),
            symbol: "NXM".to_string(),
            category: "Insurance".to_string(),
            tvl: 450_000_000.0,
            tvl_change_24h: 0.5,
            tvl_change_7d: 1.8,
            volume_24h: 8_500_000.0,
            fees_24h: 85_000.0,
            revenue_24h: 42_000.0,
            active_users: 8_500,
            tx_count_24h: 1_200,
            avg_apr: 0.0,
            num_pools: 25,
            chain: "Ethereum".to_string(),
            logo_url: "https://cryptologos.cc/logos/nexus-mutual-nxm-logo.png".to_string(),
        },
    ]
}

/// Get DeFi market overview
pub fn get_defi_market_stats() -> DefiMarketStats {
    let protocols = get_defi_protocol_stats();
    
    // Calculate category stats
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
        .map(|(cat, (tvl, vol, tops))| DefiCategoryStats {
            category: cat,
            total_tvl: tvl,
            change_24h: 1.5, // Mock
            protocols_count: protocols.iter().filter(|p| p.category == cat).count() as u32,
            top_protocols: tops,
        })
        .collect();
    
    // Calculate totals
    let total_tvl: f64 = protocols.iter().map(|p| p.tvl).sum();
    let total_volume: f64 = protocols.iter().map(|p| p.volume_24h).sum();
    let total_fees: f64 = protocols.iter().map(|p| p.fees_24h).sum();
    let total_revenue: f64 = protocols.iter().map(|p| p.revenue_24h).sum();
    
    // Get top protocols by TVL
    let mut sorted_protocols = protocols.clone();
    sorted_protocols.sort_by(|a, b| b.tvl.partial_cmp(&a.tvl).unwrap());
    let top_protocols: Vec<DefiProtocolStats> = sorted_protocols.into_iter().take(10).collect();
    
    DefiMarketStats {
        total_tvl,
        total_tvl_change_24h: 1.8,
        total_volume_24h: total_volume,
        total_fees_24h: total_fees,
        total_revenue_24h: total_revenue,
        categories,
        top_protocols,
        last_updated: chrono::Utc::now().to_rfc3339(),
    }
}

/// Get historical TVL data for a protocol
pub fn get_protocol_tvl_history(protocol: &str, days: u32) -> Vec<ProtocolHistoryPoint> {
    let now = chrono::Utc::now().timestamp();
    let base_tvl = match protocol {
        "Aave" => 32_000_000_000.0,
        "Uniswap" => 8_500_000_000.0,
        "Lido" => 40_000_000_000.0,
        "Compound" => 12_500_000_000.0,
        _ => 5_000_000_000.0,
    };
    
    let mut points = Vec::new();
    let day_seconds: i64 = 86400;
    
    for i in (0..days).rev() {
        let timestamp = now - (i as i64 * day_seconds);
        // Generate pseudo-random variation
        let variation = 1.0 + (i as f64 * 0.002) - (days as f64 * 0.001);
        let tvl = base_tvl * variation;
        let volume = tvl * 0.05 * (1.0 + (i as f64 * 0.01));
        let users = (50000.0 + (i as f64 * 500.0)) as u64;
        
        points.push(ProtocolHistoryPoint {
            timestamp,
            tvl,
            volume,
            users,
        });
    }
    
    points
}
