use serde::{Deserialize, Serialize};
/// Gas fee analytics entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasAnalyticsEntry {
    pub date: String,
    pub total_gas_used: String,
    pub total_gas_cost_eth: String,
    pub total_gas_cost_usd: String,
    pub transaction_count: u64,
    pub avg_gas_price: String,
    pub avg_gas_used: String,
}

/// Gas analytics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasAnalyticsSummary {
    pub total_gas_eth: String,
    pub total_gas_usd: String,
    pub total_transactions: u64,
    pub avg_gas_price: String,
    pub avg_transaction_cost: String,
    pub period_days: u32,
    pub trend: String, // "increasing", "decreasing", "stable"
    pub change_percentage: f64,
}

/// Gas analytics by hour
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasHourlyAnalytics {
    pub hour: String,
    pub gas_used: String,
    pub gas_cost_eth: String,
    pub transaction_count: u64,
}

/// Gas analytics by day of week
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasDayOfWeekAnalytics {
    pub day: String,
    pub avg_gas_used: String,
    pub avg_gas_cost: String,
    pub transaction_count: u64,
}

/// Gas optimization suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasOptimizationSuggestion {
    pub suggestion_type: String,
    pub description: String,
    pub potential_savings_eth: String,
    pub potential_savings_usd: String,
    pub priority: String, // "high", "medium", "low"
}

/// Gas fee analytics input
#[derive(Debug, Clone, Deserialize)]
pub struct GasAnalyticsInput {
    pub address: String,
    pub chain_id: Option<u64>,
    pub period_days: Option<u32>, // Default: 30
}

/// Input for gas by hour
#[derive(Debug, Clone, Deserialize)]
pub struct GasByHourInput {
    pub address: String,
    pub chain_id: Option<u64>,
    pub days: Option<u32>,
}

/// Input for gas by day of week
#[derive(Debug, Clone, Deserialize)]
pub struct GasByDayOfWeekInput {
    pub address: String,
    pub chain_id: Option<u64>,
    pub days: Option<u32>,
}

/// Gas comparison input
#[derive(Debug, Clone, Deserialize)]
pub struct GasComparisonInput {
    pub address: String,
    pub chain_id: Option<u64>,
    pub period_days: Option<u32>,
}

/// Gas comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasComparisonResult {
    pub current_period: GasAnalyticsSummary,
    pub previous_period: GasAnalyticsSummary,
    pub comparison: GasComparison,
}

/// Gas comparison data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasComparison {
    pub gas_change_percent: f64,
    pub cost_change_percent: f64,
    pub transaction_change_percent: f64,
    pub trend: String,
}
