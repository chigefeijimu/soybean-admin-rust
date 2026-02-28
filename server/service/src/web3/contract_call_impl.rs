// =========================================
// Contract Call Implementation
// Uses JSON-RPC for contract calls (compatible with all alloy versions)
// =========================================

use alloy::primitives::{Address, U256};
use reqwest::Client;
use serde_json::{json, Value};
use std::str::FromStr;

/// ERC20 ABI functions (selectors)
#[allow(dead_code)]
mod selectors {
    pub const NAME: &[u8; 4] = &[0x06, 0xfd, 0xde, 0x03];
    pub const SYMBOL: &[u8; 4] = &[0x95, 0xd7, 0x89, 0x3d];
    pub const DECIMALS: &[u8; 4] = &[0x31, 0x3c, 0xcd, 0x7d];
    pub const TOTAL_SUPPLY: &[u8; 4] = &[0x18, 0x1c, 0x60, 0xde];
    pub const BALANCE_OF: &[u8; 4] = &[0x70, 0xa0, 0x2e, 0x27];
    pub const ALLOWANCE: &[u8; 4] = &[0x09, 0x5e, 0xa7, 0xb3];
    pub const TRANSFER: &[u8; 4] = &[0xa9, 0x05, 0x9a, 0xab];
    pub const APPROVE: &[u8; 4] = &[0x09, 0x5e, 0xa7, 0xb3];
}

/// Build call data for contract method
fn build_call_data(selector: &[u8; 4], params: Vec<String>) -> String {
    let mut data = format!("0x{}", hex::encode(selector));
    for param in params {
        let param_clean = param.trim_start_matches("0x").trim_start_matches("0X");
        if param_clean.len() == 40 {
            // Address - pad to 32 bytes
            data.push_str(&"0".repeat(24));
            data.push_str(param_clean);
        } else if param_clean.len() <= 64 {
            // Uint - pad to 32 bytes
            data.push_str(&"0".repeat(64 - param_clean.len()));
            data.push_str(param_clean);
        } else {
            data.push_str(param_clean);
        }
    }
    data
}

/// Execute a read-only contract call via JSON-RPC
pub async fn execute_contract_read(
    rpc_url: &str,
    contract_address: &str,
    method: &str,
    params: Vec<String>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    
    // Validate address
    let _addr = Address::from_str(contract_address)?;
    
    // Get selector
    let selector = match method {
        "name" => selectors::NAME,
        "symbol" => selectors::SYMBOL,
        "decimals" => selectors::DECIMALS,
        "totalSupply" => selectors::TOTAL_SUPPLY,
        "balanceOf" => selectors::BALANCE_OF,
        "allowance" => selectors::ALLOWANCE,
        _ => return Err(format!(
            "Unknown method: {}. Supported: name, symbol, decimals, totalSupply, balanceOf, allowance",
            method
        ).into()),
    };
    
    // Build call data
    let data = build_call_data(selector, params);
    
    // Make JSON-RPC call
    let request = json!({
        "jsonrpc": "2.0",
        "method": "eth_call",
        "params": [{
            "to": contract_address,
            "data": data
        }, "latest"],
        "id": 1
    });
    
    let response = client
        .post(rpc_url)
        .json(&request)
        .send()
        .await?;
    
    let json: Value = response.json().await?;
    
    if let Some(error) = json.get("error") {
        return Err(format!("RPC error: {:?}", error).into());
    }
    
    let result = json["result"]
        .as_str()
        .ok_or("No result in response")?;
    
    // Parse result based on method
    match method {
        "name" | "symbol" => {
            // String return - need to decode from bytes
            // Simplified: return raw hex
            Ok(result.to_string())
        }
        "decimals" => {
            // Uint8 - last byte
            let clean = result.trim_start_matches("0x");
            let last_two = &clean[clean.len()-2..];
            let val = u8::from_str_radix(last_two, 16)?;
            Ok(val.to_string())
        }
        _ => {
            // Uint256 - return as-is
            Ok(result.to_string())
        }
    }
}

/// Get ERC20 token balance for an address
pub async fn get_erc20_balance(
    rpc_url: &str,
    token_address: &str,
    owner_address: &str,
) -> Result<U256, Box<dyn std::error::Error + Send + Sync>> {
    let result = execute_contract_read(
        rpc_url,
        token_address,
        "balanceOf",
        vec![owner_address.to_string()],
    ).await?;
    
    let clean = result.trim_start_matches("0x");
    let balance = U256::from_str_radix(clean, 16)?;
    
    Ok(balance)
}

/// Get multiple token balances
pub async fn get_token_balances(
    rpc_url: &str,
    token_addresses: Vec<String>,
    owner_address: &str,
) -> Result<Vec<(String, U256)>, Box<dyn std::error::Error + Send + Sync>> {
    let mut results = Vec::new();
    
    for token_addr in token_addresses {
        let balance = get_erc20_balance(rpc_url, &token_addr, owner_address).await?;
        results.push((token_addr, balance));
    }
    
    Ok(results)
}

/// Format balance based on decimals
pub fn format_token_balance(balance: U256, decimals: u8) -> String {
    let divisor = U256::from(10).pow(U256::from(decimals));
    let whole = balance / divisor;
    let fractional = balance % divisor;
    
    let frac_str = fractional.to_string();
    let frac_padded = format!("{}{}", "0".repeat((decimals as usize).saturating_sub(frac_str.len())), frac_str);
    
    if frac_padded == "0".repeat(decimals as usize) {
        whole.to_string()
    } else {
        format!("{}.{}", whole, frac_padded.trim_end_matches('0'))
    }
}

/*
## Usage Example

```rust
use crate::web3::contract_call_impl::{execute_contract_read, get_erc20_balance};

// Get token info
let name = execute_contract_read(
    "https://eth.llamarpc.com",
    "0xdAC17F958D2ee523a2206206994597C13D831ec7", // USDT
    "name",
    vec![]
).await?;

// Get balance
let balance = get_erc20_balance(
    "https://eth.llamarpc.com",
    "0xdAC17F958D2ee523a2206206994597C13D831ec7", // USDT
    "0x1234567890123456789012345678901234567890"  // Owner
).await?;
```
*/
