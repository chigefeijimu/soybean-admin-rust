// =========================================
// Web3 Service with Alloy (Rust)
// =========================================

use async_trait::async_trait;
use alloy::{
    providers::{Provider, Http},
    signers::local::PrivateKeySigner,
    primitives::{Address, U256, keccak256},
    rpc::types::TransactionRequest,
};
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, Set,
};
use ulid::Ulid;
use std::sync::Arc;
use sea_orm::DatabaseConnection;

// ============ 钱包服务 ============

#[derive(Clone)]
pub struct Web3WalletService;

/// 生成签名消息 (EIP-191)
pub fn generate_sign_message(address: &str, nonce: &str) -> String {
    format!(
        "Sign this message to verify wallet ownership.\n\nAddress: {}\nNonce: {}",
        address.to_lowercase(),
        nonce
    )
}

/// 使用 Alloy 验证签名
pub async fn verify_wallet_signature(
    address: &str,
    message: &str,
    signature: &str,
) -> Result<bool, String> {
    // 计算消息哈希 (EIP-191)
    let prefixed_message = format!(
        "\x19Ethereum Signed Message:\n{}{}",
        message.len(),
        message
    );
    let message_hash = keccak256(prefixed_message.as_bytes());
    
    // 从签名恢复地址 (简化版本)
    // 在生产环境中使用 ecrecover
    // 
    // 注意: Alloy 提供了 signers 模块进行签名验证
    // 这里返回 true 作为占位符
    
    Ok(true)
}

/// 使用 Alloy 发送交易
pub async fn send_transaction(
    rpc_url: &str,
    from_private_key: &str,
    to_address: &str,
    value_eth: f64,
) -> Result<String, String> {
    // 创建 provider
    let provider = Http::from(rpc_url);
    
    // 解析私钥
    let signer: PrivateKeySigner = from_private_key.parse()
        .map_err(|e| format!("Invalid private key: {}", e))?;
    
    // 解析地址
    let to: Address = to_address.parse()
        .map_err(|e| format!("Invalid address: {}", e))?;
    
    // 转换 ETH 到 Wei
    let value = U256::from((value_eth * 1e18) as u64);
    
    // 构建交易
    let tx = TransactionRequest::default()
        .to(to)
        .value(value)
        .from(signer.address());
    
    // 发送交易
    let pending_tx = provider.send_transaction(tx)
        .await
        .map_err(|e| format!("Failed to send transaction: {}", e))?;
    
    // 返回交易哈希
    Ok(pending_tx.tx_hash().to_string())
}

/// 使用 Alloy 读取合约
pub async fn call_contract(
    rpc_url: &str,
    contract_address: &str,
    function_name: &str,
    params: &[String],
) -> Result<String, String> {
    let provider = Http::from(rpc_url);
    
    // 这里需要使用 alloy-contract 宏来生成合约调用代码
    // 简化版本返回占位符
    
    Ok(format!(
        "Contract call: {} on {} with params {:?}",
        function_name, contract_address, params
    ))
}

/// 估计 Gas 费用
pub async fn estimate_gas(
    rpc_url: &str,
    from: &str,
    to: &str,
    value_eth: f64,
) -> Result<u64, String> {
    let provider = Http::from(rpc_url);
    
    let from: Address = from.parse()
        .map_err(|e| format!("Invalid from address: {}", e))?;
    let to: Address = to.parse()
        .map_err(|e| format!("Invalid to address: {}", e))?;
    
    let value = U256::from((value_eth * 1e18) as u64);
    
    let tx = TransactionRequest::default()
        .from(from)
        .to(to)
        .value(value);
    
    let gas = provider.estimate_gas(&tx)
        .await
        .map_err(|e| format!("Gas estimation failed: {}", e))?;
    
    Ok(gas)
}

/// 获取当前 Gas 价格
pub async fn get_gas_price(rpc_url: &str) -> Result<U256, String> {
    let provider = Http::from(rpc_url);
    let gas_price = provider.get_gas_price()
        .await
        .map_err(|e| format!("Failed to get gas price: {}", e))?;
    Ok(gas_price)
}
