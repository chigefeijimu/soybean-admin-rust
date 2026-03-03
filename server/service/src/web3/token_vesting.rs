// Token Vesting Schedule Tracker Service
// Tracks token unlock schedules for teams, investors, and advisors

use serde::{Deserialize, Serialize};

/// Token Vesting Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenVestingInfo {
    pub token_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub chain_id: u64,
    pub total_supply: String,
    pub vesting_schedule: Vec<VestingSchedule>,
    pub summary: VestingSummary,
}

/// Individual Vesting Schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingSchedule {
    pub beneficiary: String,
    pub beneficiary_type: String, // team, investor, advisor, foundation, community
    pub total_allocated: String,
    pub unlocked_amount: String,
    pub locked_amount: String,
    pub start_date: String,
    pub end_date: String,
    pub cliff_period: Option<u64>, // days
    pub vesting_period: Option<u64>, // days
    pub release_schedule: Vec<ReleaseEvent>,
    pub risk_level: String, // low, medium, high
}

/// Release Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseEvent {
    pub date: String,
    pub amount: String,
    pub percentage: f64,
    pub is_released: bool,
}

/// Vesting Summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingSummary {
    pub total_vested: String,
    pub total_unlocked: String,
    pub total_locked: String,
    pub unlock_percentage: f64,
    pub next_unlock_date: Option<String>,
    pub next_unlock_amount: Option<String>,
    pub fully_unlocked_count: u32,
    pub partially_unlocked_count: u32,
    pub fully_locked_count: u32,
}

/// Vesting Timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingTimeline {
    pub token_address: String,
    pub chain_id: u64,
    pub timeline: Vec<TimelineEvent>,
}

/// Timeline Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub date: String,
    pub event_type: String, // unlock, cliff, schedule_start
    pub amount: String,
    pub beneficiary: String,
    pub cumulative_unlocked: String,
}

/// Popular Vesting Tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularVestingToken {
    pub token_address: String,
    pub token_name: String,
    pub token_symbol: String,
    pub chain_id: u64,
    pub total_vested: String,
    pub unlock_progress: f64,
    pub days_until_next_unlock: Option<i64>,
}

/// Vesting Comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingComparison {
    pub token1: TokenVestingInfo,
    pub token2: TokenVestingInfo,
    pub comparison: VestingComparisonResult,
}

/// Vesting Comparison Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VestingComparisonResult {
    pub more_centralized: String,
    pub more_liquid: String,
    pub risk_comparison: String,
    pub unlock_schedule_difference_days: i64,
}

/// Input for getting vesting info
#[derive(Debug, Deserialize)]
pub struct GetVestingInput {
    pub token_address: String,
    pub chain_id: Option<u64>,
}

/// Input for searching vesting tokens
#[derive(Debug, Deserialize)]
pub struct SearchVestingTokensInput {
    pub query: Option<String>,
    pub chain_id: Option<u64>,
    pub sort_by: Option<String>, // unlock_progress, total_vested, next_unlock
    pub limit: Option<usize>,
}

/// Input for vesting timeline
#[derive(Debug, Deserialize)]
pub struct GetVestingTimelineInput {
    pub token_address: String,
    pub chain_id: Option<u64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Input for comparing vesting schedules
#[derive(Debug, Deserialize)]
pub struct CompareVestingInput {
    pub token1_address: String,
    pub token2_address: String,
    pub chain_id: Option<u64>,
}

/// Token Vesting Service
pub struct TokenVestingService;

// Mock data for demonstration - in production, this would query blockchain/external APIs
impl TokenVestingService {
    /// Get vesting information for a token
    pub fn get_vesting_info(input: &GetVestingInput) -> Result<TokenVestingInfo, String> {
        let chain_id = input.chain_id.unwrap_or(1);
        
        // Mock data - in production, fetch from blockchain or API
        let mock_data = get_mock_vesting_data(&input.token_address, chain_id)?;
        
        Ok(mock_data)
    }

    /// Get popular vesting tokens
    pub fn get_popular_vesting_tokens(input: &SearchVestingTokensInput) -> Result<Vec<PopularVestingToken>, String> {
        let mut tokens = get_popular_vesting_tokens_mock();
        
        // Filter by chain
        if let Some(chain_id) = input.chain_id {
            tokens.retain(|t| t.chain_id == chain_id);
        }
        
        // Filter by query
        if let Some(query) = &input.query {
            let query_lower = query.to_lowercase();
            tokens.retain(|t| 
                t.token_name.to_lowercase().contains(&query_lower) ||
                t.token_symbol.to_lowercase().contains(&query_lower)
            );
        }
        
        // Sort
        if let Some(sort_by) = &input.sort_by {
            match sort_by.as_str() {
                "unlock_progress" => tokens.sort_by(|a, b| {
                    b.unlock_progress.partial_cmp(&a.unlock_progress).unwrap()
                }),
                "total_vested" => tokens.sort_by(|a, b| {
                    b.total_vested.cmp(&a.total_vested)
                }),
                "next_unlock" => tokens.sort_by(|a, b| {
                    match (a.days_until_next_unlock, b.days_until_next_unlock) {
                        (Some(da), Some(db)) => da.cmp(&db),
                        (Some(_), None) => std::cmp::Ordering::Less,
                        (None, Some(_)) => std::cmp::Ordering::Greater,
                        _ => std::cmp::Ordering::Equal,
                    }
                }),
                _ => {}
            }
        }
        
        // Limit
        if let Some(limit) = input.limit {
            tokens.truncate(limit);
        }
        
        Ok(tokens)
    }

    /// Get vesting timeline
    pub fn get_vesting_timeline(input: &GetVestingTimelineInput) -> Result<VestingTimeline, String> {
        let chain_id = input.chain_id.unwrap_or(1);
        
        // Mock timeline data
        let timeline = generate_mock_timeline(&input.token_address, chain_id);
        
        Ok(VestingTimeline {
            token_address: input.token_address.clone(),
            chain_id,
            timeline,
        })
    }

    /// Compare two vesting schedules
    pub fn compare_vesting(input: &CompareVestingInput) -> Result<VestingComparison, String> {
        let chain_id = input.chain_id.unwrap_or(1);
        
        let token1_data = get_mock_vesting_data(&input.token1_address, chain_id)?;
        let token2_data = get_mock_vesting_data(&input.token2_address, chain_id)?;
        
        // Calculate comparison
        let token1_unlocked_pct = token1_data.summary.unlock_percentage;
        let token2_unlocked_pct = token2_data.summary.unlock_percentage;
        
        let more_liquid = if token1_unlocked_pct > token2_unlocked_pct {
            token1_data.token_symbol.clone()
        } else {
            token2_data.token_symbol.clone()
        };
        
        let more_centralized = if token1_data.vesting_schedule.len() < token2_data.vesting_schedule.len() {
            token1_data.token_symbol.clone()
        } else {
            token2_data.token_symbol.clone()
        };
        
        let risk_comparison = compare_risk_levels(&token1_data, &token2_data);
        
        Ok(VestingComparison {
            token1: token1_data,
            token2: token2_data,
            comparison: VestingComparisonResult {
                more_centralized,
                more_liquid,
                risk_comparison,
                unlock_schedule_difference_days: 0,
            },
        })
    }
}

// Helper functions
fn get_mock_vesting_data(token_address: &str, chain_id: u64) -> Result<TokenVestingInfo, String> {
    // Return mock data based on token address
    let (token_name, token_symbol) = match token_address.to_lowercase().as_str() {
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48" | "usdc" => ("USD Coin", "USDC"),
        "0x514910771af9ca656af840dff83e8264ecf986ca" | "link" => ("Chainlink", "LINK"),
        "0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9" | "aave" => ("Aave", "AAVE"),
        "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984" | "uni" => ("Uniswap", "UNI"),
        "0x7d1afa7b718fb893db30a3abc0cfc608aacfebb0" | "matic" => ("Polygon", "MATIC"),
        _ => ("Sample Token", "SAMPLE"),
    };
    
    let total_supply = "1000000000".to_string();
    
    let vesting_schedule = vec![
        VestingSchedule {
            beneficiary: "0x742d35Cc6634C0532925a3b844Bc9e7595f0Ab12".to_string(),
            beneficiary_type: "team".to_string(),
            total_allocated: "150000000".to_string(),
            unlocked_amount: "75000000".to_string(),
            locked_amount: "75000000".to_string(),
            start_date: "2024-01-01".to_string(),
            end_date: "2027-01-01".to_string(),
            cliff_period: Some(365),
            vesting_period: Some(1095),
            release_schedule: vec![
                ReleaseEvent {
                    date: "2024-01-01".to_string(),
                    amount: "0".to_string(),
                    percentage: 0.0,
                    is_released: false,
                },
                ReleaseEvent {
                    date: "2025-01-01".to_string(),
                    amount: "37500000".to_string(),
                    percentage: 25.0,
                    is_released: true,
                },
                ReleaseEvent {
                    date: "2026-01-01".to_string(),
                    amount: "37500000".to_string(),
                    percentage: 25.0,
                    is_released: false,
                },
                ReleaseEvent {
                    date: "2027-01-01".to_string(),
                    amount: "75000000".to_string(),
                    percentage: 50.0,
                    is_released: false,
                },
            ],
            risk_level: "medium".to_string(),
        },
        VestingSchedule {
            beneficiary: "0x8ba1f109551bD432803012645Ac136ddd64DBA72".to_string(),
            beneficiary_type: "investor".to_string(),
            total_allocated: "100000000".to_string(),
            unlocked_amount: "100000000".to_string(),
            locked_amount: "0".to_string(),
            start_date: "2023-06-01".to_string(),
            end_date: "2024-06-01".to_string(),
            cliff_period: Some(90),
            vesting_period: Some(365),
            release_schedule: vec![
                ReleaseEvent {
                    date: "2023-09-01".to_string(),
                    amount: "25000000".to_string(),
                    percentage: 25.0,
                    is_released: true,
                },
                ReleaseEvent {
                    date: "2023-12-01".to_string(),
                    amount: "25000000".to_string(),
                    percentage: 25.0,
                    is_released: true,
                },
                ReleaseEvent {
                    date: "2024-03-01".to_string(),
                    amount: "25000000".to_string(),
                    percentage: 25.0,
                    is_released: true,
                },
                ReleaseEvent {
                    date: "2024-06-01".to_string(),
                    amount: "25000000".to_string(),
                    percentage: 25.0,
                    is_released: true,
                },
            ],
            risk_level: "low".to_string(),
        },
        VestingSchedule {
            beneficiary: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            beneficiary_type: "foundation".to_string(),
            total_allocated: "200000000".to_string(),
            unlocked_amount: "40000000".to_string(),
            locked_amount: "160000000".to_string(),
            start_date: "2024-01-01".to_string(),
            end_date: "2029-01-01".to_string(),
            cliff_period: Some(180),
            vesting_period: Some(1825),
            release_schedule: vec![
                ReleaseEvent {
                    date: "2024-07-01".to_string(),
                    amount: "20000000".to_string(),
                    percentage: 10.0,
                    is_released: true,
                },
                ReleaseEvent {
                    date: "2025-01-01".to_string(),
                    amount: "20000000".to_string(),
                    percentage: 10.0,
                    is_released: false,
                },
                ReleaseEvent {
                    date: "2026-01-01".to_string(),
                    amount: "20000000".to_string(),
                    percentage: 10.0,
                    is_released: false,
                },
                ReleaseEvent {
                    date: "2027-01-01".to_string(),
                    amount: "20000000".to_string(),
                    percentage: 10.0,
                    is_released: false,
                },
                ReleaseEvent {
                    date: "2028-01-01".to_string(),
                    amount: "20000000".to_string(),
                    percentage: 10.0,
                    is_released: false,
                },
                ReleaseEvent {
                    date: "2029-01-01".to_string(),
                    amount: "100000000".to_string(),
                    percentage: 50.0,
                    is_released: false,
                },
            ],
            risk_level: "low".to_string(),
        },
        VestingSchedule {
            beneficiary: "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B".to_string(),
            beneficiary_type: "community".to_string(),
            total_allocated: "550000000".to_string(),
            unlocked_amount: "275000000".to_string(),
            locked_amount: "275000000".to_string(),
            start_date: "2023-01-01".to_string(),
            end_date: "2026-01-01".to_string(),
            cliff_period: None,
            vesting_period: Some(1095),
            release_schedule: vec![
                ReleaseEvent {
                    date: "2023-01-01".to_string(),
                    amount: "137500000".to_string(),
                    percentage: 25.0,
                    is_released: true,
                },
                ReleaseEvent {
                    date: "2024-01-01".to_string(),
                    amount: "137500000".to_string(),
                    percentage: 25.0,
                    is_released: true,
                },
                ReleaseEvent {
                    date: "2025-01-01".to_string(),
                    amount: "137500000".to_string(),
                    percentage: 25.0,
                    is_released: false,
                },
                ReleaseEvent {
                    date: "2026-01-01".to_string(),
                    amount: "137500000".to_string(),
                    percentage: 25.0,
                    is_released: false,
                },
            ],
            risk_level: "medium".to_string(),
        },
    ];
    
    let summary = VestingSummary {
        total_vested: "1000000000".to_string(),
        total_unlocked: "490000000".to_string(),
        total_locked: "510000000".to_string(),
        unlock_percentage: 49.0,
        next_unlock_date: Some("2025-06-01".to_string()),
        next_unlock_amount: Some("50000000".to_string()),
        fully_unlocked_count: 1,
        partially_unlocked_count: 3,
        fully_locked_count: 0,
    };
    
    Ok(TokenVestingInfo {
        token_address: token_address.to_string(),
        token_name: token_name.to_string(),
        token_symbol: token_symbol.to_string(),
        chain_id,
        total_supply,
        vesting_schedule,
        summary,
    })
}

fn get_popular_vesting_tokens_mock() -> Vec<PopularVestingToken> {
    vec![
        PopularVestingToken {
            token_address: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
            token_name: "USD Coin".to_string(),
            token_symbol: "USDC".to_string(),
            chain_id: 1,
            total_vested: "10000000000".to_string(),
            unlock_progress: 85.5,
            days_until_next_unlock: Some(0),
        },
        PopularVestingToken {
            token_address: "0x514910771af9ca656af840dff83e8264ecf986ca".to_string(),
            token_name: "Chainlink".to_string(),
            token_symbol: "LINK".to_string(),
            chain_id: 1,
            total_vested: "1000000000".to_string(),
            unlock_progress: 62.3,
            days_until_next_unlock: Some(15),
        },
        PopularVestingToken {
            token_address: "0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9".to_string(),
            token_name: "Aave".to_string(),
            token_symbol: "AAVE".to_string(),
            chain_id: 1,
            total_vested: "16000000".to_string(),
            unlock_progress: 45.2,
            days_until_next_unlock: Some(30),
        },
        PopularVestingToken {
            token_address: "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984".to_string(),
            token_name: "Uniswap".to_string(),
            token_symbol: "UNI".to_string(),
            chain_id: 1,
            total_vested: "1000000000".to_string(),
            unlock_progress: 38.7,
            days_until_next_unlock: Some(7),
        },
        PopularVestingToken {
            token_address: "0x7d1afa7b718fb893db30a3abc0cfc608aacfebb0".to_string(),
            token_name: "Polygon".to_string(),
            token_symbol: "MATIC".to_string(),
            chain_id: 137,
            total_vested: "10000000000".to_string(),
            unlock_progress: 72.1,
            days_until_next_unlock: Some(3),
        },
        PopularVestingToken {
            token_address: "0x3c499c542cEF5E3998Eb327A8EDc8028e12fD251".to_string(),
            token_name: "WETH".to_string(),
            token_symbol: "WETH".to_string(),
            chain_id: 42161,
            total_vested: "500000000".to_string(),
            unlock_progress: 55.0,
            days_until_next_unlock: Some(45),
        },
    ]
}

fn generate_mock_timeline(_token_address: &str, _chain_id: u64) -> Vec<TimelineEvent> {
    vec![
        TimelineEvent {
            date: "2023-01-01".to_string(),
            event_type: "schedule_start".to_string(),
            amount: "0".to_string(),
            beneficiary: "Multiple".to_string(),
            cumulative_unlocked: "0".to_string(),
        },
        TimelineEvent {
            date: "2023-06-01".to_string(),
            event_type: "unlock".to_string(),
            amount: "25000000".to_string(),
            beneficiary: "Investor".to_string(),
            cumulative_unlocked: "25000000".to_string(),
        },
        TimelineEvent {
            date: "2024-01-01".to_string(),
            event_type: "cliff".to_string(),
            amount: "0".to_string(),
            beneficiary: "Team".to_string(),
            cumulative_unlocked: "25000000".to_string(),
        },
        TimelineEvent {
            date: "2024-01-01".to_string(),
            event_type: "unlock".to_string(),
            amount: "137500000".to_string(),
            beneficiary: "Community".to_string(),
            cumulative_unlocked: "162500000".to_string(),
        },
        TimelineEvent {
            date: "2024-06-01".to_string(),
            event_type: "unlock".to_string(),
            amount: "100000000".to_string(),
            beneficiary: "Investor".to_string(),
            cumulative_unlocked: "262500000".to_string(),
        },
        TimelineEvent {
            date: "2024-07-01".to_string(),
            event_type: "unlock".to_string(),
            amount: "20000000".to_string(),
            beneficiary: "Foundation".to_string(),
            cumulative_unlocked: "282500000".to_string(),
        },
        TimelineEvent {
            date: "2025-01-01".to_string(),
            event_type: "unlock".to_string(),
            amount: "57500000".to_string(),
            beneficiary: "Multiple".to_string(),
            cumulative_unlocked: "340000000".to_string(),
        },
        TimelineEvent {
            date: "2025-06-01".to_string(),
            event_type: "unlock".to_string(),
            amount: "50000000".to_string(),
            beneficiary: "Team".to_string(),
            cumulative_unlocked: "390000000".to_string(),
        },
        TimelineEvent {
            date: "2026-01-01".to_string(),
            event_type: "unlock".to_string(),
            amount: "170000000".to_string(),
            beneficiary: "Multiple".to_string(),
            cumulative_unlocked: "560000000".to_string(),
        },
        TimelineEvent {
            date: "2027-01-01".to_string(),
            event_type: "unlock".to_string(),
            amount: "75000000".to_string(),
            beneficiary: "Team".to_string(),
            cumulative_unlocked: "635000000".to_string(),
        },
    ]
}

fn compare_risk_levels(token1: &TokenVestingInfo, token2: &TokenVestingInfo) -> String {
    let count_risky = |schedules: &[VestingSchedule]| {
        schedules.iter().filter(|s| s.risk_level == "high").count()
    };
    
    let risk1 = count_risky(&token1.vesting_schedule);
    let risk2 = count_risky(&token2.vesting_schedule);
    
    if risk1 > risk2 {
        format!("{} has higher risk allocations", token1.token_symbol)
    } else if risk2 > risk1 {
        format!("{} has higher risk allocations", token2.token_symbol)
    } else {
        "Both tokens have similar risk profiles".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_vesting_info() {
        let input = GetVestingInput {
            token_address: "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".to_string(),
            chain_id: Some(1),
        };
        
        let result = TokenVestingService::get_vesting_info(&input);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_popular_tokens() {
        let input = SearchVestingTokensInput {
            query: None,
            chain_id: Some(1),
            sort_by: Some("unlock_progress".to_string()),
            limit: Some(5),
        };
        
        let result = TokenVestingService::get_popular_vesting_tokens(&input);
        assert!(result.is_ok());
    }
}
