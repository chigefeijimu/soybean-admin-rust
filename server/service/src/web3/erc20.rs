// =========================================
// ERC20 Token Utilities (Simplified)
// Uses JSON-RPC calls for compatibility
// =========================================

use alloy::primitives::{Address, U256};
use std::error::Error;

/// ERC20 Token info
#[derive(Debug, Clone)]
pub struct Erc20Token {
    pub address: Address,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u8,
}

/// ERC20 Token holder info
#[derive(Debug, Clone)]
pub struct Erc20Holder {
    pub address: Address,
    pub balance: U256,
    pub balance_formatted: String,
}

/// ERC20 Functions selector (first 4 bytes of Keccak-256)
pub mod selectors {
    /// balanceOf(address) selector
    pub const BALANCE_OF: &[u8; 4] = &0x70_a0_2e_27u32.to_be_bytes();
    /// name() selector
    pub const NAME: &[u8; 4] = &0x06_fd_de_03u32.to_be_bytes();
    /// symbol() selector
    pub const SYMBOL: &[u8; 4] = &0x95_d7_89_3du32.to_be_bytes();
    /// decimals() selector
    pub const DECIMALS: &[u8; 4] = &0x31_3c_cd_7du32.to_be_bytes();
    /// totalSupply() selector
    pub const TOTAL_SUPPLY: &[u8; 4] = &0x18_1c_60_deu32.to_be_bytes();
    /// transfer(address,uint256) selector
    pub const TRANSFER: &[u8; 4] = &0xa9_05_9a_abu32.to_be_bytes();
    /// approve(address,uint256) selector
    pub const APPROVE: &[u8; 4] = &0x09_5e_a7_b3u32.to_be_bytes();
}

/// Encode address parameter for contract call
pub fn encode_address(addr: Address) -> [u8; 32] {
    let mut padded = [0u8; 32];
    padded[12..].copy_from_slice(addr.as_slice());
    padded
}

/// Encode uint256 parameter for contract call
/// U256 is 256 bits = 32 bytes
#[allow(dead_code)]
pub fn encode_uint256(value: U256) -> [u8; 32] {
    // Convert U256 to bytes - simple approach using to_string
    let hex_str = value.to_string();
    let hex_clean = hex_str.strip_prefix("0x").unwrap_or(&hex_str);
    
    // Pad to 64 chars (32 bytes)
    let padded = format!("{:0>64}", hex_clean);
    let decoded = hex::decode(padded).unwrap_or_default();
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&decoded);
    bytes
}

/// ERC20 Call data builder
pub struct Erc20Call {
    pub to: Address,
    pub data: Vec<u8>,
}

impl Erc20Call {
    /// Create balanceOf call
    pub fn balance_of(token: Address, owner: Address) -> Self {
        let mut data = Vec::with_capacity(36);
        data.extend_from_slice(selectors::BALANCE_OF);
        data.extend_from_slice(&encode_address(owner));
        Self { to: token, data }
    }

    /// Create name call
    pub fn name(token: Address) -> Self {
        let mut data = Vec::with_capacity(4);
        data.extend_from_slice(selectors::NAME);
        Self { to: token, data }
    }

    /// Create symbol call
    pub fn symbol(token: Address) -> Self {
        let mut data = Vec::with_capacity(4);
        data.extend_from_slice(selectors::SYMBOL);
        Self { to: token, data }
    }

    /// Create decimals call
    pub fn decimals(token: Address) -> Self {
        let mut data = Vec::with_capacity(4);
        data.extend_from_slice(selectors::DECIMALS);
        Self { to: token, data }
    }
}

/// Format token balance based on decimals
pub fn format_balance(balance: U256, decimals: u8) -> String {
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

/// Parse address from hex string
pub fn parse_address(s: &str) -> Result<Address, Box<dyn Error + Send + Sync>> {
    let s = s.trim_start_matches("0x").trim_start_matches("0X");
    let bytes = hex::decode(s)?;
    if bytes.len() != 20 {
        return Err("Address must be 20 bytes".into());
    }
    Ok(Address::from_slice(&bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_balance() {
        // 1 ETH = 10^18 wei
        let balance = U256::from(1_000_000_000_000_000_000u64);
        assert_eq!(format_balance(balance, 18), "1");
        
        let balance = U256::from(1_500_000_000_000_000_000u64);
        assert_eq!(format_balance(balance, 18), "1.5");
        
        // USDC (6 decimals)
        let balance = U256::from(1_000_000u64);
        assert_eq!(format_balance(balance, 6), "1");
    }

    #[test]
    fn test_selectors() {
        assert_eq!(selectors::BALANCE_OF, &[0x70, 0xa0, 0x2e, 0x27]);
        assert_eq!(selectors::NAME, &[0x06, 0xfd, 0xde, 0x03]);
    }

    #[test]
    fn test_encode_address() {
        let addr = Address::from_low_u64_be(0x1234567890abcdef1234567890abcdef12345678);
        let encoded = encode_address(addr);
        assert_eq!(encoded[12..], addr.as_slice());
    }

    #[test]
    fn test_parse_address() {
        let addr = parse_address("0x1234567890abcdef1234567890abcdef12345678").unwrap();
        assert_eq!(addr, Address::from_low_u64_be(0x1234567890abcdef1234567890abcdef12345678));
    }
}
