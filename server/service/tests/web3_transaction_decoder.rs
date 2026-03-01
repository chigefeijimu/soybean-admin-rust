//! Transaction Decoder Tests

#[cfg(test)]
mod tests {
    use server_service::web3::transaction_decoder::{
        decode_transaction_data, MethodSignatureDatabase,
    };

    /// Test MethodSignatureDatabase::new() creates database with known signatures
    #[test]
    fn test_method_signature_database_new() {
        let db = MethodSignatureDatabase::new();
        
        // ERC20 transfer
        assert!(db.lookup("a9059cbb").is_some());
        let transfer = db.lookup("a9059cbb").unwrap();
        assert_eq!(transfer.name, "transfer");
        assert_eq!(transfer.params.len(), 2);
        assert_eq!(transfer.params[0].name, "to");
        assert_eq!(transfer.params[1].name, "amount");
        
        // ERC20 approve
        assert!(db.lookup("095ea7b3").is_some());
        let approve = db.lookup("095ea7b3").unwrap();
        assert_eq!(approve.name, "approve");
        assert_eq!(approve.params.len(), 2);
        
        // ERC20 balanceOf
        assert!(db.lookup("70a08231").is_some());
        let balance = db.lookup("70a08231").unwrap();
        assert_eq!(balance.name, "balanceOf");
        
        // ERC20 totalSupply
        assert!(db.lookup("18160ddd").is_some());
        let supply = db.lookup("18160ddd").unwrap();
        assert_eq!(supply.name, "totalSupply");
        assert!(supply.params.is_empty());
        
        // ERC721 safeTransferFrom (4-byte)
        assert!(db.lookup("42842e0e").is_some());
        
        // WETH deposit
        assert!(db.lookup("d0e30db0").is_some());
        let deposit = db.lookup("d0e30db0").unwrap();
        assert_eq!(deposit.name, "deposit");
        
        // WETH withdraw
        assert!(db.lookup("2e1a7d4d").is_some());
    }

    /// Test lookup returns None for unknown signatures
    #[test]
    fn test_method_signature_database_unknown() {
        let db = MethodSignatureDatabase::new();
        
        assert!(db.lookup("ffffffff").is_none());
        assert!(db.lookup("00000000").is_none());
    }

    /// Test decoding ERC20 transfer transaction
    #[test]
    fn test_decode_transfer() {
        // transfer(address to, uint256 amount)
        // to: 0x1234567890123456789012345678901234567890
        // amount: 1000000000 (1 USDC in units)
        let data = "0xa9059cbb0000000000000000000000001234567890123456789012345678901234567890000000000000000000000000000000000000000000000000000003b9aca000";
        
        let result = decode_transaction_data(data);
        
        assert_eq!(result.method, "transfer");
        assert_eq!(result.signature, "0xa9059cbb");
        assert_eq!(result.params.len(), 2);
        
        // Check 'to' parameter
        let to_param = &result.params[0];
        assert_eq!(to_param.name, "to");
        assert_eq!(to_param.param_type, "address");
        assert!(to_param.value.starts_with("0x"));
        
        // Check 'amount' parameter
        let amount_param = &result.params[1];
        assert_eq!(amount_param.name, "amount");
        assert_eq!(amount_param.param_type, "uint256");
    }

    /// Test decoding ERC20 approve transaction
    #[test]
    fn test_decode_approve() {
        // approve(address spender, uint256 amount)
        let data = "0x095ea7b3000000000000000000000000abcdefabcdefabcdefabcdefabcdefabcdefabcd0000000000000000000000000000000000000000000000000000000000000001000000";
        
        let result = decode_transaction_data(data);
        
        assert_eq!(result.method, "approve");
        assert_eq!(result.signature, "0x095ea7b3");
        assert_eq!(result.params.len(), 2);
        assert_eq!(result.params[0].name, "spender");
        assert_eq!(result.params[1].name, "amount");
    }

    /// Test decoding WETH deposit
    #[test]
    fn test_decode_weth_deposit() {
        // deposit() - no parameters
        let data = "0xd0e30db0";
        
        let result = decode_transaction_data(data);
        
        assert_eq!(result.method, "deposit");
        assert_eq!(result.signature, "0xd0e30db0");
        assert!(result.params.is_empty());
    }

    /// Test decoding WETH withdraw
    #[test]
    fn test_decode_weth_withdraw() {
        // withdraw(uint256 amount)
        let data = "0x2e1a7d4d0000000000000000000000000000000000000000000000000000000000000001000000";
        
        let result = decode_transaction_data(data);
        
        assert_eq!(result.method, "withdraw");
        assert_eq!(result.signature, "0x2e1a7d4d");
        assert_eq!(result.params.len(), 1);
        assert_eq!(result.params[0].name, "amount");
        assert_eq!(result.params[0].param_type, "uint256");
    }

    /// Test decoding unknown method signature
    #[test]
    fn test_decode_unknown_method() {
        let data = "0xdeadbeef0000000000000000000000000000000000000000000000000000000000000001";
        
        let result = decode_transaction_data(data);
        
        // Unknown method should return the signature as the method name
        assert!(result.method.starts_with("0x"));
        assert!(result.signature.starts_with("0x"));
        // Should have at least one param with raw data
        assert!(!result.params.is_empty());
    }

    /// Test decoding transaction with 0x prefix
    #[test]
    fn test_decode_with_0x_prefix() {
        // Valid data with single 0x prefix - should work
        let data = "0x095ea7b3000000000000000000000000abcdefabcdefabcdefabcdefabcdefabcdefabcd0000000000000000000000000000000000000000000000000000000000000001";
        
        let result = decode_transaction_data(data);
        
        // Should successfully decode
        assert_eq!(result.method, "approve");
    }

    /// Test decoding transaction with short data
    #[test]
    fn test_decode_short_data() {
        let data = "0x1234"; // Too short to be valid
        
        let result = decode_transaction_data(data);
        
        assert_eq!(result.method, "Unknown");
        assert_eq!(result.signature, "0x00000000");
        assert!(result.params.is_empty());
    }

    /// Test DecodedData fields are correctly populated
    #[test]
    fn test_decoded_data_fields() {
        let data = "0xa9059cbb0000000000000000000000001234567890123456789012345678901234567890000000000000000000000000000000000000000000000000000003b9aca000";
        
        let result = decode_transaction_data(data);
        
        // Verify all fields exist and are non-empty
        assert!(!result.method.is_empty());
        assert!(!result.signature.is_empty());
        assert!(result.signature.starts_with("0x"));
        assert!(result.signature.len() == 10); // 0x + 8 hex chars
        
        // Verify params are properly populated
        for param in &result.params {
            assert!(!param.name.is_empty());
            assert!(!param.param_type.is_empty());
            assert!(!param.value.is_empty());
        }
    }

    /// Test ERC721 safeTransferFrom decoding
    #[test]
    fn test_decode_erc721_safe_transfer() {
        // safeTransferFrom(address from, address to, uint256 tokenId)
        let data = "0x42842e0e0000000000000000000000001111111111111111111111111111111111111111000000000000000000000000002222222222222222222222222222222222222222000000000000000000000000000000000000000000000000000000000000000000001";
        
        let result = decode_transaction_data(data);
        
        assert_eq!(result.method, "safeTransferFrom");
        assert_eq!(result.params.len(), 3);
        assert_eq!(result.params[0].name, "from");
        assert_eq!(result.params[1].name, "to");
        assert_eq!(result.params[2].name, "tokenId");
    }

    /// Test Uniswap exactInputSingle decoding
    #[test]
    fn test_decode_uniswap_exact_input_single() {
        // exactInputSingle((address tokenIn, address tokenOut, uint24 fee, address recipient, uint256 deadline, uint256 amountIn, uint256 amountOutMinimum, uint160 sqrtPriceLimitX96))
        let data = "0xc04b8d59000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa000000000000000000000000bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb0000000000000000000000000000000000000000000000000000000000000000003e800000000000000000000000000000000000000000000000000000000000000016345785d8a0000";
        
        let result = decode_transaction_data(data);
        
        assert_eq!(result.method, "exactInputSingle");
        // Should decode params
        assert!(!result.params.is_empty());
    }
}
