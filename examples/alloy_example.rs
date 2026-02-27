// =========================================
// Alloy 示例代码 (替代 ethers-rs)
// =========================================
//
// Alloy 是现代 Ethereum Rust 库，版本 1.7.3
// 官方文档: https://docs.rs/alloy/latest/alloy/
//
// 安装:
// alloy = { version = "1.7", features = ["rpc-client", "signer", "sol-types", "consensus"] }

// ============ 1. 基本连接 ============
use alloy::{
    providers::{Provider, Http},
    signers::local::PrivateKeySigner,
    primitives::Address,
};

// 创建 HTTP provider
let provider = Http::from("https://eth.llamarpc.com");

// 签名者 (从私钥)
let signer: PrivateKeySigner = "0x...".parse().unwrap();

// ============ 2. 获取余额 ============
use alloy::primitives::U256;

let balance = provider.get_balance(Address::ZERO).await?;
println!("Balance: {}", balance);

// ============ 3. 发送交易 ============
use alloy::rpc::types::TransactionRequest;

let tx = TransactionRequest::default()
    .to(to_address)
    .value(U256::from(1000000000000000000u64)) // 1 ETH
    .from(from_address);

let pending_tx = provider.send_transaction(tx).await?;
let receipt = pending_tx.await?;

// ============ 4. 合约交互 ============
// 使用 alloy-contract 宏
// #[alloy::solrpc]
// contract MyContract {
//     function transfer(address to, uint256 amount) external;
// }

// ============ 5. 签名验证 (EIP-191) ============
use alloy::signers::utils::message_hash;
use alloy::primitives::keccak256;

fn verify_signature(message: &str, signature: &str, expected_address: &str) -> bool {
    let message_hash = keccak256(format!("\x19Ethereum Signed Message:\n{}{}", message.len(), message).as_bytes());
    // 使用 ecrecover 恢复地址
    // ...
    true
}

// ============ 6. 多链配置 ============
const CHAINS: &[(u64, &str)] = &[
    (1, "https://eth.llamarpc.com"),         // Ethereum
    (56, "https://bsc-dataseed.binance.org"), // BSC
    (137, "https://polygon-rpc.com"),         // Polygon
    (11155111, "https://sepolia.infura.io/v3/YOUR_KEY"), // Sepolia
];

fn get_rpc_url(chain_id: u64) -> Option<&'static str> {
    CHAINS.iter()
        .find(|(id, _)| *id == chain_id)
        .map(|(_, url)| *url)
}

// ============ 7. ABI 编码/解码 ============
use alloy::sol_types::{SolInterface, SolCall};

#[derive(Debug, SolCall, serde::Serialize, serde::Deserialize)]
#[sol(discard)]
struct TransferCall {
    address to,
    uint256 amount,
}

// 编码
let data = TransferCall { to: address, amount: U256::from(100) }.encode();

// 解码
let call = TransferCall::decode(&data, true)?;
