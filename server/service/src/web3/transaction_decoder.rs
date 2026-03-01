//! Transaction Decoder Service
//! Decodes transaction data and provides human-readable transaction information

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of transaction decoding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedTransaction {
    pub to: String,
    pub from: String,
    pub value: String,
    pub gas: String,
    pub gas_price: String,
    pub nonce: u64,
    pub data: DecodedData,
    pub chain_id: u64,
}

/// Decoded transaction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedData {
    pub method: String,
    pub signature: String,
    pub params: Vec<DecodedParam>,
}

/// Individual decoded parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedParam {
    pub name: String,
    pub param_type: String,
    pub value: String,
    pub decoded: String,
}

/// Known method signatures for common contracts
pub struct MethodSignatureDatabase {
    signatures: HashMap<String, MethodInfo>,
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub params: Vec<ParamInfo>,
}

#[derive(Debug, Clone)]
pub struct ParamInfo {
    pub name: String,
    pub param_type: String,
}

impl MethodSignatureDatabase {
    pub fn new() -> Self {
        let mut signatures = HashMap::new();
        
        // ERC20 Methods
        signatures.insert(
            "a9059cbb".to_string(),
            MethodInfo {
                name: "transfer".to_string(),
                params: vec![
                    ParamInfo { name: "to".to_string(), param_type: "address".to_string() },
                    ParamInfo { name: "amount".to_string(), param_type: "uint256".to_string() },
                ],
            },
        );
        
        signatures.insert(
            "095ea7b3".to_string(),
            MethodInfo {
                name: "approve".to_string(),
                params: vec![
                    ParamInfo { name: "spender".to_string(), param_type: "address".to_string() },
                    ParamInfo { name: "amount".to_string(), param_type: "uint256".to_string() },
                ],
            },
        );
        
        signatures.insert(
            "23b872dd".to_string(),
            MethodInfo {
                name: "transferFrom".to_string(),
                params: vec![
                    ParamInfo { name: "from".to_string(), param_type: "address".to_string() },
                    ParamInfo { name: "to".to_string(), param_type: "address".to_string() },
                    ParamInfo { name: "amount".to_string(), param_type: "uint256".to_string() },
                ],
            },
        );
        
        signatures.insert(
            "70a08231".to_string(),
            MethodInfo {
                name: "balanceOf".to_string(),
                params: vec![
                    ParamInfo { name: "owner".to_string(), param_type: "address".to_string() },
                ],
            },
        );
        
        signatures.insert(
            "18160ddd".to_string(),
            MethodInfo {
                name: "totalSupply".to_string(),
                params: vec![],
            },
        );
        
        // ERC721 Methods
        signatures.insert(
            "42842e0e".to_string(),
            MethodInfo {
                name: "safeTransferFrom".to_string(),
                params: vec![
                    ParamInfo { name: "from".to_string(), param_type: "address".to_string() },
                    ParamInfo { name: "to".to_string(), param_type: "address".to_string() },
                    ParamInfo { name: "tokenId".to_string(), param_type: "uint256".to_string() },
                ],
            },
        );
        
        signatures.insert(
            "b88d4fde".to_string(),
            MethodInfo {
                name: "safeTransferFrom".to_string(),
                params: vec![
                    ParamInfo { name: "from".to_string(), param_type: "address".to_string() },
                    ParamInfo { name: "to".to_string(), param_type: "address".to_string() },
                    ParamInfo { name: "tokenId".to_string(), param_type: "uint256".to_string() },
                    ParamInfo { name: "data".to_string(), param_type: "bytes".to_string() },
                ],
            },
        );
        
        // Uniswap V3
        signatures.insert(
            "c04b8d59".to_string(),
            MethodInfo {
                name: "exactInputSingle".to_string(),
                params: vec![
                    ParamInfo { name: "params".to_string(), param_type: "tuple".to_string() },
                ],
            },
        );
        
        signatures.insert(
            "04e251f7".to_string(),
            MethodInfo {
                name: "exactInput".to_string(),
                params: vec![
                    ParamInfo { name: "path".to_string(), param_type: "bytes".to_string() },
                    ParamInfo { name: "recipient".to_string(), param_type: "address".to_string() },
                    ParamInfo { name: "deadline".to_string(), param_type: "uint256".to_string() },
                    ParamInfo { name: "amountIn".to_string(), param_type: "uint256".to_string() },
                    ParamInfo { name: "amountOutMinimum".to_string(), param_type: "uint256".to_string() },
                ],
            },
        );
        
        signatures.insert(
            "0dfe1681".to_string(),
            MethodInfo {
                name: "exactOutputSingle".to_string(),
                params: vec![
                    ParamInfo { name: "params".to_string(), param_type: "tuple".to_string() },
                ],
            },
        );
        
        // Wrapper (WETH)
        signatures.insert(
            "d0e30db0".to_string(),
            MethodInfo {
                name: "deposit".to_string(),
                params: vec![],
            },
        );
        
        signatures.insert(
            "2e1a7d4d".to_string(),
            MethodInfo {
                name: "withdraw".to_string(),
                params: vec![
                    ParamInfo { name: "amount".to_string(), param_type: "uint256".to_string() },
                ],
            },
        );
        
        Self { signatures }
    }
    
    /// Look up a method by its 4-byte signature
    pub fn lookup(&self, signature: &str) -> Option<&MethodInfo> {
        self.signatures.get(signature)
    }
}

/// Decode transaction input data
pub fn decode_transaction_data(data: &str) -> DecodedData {
    let data = data.strip_prefix("0x").unwrap_or(data);
    
    if data.len() < 8 {
        return DecodedData {
            method: "Unknown".to_string(),
            signature: "0x00000000".to_string(),
            params: vec![],
        };
    }
    
    let signature = &data[..8];
    let params = &data[8..];
    
    let db = MethodSignatureDatabase::new();
    
    if let Some(method_info) = db.lookup(signature) {
        let decoded_params = decode_params(params, &method_info.params);
        
        DecodedData {
            method: method_info.name.clone(),
            signature: format!("0x{}", signature),
            params: decoded_params,
        }
    } else {
        // Unknown method - just show raw data
        DecodedData {
            method: format!("0x{}", signature),
            signature: format!("0x{}", signature),
            params: vec![DecodedParam {
                name: "data".to_string(),
                param_type: "bytes".to_string(),
                value: format!("0x{}", params),
                decoded: "Unable to decode".to_string(),
            }],
        }
    }
}

/// Decode function parameters from hex string
fn decode_params(data: &str, param_info: &[ParamInfo]) -> Vec<DecodedParam> {
    let mut params = Vec::new();
    let mut offset = 0;
    
    for info in param_info {
        let (value, new_offset) = decode_single_param(data, offset, &info.param_type);
        params.push(DecodedParam {
            name: info.name.clone(),
            param_type: info.param_type.clone(),
            value: value.0,
            decoded: value.1,
        });
        offset = new_offset;
    }
    
    params
}

/// Decode a single parameter
fn decode_single_param(data: &str, offset: usize, param_type: &str) -> ((String, String), usize) {
    if offset >= data.len() {
        return (("0x".to_string(), "0".to_string()), offset);
    }
    
    match param_type {
        "address" => {
            let end = offset + 64;
            let hex = &data[offset..end.min(data.len())];
            let addr = format!("0x{}", &hex[24..]);
            ((addr.clone(), addr), end)
        }
        "uint256" | "uint128" | "uint64" | "uint32" | "uint16" | "uint8" => {
            let end = offset + 64;
            let hex = &data[offset..end.min(data.len())];
            let value = u64::from_str_radix(hex, 16).unwrap_or(0);
            ((format!("0x{}", hex), value.to_string()), end)
        }
        "bytes32" => {
            let end = offset + 64;
            let hex = &data[offset..end.min(data.len())];
            ((format!("0x{}", hex), format!("0x{}", hex)), end)
        }
        "string" | "bytes" => {
            // Dynamic type - would need to parse offset
            let end = offset + 64;
            let hex = &data[offset..end.min(data.len())];
            ((format!("0x{}", hex), "[dynamic]".to_string()), end)
        }
        _ => {
            let end = offset + 64;
            let hex = &data[offset..end.min(data.len())];
            ((format!("0x{}", hex), format!("0x{}", hex)), end)
        }
    }
}

impl Default for MethodSignatureDatabase {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decode_transfer() {
        // transfer(address to, uint256 amount)
        // 0xa9059cbb + address(20 bytes) + uint256(32 bytes)
        let data = "a9059cbb000000000000000000000000742d35cc6634c0532925a3b844bc9e7595f0beb1000000000000000000000000000000000000000000000000000de0b6b3a7640000";
        let decoded = decode_transaction_data(data);
        
        assert_eq!(decoded.method, "transfer");
        assert_eq!(decoded.params.len(), 2);
    }
    
    #[test]
    fn test_decode_approve() {
        let data = "095ea7b3000000000000000000000000742d35cc6634c0532925a3b844bc9e7595f0beb1000000000000000000000000000000000000000000000000000de0b6b3a7640000";
        let decoded = decode_transaction_data(data);
        
        assert_eq!(decoded.method, "approve");
    }
}
