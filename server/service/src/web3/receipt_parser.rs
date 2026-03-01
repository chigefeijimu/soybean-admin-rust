// =========================================
// Transaction Receipt Parser
// Parses transaction receipts and extracts event logs
// =========================================

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Transaction receipt from JSON-RPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub transaction_hash: String,
    pub block_number: String,
    pub block_hash: String,
    pub contract_address: Option<String>,
    pub cumulative_gas_used: String,
    pub effective_gas_price: String,
    pub from: String,
    pub gas_used: String,
    pub logs: Vec<Log>,
    pub logs_bloom: String,
    pub status: String,
    pub to: Option<String>,
    pub transaction_index: String,
}

/// Individual log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub log_index: String,
    pub transaction_index: String,
    pub transaction_hash: String,
    pub block_number: String,
    pub block_hash: String,
}

/// Parsed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedEvent {
    pub name: String,
    pub signature: String,
    pub address: String,
    pub params: Vec<EventParam>,
    pub log_index: usize,
}

/// Event parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventParam {
    pub name: String,
    pub value: String,
    pub indexed: bool,
}

/// Transaction receipt with parsed events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedReceipt {
    pub transaction_hash: String,
    pub block_number: u64,
    pub from: String,
    pub to: Option<String>,
    pub contract_address: Option<String>,
    pub status: bool,
    pub gas_used: u64,
    pub effective_gas_price: u64,
    pub events: Vec<ParsedEvent>,
}

/// Known event signatures (topics[0])
pub struct EventSignatureDatabase {
    signatures: HashMap<String, EventInfo>,
}

#[derive(Debug, Clone)]
pub struct EventInfo {
    pub name: String,
    pub params: Vec<EventParamInfo>,
}

#[derive(Debug, Clone)]
pub struct EventParamInfo {
    pub name: String,
    pub param_type: String,
    pub indexed: bool,
}

impl EventSignatureDatabase {
    pub fn new() -> Self {
        let mut signatures = HashMap::new();
        
        // ERC20 Events
        // Transfer(address indexed from, address indexed to, uint256 value)
        signatures.insert(
            "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string(),
            EventInfo {
                name: "Transfer".to_string(),
                params: vec![
                    EventParamInfo { name: "from".to_string(), param_type: "address".to_string(), indexed: true },
                    EventParamInfo { name: "to".to_string(), param_type: "address".to_string(), indexed: true },
                    EventParamInfo { name: "value".to_string(), param_type: "uint256".to_string(), indexed: false },
                ],
            },
        );
        
        // Approval(address indexed owner, address indexed spender, uint256 value)
        signatures.insert(
            "8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925".to_string(),
            EventInfo {
                name: "Approval".to_string(),
                params: vec![
                    EventParamInfo { name: "owner".to_string(), param_type: "address".to_string(), indexed: true },
                    EventParamInfo { name: "spender".to_string(), param_type: "address".to_string(), indexed: true },
                    EventParamInfo { name: "value".to_string(), param_type: "uint256".to_string(), indexed: false },
                ],
            },
        );
        
        // OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
        signatures.insert(
            "8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0".to_string(),
            EventInfo {
                name: "OwnershipTransferred".to_string(),
                params: vec![
                    EventParamInfo { name: "previousOwner".to_string(), param_type: "address".to_string(), indexed: true },
                    EventParamInfo { name: "newOwner".to_string(), param_type: "address".to_string(), indexed: true },
                ],
            },
        );
        
        // Swap(address indexed sender, uint amount0In, uint amount1In, uint amount0Out, uint amount1Out, address indexed to)
        signatures.insert(
            "d78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d1308409d822cc".to_string(),
            EventInfo {
                name: "Swap".to_string(),
                params: vec![
                    EventParamInfo { name: "sender".to_string(), param_type: "address".to_string(), indexed: true },
                    EventParamInfo { name: "amount0In".to_string(), param_type: "uint256".to_string(), indexed: false },
                    EventParamInfo { name: "amount1In".to_string(), param_type: "uint256".to_string(), indexed: false },
                    EventParamInfo { name: "amount0Out".to_string(), param_type: "uint256".to_string(), indexed: false },
                    EventParamInfo { name: "amount1Out".to_string(), param_type: "uint256".to_string(), indexed: false },
                    EventParamInfo { name: "to".to_string(), param_type: "address".to_string(), indexed: true },
                ],
            },
        );
        
        // Deposit(address indexed dst, uint wad)
        signatures.insert(
            "e1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109c".to_string(),
            EventInfo {
                name: "Deposit".to_string(),
                params: vec![
                    EventParamInfo { name: "dst".to_string(), param_type: "address".to_string(), indexed: true },
                    EventParamInfo { name: "wad".to_string(), param_type: "uint256".to_string(), indexed: false },
                ],
            },
        );
        
        // Withdrawal(address indexed src, uint wad)
        signatures.insert(
            "7fcf532c15f0a6d0e3aead4e1dc3d9b5a9a9e9f5e5e5e5e5e5e5e5e5e5e5e5e5".to_string(),
            EventInfo {
                name: "Withdrawal".to_string(),
                params: vec![
                    EventParamInfo { name: "src".to_string(), param_type: "address".to_string(), indexed: true },
                    EventParamInfo { name: "wad".to_string(), param_type: "uint256".to_string(), indexed: false },
                ],
            },
        );
        
        Self { signatures }
    }
    
    /// Look up an event by its signature (topic[0])
    pub fn lookup(&self, signature: &str) -> Option<&EventInfo> {
        self.signatures.get(signature)
    }
}

/// Parse a transaction receipt and extract events
pub fn parse_receipt(receipt: TransactionReceipt) -> ParsedReceipt {
    let db = EventSignatureDatabase::new();
    let mut events = Vec::new();
    
    for (idx, log) in receipt.logs.iter().enumerate() {
        if let Some(topic0) = log.topics.first() {
            if let Some(event_info) = db.lookup(topic0) {
                let params = parse_event_params(log, &event_info.params);
                
                events.push(ParsedEvent {
                    name: event_info.name.clone(),
                    signature: topic0.clone(),
                    address: log.address.clone(),
                    params,
                    log_index: idx,
                });
            } else {
                // Unknown event - create generic representation
                let params: Vec<EventParam> = log.topics.iter().enumerate().map(|(i, t)| {
                    EventParam {
                        name: format!("topic{}", i),
                        value: t.clone(),
                        indexed: i > 0, // topic0 is the signature, others can be indexed
                    }
                }).collect();
                
                events.push(ParsedEvent {
                    name: "Unknown".to_string(),
                    signature: topic0.clone(),
                    address: log.address.clone(),
                    params,
                    log_index: idx,
                });
            }
        }
    }
    
    let status = receipt.status == "0x1";
    let gas_used = u64::from_str_radix(receipt.gas_used.trim_start_matches("0x"), 16).unwrap_or(0);
    let gas_price = u64::from_str_radix(receipt.effective_gas_price.trim_start_matches("0x"), 16).unwrap_or(0);
    let block_number = u64::from_str_radix(receipt.block_number.trim_start_matches("0x"), 16).unwrap_or(0);
    
    ParsedReceipt {
        transaction_hash: receipt.transaction_hash,
        block_number,
        from: receipt.from,
        to: receipt.to,
        contract_address: receipt.contract_address,
        status,
        gas_used,
        effective_gas_price: gas_price,
        events,
    }
}

/// Parse event parameters from log
fn parse_event_params(log: &Log, param_info: &[EventParamInfo]) -> Vec<EventParam> {
    let mut params = Vec::new();
    
    for (i, info) in param_info.iter().enumerate() {
        let value = if info.indexed {
            // Indexed parameters are in topics (starting from topic 1)
            if i + 1 < log.topics.len() {
                decode_indexed_param(&log.topics[i + 1], &info.param_type)
            } else {
                "".to_string()
            }
        } else {
            // Non-indexed parameters are in data
            decode_data_param(&log.data, &info.param_type)
        };
        
        params.push(EventParam {
            name: info.name.clone(),
            value,
            indexed: info.indexed,
        });
    }
    
    params
}

/// Decode an indexed parameter (from topics)
fn decode_indexed_param(topic: &str, param_type: &str) -> String {
    let topic = topic.trim_start_matches("0x");
    
    match param_type {
        "address" => {
            // Last 20 bytes of 32-byte topic
            if topic.len() >= 40 {
                format!("0x{}", &topic[topic.len() - 40..])
            } else {
                format!("0x{}", topic)
            }
        }
        "uint256" | "uint128" | "uint64" | "uint32" | "uint16" | "uint8" => {
            // Big integer as string
            u64::from_str_radix(topic, 16)
                .map(|v| v.to_string())
                .unwrap_or_else(|_| topic.to_string())
        }
        _ => format!("0x{}", topic),
    }
}

/// Decode a non-indexed parameter (from data)
fn decode_data_param(data: &str, param_type: &str) -> String {
    let data = data.trim_start_matches("0x");
    
    match param_type {
        "address" => {
            if data.len() >= 64 {
                let addr = &data[data.len() - 64..];
                format!("0x{}", &addr[24..])
            } else {
                format!("0x{}", data)
            }
        }
        "uint256" => {
            if data.len() >= 64 {
                let val = &data[data.len() - 64..];
                u64::from_str_radix(val, 16)
                    .map(|v| v.to_string())
                    .unwrap_or_else(|_| format!("0x{}", val))
            } else {
                format!("0x{}", data)
            }
        }
        "bytes" | "string" => {
            // Dynamic types would need more complex decoding
            if data.len() > 64 {
                // First 32 bytes is offset, next 32 is length
                // For simplicity, just return the raw data
                format!("0x{}", &data[64..])
            } else {
                format!("0x{}", data)
            }
        }
        _ => format!("0x{}", data),
    }
}

/// Format gas info for display
pub fn format_gas_info(receipt: &ParsedReceipt) -> String {
    let total_gas = receipt.gas_used * receipt.effective_gas_price;
    let eth_gas = total_gas as f64 / 1e18;
    let gwei_gas = receipt.effective_gas_price as f64 / 1e9;
    
    format!(
        "Gas Used: {} | Gas Price: {} Gwei | Total: {:.6} ETH",
        receipt.gas_used, gwei_gas, eth_gas
    )
}

impl Default for EventSignatureDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_transfer_event() {
        // Example Transfer event
        let receipt = TransactionReceipt {
            transaction_hash: "0x1234".to_string(),
            block_number: "0x10".to_string(),
            block_hash: "0xabc".to_string(),
            contract_address: None,
            cumulative_gas_used: "0x1000".to_string(),
            effective_gas_price: "0x4e3b29200".to_string(),
            from: "0xfrom".to_string(),
            gas_used: "0x100".to_string(),
            logs: vec![Log {
                address: "0xtoken".to_string(),
                topics: vec![
                    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string(),
                    "0x000000000000000000000000742d35cc6634c0532925a3b844bc9e7595f0beb1".to_string(),
                    "0x0000000000000000000000001234567890123456789012345678901234567890".to_string(),
                ],
                data: "0x000000000000000000000000000000000000000000000000000de0b6b3a7640000".to_string(),
                log_index: "0x0".to_string(),
                transaction_index: "0x0".to_string(),
                transaction_hash: "0x1234".to_string(),
                block_number: "0x10".to_string(),
                block_hash: "0xabc".to_string(),
            }],
            logs_bloom: "".to_string(),
            status: "0x1".to_string(),
            to: Some("0xto".to_string()),
            transaction_index: "0x0".to_string(),
        };
        
        let parsed = parse_receipt(receipt);
        assert!(parsed.status);
        assert_eq!(parsed.events.len(), 1);
        assert_eq!(parsed.events[0].name, "Transfer");
    }
}
