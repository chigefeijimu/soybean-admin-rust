//! Price Alert Service
//! Monitors token prices and triggers alerts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Price alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceAlert {
    pub id: String,
    pub user_address: String,
    pub token: String,
    pub condition: AlertCondition,
    pub target_price: f64,
    pub current_price: f64,
    pub triggered: bool,
    pub created_at: String,
    pub triggered_at: Option<String>,
}

/// Alert condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    Above,
    Below,
    ChangePercent,
}

/// Alert notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertNotification {
    pub alert_id: String,
    pub token: String,
    pub message: String,
    pub triggered_price: f64,
    pub target_price: f64,
    pub timestamp: String,
}

/// Price history for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub timestamp: u64,
    pub price: f64,
}

/// Price alert service
pub struct PriceAlertService {
    alerts: HashMap<String, Vec<PriceAlert>>,
    prices: HashMap<String, f64>,
}

impl PriceAlertService {
    pub fn new() -> Self {
        let mut prices = HashMap::new();
        prices.insert("ETH".to_string(), 2500.0);
        prices.insert("BTC".to_string(), 62500.0);
        prices.insert("USDC".to_string(), 1.0);
        prices.insert("USDT".to_string(), 1.0);
        prices.insert("DAI".to_string(), 1.0);
        prices.insert("WBTC".to_string(), 62500.0);
        prices.insert("SOL".to_string(), 120.0);
        prices.insert("UNI".to_string(), 8.5);
        prices.insert("AAVE".to_string(), 85.0);
        prices.insert("LINK".to_string(), 15.0);
        
        Self {
            alerts: HashMap::new(),
            prices,
        }
    }

    /// Create a new price alert
    pub fn create_alert(
        &mut self,
        user_address: &str,
        token: &str,
        condition: AlertCondition,
        target_price: f64,
    ) -> PriceAlert {
        let id = format!("alert_{}_{}", token, self.alerts.len());
        
        let alert = PriceAlert {
            id: id.clone(),
            user_address: user_address.to_string(),
            token: token.to_string(),
            condition,
            target_price,
            current_price: *self.prices.get(token).unwrap_or(&0.0),
            triggered: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            triggered_at: None,
        };
        
        self.alerts
            .entry(user_address.to_string())
            .or_default()
            .push(alert.clone());
        
        alert
    }

    /// Get user's alerts
    pub fn get_alerts(&self, user_address: &str) -> Vec<PriceAlert> {
        self.alerts
            .get(user_address)
            .cloned()
            .unwrap_or_default()
    }

    /// Check and trigger alerts
    pub fn check_alerts(&mut self, token: &str) -> Vec<AlertNotification> {
        let mut notifications = Vec::new();
        
        if let Some(&current_price) = self.prices.get(token) {
            for alerts in self.alerts.values_mut() {
                for alert in alerts.iter_mut() {
                    if alert.token == token && !alert.triggered {
                        let should_trigger = match alert.condition {
                            AlertCondition::Above => current_price >= alert.target_price,
                            AlertCondition::Below => current_price <= alert.target_price,
                            AlertCondition::ChangePercent => {
                                let change = ((current_price - alert.current_price) / alert.current_price * 100.0).abs();
                                change >= alert.target_price
                            }
                        };
                        
                        if should_trigger {
                            alert.triggered = true;
                            alert.triggered_at = Some(chrono::Utc::now().to_rfc3339());
                            
                            notifications.push(AlertNotification {
                                alert_id: alert.id.clone(),
                                token: token.to_string(),
                                message: format!(
                                    "{} is now {} ${:.2} (target: ${:.2})",
                                    token,
                                    if matches!(alert.condition, AlertCondition::Above) { "above" } else { "below" },
                                    current_price,
                                    alert.target_price
                                ),
                                triggered_price: current_price,
                                target_price: alert.target_price,
                                timestamp: chrono::Utc::now().to_rfc3339(),
                            });
                        }
                    }
                }
            }
        }
        
        notifications
    }

    /// Delete an alert
    pub fn delete_alert(&mut self, user_address: &str, alert_id: &str) -> bool {
        if let Some(alerts) = self.alerts.get_mut(user_address) {
            let initial_len = alerts.len();
            alerts.retain(|a| a.id != alert_id);
            return alerts.len() < initial_len;
        }
        false
    }

    /// Update current prices
    pub fn update_prices(&mut self, token_prices: HashMap<String, f64>) {
        for (token, price) in token_prices {
            self.prices.insert(token, price);
        }
    }

    /// Get price history (mock)
    pub fn get_price_history(&self, token: &str, hours: u32) -> Vec<PricePoint> {
        let base_price = self.prices.get(token).copied().unwrap_or(100.0);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        (0..hours)
            .map(|i| {
                let variation = 1.0 + (i as f64 * 0.1).sin() * 0.05;
                PricePoint {
                    timestamp: now - (i as u64 * 3600),
                    price: base_price * variation,
                }
            })
            .collect()
    }

    /// Get all active alerts
    pub fn get_all_active_alerts(&self) -> Vec<PriceAlert> {
        self.alerts
            .values()
            .flatten()
            .filter(|a| !a.triggered)
            .cloned()
            .collect()
    }
}

impl Default for PriceAlertService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_alert() {
        let mut service = PriceAlertService::new();
        let alert = service.create_alert(
            "0x123",
            "ETH",
            AlertCondition::Above,
            3000.0,
        );
        
        assert_eq!(alert.token, "ETH");
        assert!(!alert.triggered);
    }

    #[test]
    fn test_check_alerts() {
        let mut service = PriceAlertService::new();
        service.create_alert("0x123", "ETH", AlertCondition::Above, 2000.0);
        
        let notifications = service.check_alerts("ETH");
        // Should trigger since 2500 > 2000
        assert!(!notifications.is_empty());
    }
}
