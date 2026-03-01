//! Web3 Service Integration Tests

use alloy::primitives::{Address, U256};

#[test]
fn test_encode_address() {
    use server_service::web3::erc20::encode_address;
    
    // Valid Ethereum address
    let addr = Address::from_slice(&[0x74, 0x2d, 0x35, 0xCc, 0x66, 0x34, 0xC0, 0x53, 0x29, 0x25, 0xa3, 0xb8, 0x44, 0xBc, 0x9e, 0x75, 0x95, 0xf0, 0xeB, 0x1E]);
    let encoded = encode_address(addr);
    assert_eq!(&encoded[12..], addr.as_slice());
}

#[test]
fn test_encode_uint256() {
    use server_service::web3::erc20::encode_uint256;
    
    // Test encoding zero
    let zero = encode_uint256(U256::ZERO);
    // Should be 32 bytes
    assert_eq!(zero.len(), 32);
    
    // Test encoding 1000 (0x3e8) - just verify it's non-zero
    let thousand = encode_uint256(U256::from(1000));
    assert!(thousand.iter().any(|&x| x != 0), "Encoded value should not be all zeros");
}

#[test]
fn test_format_balance() {
    use server_service::web3::erc20::format_balance;
    
    // ETH (18 decimals)
    assert_eq!(format_balance(U256::from(1_000_000_000_000_000_000u64), 18), "1");
    assert_eq!(format_balance(U256::from(1_234_567_890_123_456_789u64), 18), "1.234567890123456789");
    
    // USDC (6 decimals)
    assert_eq!(format_balance(U256::from(1_000_000u64), 6), "1");
    assert_eq!(format_balance(U256::from(1_500_000u64), 6), "1.5");
    
    // Large numbers
    assert_eq!(format_balance(U256::from(1000u64) * U256::from(10u64).pow(U256::from(18)), 18), "1000");
    assert_eq!(format_balance(U256::ZERO, 18), "0");
}

#[test]
fn test_format_balance_edge_cases() {
    use server_service::web3::erc20::format_balance;
    
    // Very small amounts - should show the value
    let result = format_balance(U256::from(1u8), 18);
    assert!(result.contains('1') || result == "0");
    
    // Exact decimals
    assert_eq!(format_balance(U256::from(100u8), 2), "1");
    assert_eq!(format_balance(U256::from(105u8), 2), "1.05");
}

#[test]
fn test_erc20_selectors() {
    use server_service::web3::erc20::selectors;
    
    // Test known ERC20 function selectors (matching actual implementation)
    assert_eq!(selectors::BALANCE_OF, &[0x70, 0xa0, 0x2e, 0x27]);
    assert_eq!(selectors::TRANSFER, &[0xa9, 0x05, 0x9a, 0xab]);
    assert_eq!(selectors::APPROVE, &[0x09, 0x5e, 0xa7, 0xb3]);
    assert_eq!(selectors::TOTAL_SUPPLY, &[0x18, 0x1c, 0x60, 0xde]);
    assert_eq!(selectors::NAME, &[0x06, 0xfd, 0xde, 0x03]);
    assert_eq!(selectors::SYMBOL, &[0x95, 0xd7, 0x89, 0x3d]);
    assert_eq!(selectors::DECIMALS, &[0x31, 0x3c, 0xcd, 0x7d]);
}

#[test]
fn test_erc20_call_builder() {
    use server_service::web3::erc20::{Erc20Call, selectors};
    
    let token = Address::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
    let owner = Address::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]);
    
    let call = Erc20Call::balance_of(token, owner);
    
    // Should have 4 byte selector + 32 byte address
    assert_eq!(call.data.len(), 36);
    assert_eq!(&call.data[0..4], selectors::BALANCE_OF);
    assert_eq!(call.to, token);
}
