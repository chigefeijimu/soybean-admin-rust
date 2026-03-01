//! Order Service - Trading Order Management
//! Supports limit orders, market orders, stop-loss, and take-profit

use serde::{Deserialize, Serialize};
use crate::web3::ServiceError;
use alloy_primitives::U256;

/// Order type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}

/// Order side (buy/sell)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

impl Default for OrderSide {
    fn default() -> Self {
        OrderSide::Buy
    }
}

/// Order status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,      // Order created, not yet submitted
    Submitted,    // Order submitted to network
    PartiallyFilled,
    Filled,      // Fully executed
    Cancelled,   // Cancelled by user
    Expired,     // Expired (for time-limited orders)
    Failed,      // Execution failed
}

impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::Pending
    }
}

/// Time in force (validity period)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    GTC,  // Good Till Cancel
    IOC,  // Immediate Or Cancel
    FOK,  // Fill Or Kill
    GTD,  // Good Till Date
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::GTC
    }
}

/// Order input for creating a new order
#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrderInput {
    pub user_id: Option<String>,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub token_in: String,      // Input token address
    pub token_out: String,     // Output token address
    pub amount_in: String,     // Amount in wei
    pub limit_price: Option<String>,  // For limit orders (wei)
    pub stop_price: Option<String>,    // For stop orders (wei)
    pub time_in_force: Option<TimeInForce>,
    pub expire_at: Option<String>,    // Unix timestamp for GTD orders
    pub chain_id: Option<i32>,
    pub slippage_bps: Option<i32>,     // Slippage in basis points (100 = 1%)
}

/// Order output response
#[derive(Debug, Clone, Serialize)]
pub struct OrderInfo {
    pub id: String,
    pub user_id: Option<String>,
    pub order_type: OrderType,
    pub side: OrderSide,
    pub status: OrderStatus,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
    pub amount_out: Option<String>,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub filled_amount: Option<String>,
    pub average_price: Option<String>,
    pub time_in_force: TimeInForce,
    pub expire_at: Option<String>,
    pub tx_hash: Option<String>,
    pub chain_id: i32,
    pub slippage_bps: i32,
    pub created_at: String,
    pub updated_at: Option<String>,
}

/// Order list filter
#[derive(Debug, Clone, Deserialize, Default)]
pub struct OrderListFilter {
    pub user_id: Option<String>,
    pub status: Option<OrderStatus>,
    pub side: Option<OrderSide>,
    pub order_type: Option<OrderType>,
    pub chain_id: Option<i32>,
    pub token_in: Option<String>,
    pub token_out: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Cancel order input
#[derive(Debug, Clone, Deserialize)]
pub struct CancelOrderInput {
    pub order_id: String,
    pub user_id: Option<String>,
}

/// Order execution result
#[derive(Debug, Clone, Serialize)]
pub struct OrderExecutionResult {
    pub success: bool,
    pub order_id: String,
    pub tx_hash: Option<String>,
    pub filled_amount: Option<String>,
    pub average_price: Option<String>,
    pub error: Option<String>,
}

/// Slippage calculation helper
pub fn calculate_slippage(amount: &str, bps: i32) -> Result<String, ServiceError> {
    let amount_u256 = amount.parse::<U256>()
        .map_err(|_| ServiceError::new("Invalid amount"))?;
    
    // slippage = amount * bps / 10000
    let bps_u256 = U256::from(bps);
    let divisor = U256::from(10000_u64);
    let slippage = amount_u256 * bps_u256 / divisor;
    let min_amount = amount_u256.saturating_sub(slippage);
    
    Ok(min_amount.to_string())
}

/// Validate order input
pub fn validate_order_input(input: &CreateOrderInput) -> Result<(), ServiceError> {
    // Validate chain_id
    let chain_id = input.chain_id.unwrap_or(1);
    if ![1, 11155111, 137, 42161, 10, 8453].contains(&chain_id) {
        return Err(ServiceError::new("Unsupported chain_id"));
    }
    
    // Validate addresses (basic check)
    if !input.token_in.starts_with("0x") || !input.token_out.starts_with("0x") {
        return Err(ServiceError::new("Invalid token address"));
    }
    
    // Validate amount
    if input.amount_in.parse::<U256>().is_err() {
        return Err(ServiceError::new("Invalid amount_in"));
    }
    
    // For limit orders, limit_price is required
    if input.order_type == OrderType::Limit && input.limit_price.is_none() {
        return Err(ServiceError::new("limit_price required for limit orders"));
    }
    
    // For stop orders, stop_price is required
    if matches!(input.order_type, OrderType::StopLoss | OrderType::StopLossLimit | 
                             OrderType::TakeProfit | OrderType::TakeProfitLimit) 
       && input.stop_price.is_none() {
        return Err(ServiceError::new("stop_price required for stop orders"));
    }
    
    // Validate slippage (0-10000 bps = 0-100%)
    if let Some(slippage) = input.slippage_bps {
        if slippage < 0 || slippage > 10000 {
            return Err(ServiceError::new("slippage_bps must be 0-10000"));
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_slippage() {
        // 1000 * 50 / 10000 = 5 (0.5% slippage)
        // min = 1000 - 5 = 995
        let result = calculate_slippage("1000", 50).unwrap();
        assert_eq!(result, "995");
        
        // Zero slippage
        let result = calculate_slippage("1000", 0).unwrap();
        assert_eq!(result, "1000");
    }

    #[test]
    fn test_validate_order_input() {
        let valid_input = CreateOrderInput {
            user_id: None,
            order_type: OrderType::Limit,
            side: OrderSide::Buy,
            token_in: "0xA0b86a33E6441C4C4C4C4C4C4C4C4C4C4C4C".to_string(),
            token_out: "0xB0b86a33E6441C4C4C4C4C4C4C4C4C4C4C4C".to_string(),
            amount_in: "1000000000000000000".to_string(),
            limit_price: Some("2000000000000000000".to_string()),
            stop_price: None,
            time_in_force: None,
            expire_at: None,
            chain_id: Some(1),
            slippage_bps: Some(50),
        };
        
        assert!(validate_order_input(&valid_input).is_ok());
        
        // Invalid chain
        let mut invalid_input = valid_input.clone();
        invalid_input.chain_id = Some(999);
        assert!(validate_order_input(&invalid_input).is_err());
        
        // Missing limit price for limit order
        let mut invalid_input = valid_input.clone();
        invalid_input.limit_price = None;
        assert!(validate_order_input(&invalid_input).is_err());
    }
}
