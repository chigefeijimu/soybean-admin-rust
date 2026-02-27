// =========================================
// Alloy 完整示例：智能合约交互
// =========================================

use alloy::{
    //  providers: 连接区块链
    providers::{Provider, Http, Ws, ProviderBuilder},
    // signers: 钱包签名
    signers::{local::PrivateKeySigner, Signer},
    //  primitives: 基础类型
    primitives::{Address, U256, Bytes, FixedBytes, keccak256},
    //  rpc types: RPC 类型
    rpc::types::{TransactionRequest, Block},
    //  contract: 合约调用
    contract::{Contract, CallBuilder},
};

/// 示例1: 连接网络
async fn connect_http() {
    // HTTP 连接
    let provider = Http::from("https://eth.llamarpc.com");
    
    // 带认证的 HTTP
    // let provider = Http::from("https://mainnet.infura.io/v3/YOUR_KEY");
    
    // WSS 连接 (实时)
    // let provider = Ws::connect("wss://eth-mainnet.g.alchemy.com/v2/YOUR_KEY").await?;
}

/// 示例2: 签名者
async fn signer_example() {
    // 从私钥创建签名者
    let private_key = "0x...";
    let signer: PrivateKeySigner = private_key.parse().unwrap();
    
    // 获取地址
    let address = signer.address();
    println!("Address: {}", address);
    
    // 签名消息
    let message = "Hello, Web3!";
    let signature = signer.sign_message(message).await.unwrap();
    println!("Signature: {}", signature);
}

/// 示例3: 获取余额
async fn get_balance(provider: &Provider<Http>) {
    let address = Address::ZERO; // ETH 基金会地址
    let balance = provider.get_balance(address).await.unwrap();
    println!("Balance: {} wei", balance);
    
    // 转换为 ETH
    let balance_eth = balance / U256::from(1e18);
    println!("Balance: {} ETH", balance_eth);
}

/// 示例4: 发送交易
async fn send_transaction() {
    let provider = Http::from("https://eth.llamarpc.com");
    let signer: PrivateKeySigner = "0x...".parse().unwrap();
    
    let to = Address::ZERO;
    let value = U256::from(1e18); // 1 ETH
    
    let tx = TransactionRequest::default()
        .to(to)
        .value(value)
        .from(signer.address());
    
    let pending_tx = provider.send_transaction(tx).await.unwrap();
    let tx_hash = pending_tx.tx_hash();
    
    println!("Transaction sent: {}", tx_hash);
    
    // 等待确认
    let receipt = pending_tx.await.unwrap();
    println!("Confirmed in block: {}", receipt.block_number.unwrap());
}

/// 示例5: 合约交互 (使用预编译 ABI)
async fn contract_call() {
    let provider = Http::from("https://eth.llamarpc.com");
    
    // USDT 合约地址
    let usdt = Address::from_hex("0xdAC17F958D2ee523a2206206994597C13D831ec7").unwrap();
    
    // ERC20 ABI (简化版)
    let abi = r#"[
        {"constant":true,"inputs":[],"name":"name","outputs":[{"name":"","type":"string"}],"type":"function"},
        {"constant":true,"inputs":[],"name":"symbol","outputs":[{"name":"","type":"string"}],"type":"function"},
        {"constant":true,"inputs":[{"name":"_owner","type":"address"}],"name":"balanceOf","outputs":[{"name":"balance","type":"uint256"}],"type":"function"},
        {"constant":false,"inputs":[{"name":"_to","type":"address"},{"name":"_value","type":"uint256"}],"name":"transfer","outputs":[{"name":"","type":"bool"}],"type":"function"}
    ]"#;
    
    // 创建合约实例
    let contract = Contract::new(usdt, abi.parse().unwrap(), provider.clone());
    
    // 调用 view 函数
    let name: String = contract
        .method("name", ())
        .unwrap()
        .call()
        .await
        .unwrap()
        .0;
    
    println!("Token name: {}", name);
    
    // 查询余额
    let address = Address::ZERO;
    let balance: U256 = contract
        .method("balanceOf", address)
        .unwrap()
        .call()
        .await
        .unwrap()
        .0;
    
    println!("Balance: {} wei", balance);
}

/// 示例6: 区块监听
async fn block_listener() {
    let provider = Http::from("https://eth.llamarpc.com");
    
    // 获取最新区块号
    let block_number = provider.get_block_number().await.unwrap();
    println!("Current block: {}", block_number);
    
    // 获取区块详情
    let block: Block = provider.get_block(block_number, true).await.unwrap().unwrap();
    println!("Block hash: {}", block.hash.unwrap());
    println!("Transactions: {}", block.transactions.len());
    
    // 监听新区块 (使用 WSS)
    // let mut stream = provider.watch_blocks().await.unwrap();
    // while let Some(block_hash) = stream.next().await {
    //     println!("New block: {}", block_hash);
    // }
}

/// 示例7: 事件监听
async fn event_listener() {
    let provider = Http::from("https://eth.llamarpc.com");
    
    // USDT 转账事件 (Transfer)
    // let filter = Filter::new()
    //     .address("0xdAC17F958D2ee523a2206206994597C13D831ec7")
    //     .event("Transfer(address,address,uint256)");
    
    // let mut logs = provider.get_logs(&filter).await.unwrap();
    // for log in logs.iter() {
    //     println!("From: {:?}", log.topics[1]);
    //     println!("To: {:?}", log.topics[2]);
    //     println!("Value: {:?}", log.data);
    // }
}

/// 示例8: 多链配置
mod chains {
    use alloy::primitives::U64;
    
    pub const ETH_MAINNET: (&str, u64) = ("https://eth.llamarpc.com", 1);
    pub const ETH_SEPOLIA: (&str, u64) = ("https://sepolia.infura.io/v3/YOUR_KEY", 11155111);
    pub const BSC_MAINNET: (&str, u64) = ("https://bsc-dataseed.binance.org", 56);
    pub const POLYGON: (&str, u64) = ("https://polygon-rpc.com", 137);
    
    pub fn get_rpc(chain_id: u64) -> Option<&'static str> {
        match chain_id {
            1 => Some(ETH_MAINNET.0),
            11155111 => Some(ETH_SEPOLIA.0),
            56 => Some(BSC_MAINNET.0),
            137 => Some(POLYGON.0),
            _ => None,
        }
    }
}

/// 示例9: Gas 估算
async fn estimate_gas() {
    let provider = Http::from("https://eth.llamarpc.com");
    
    let from = Address::ZERO;
    let to = Address::from_hex("0x123").unwrap();
    let value = U256::from(1e18);
    
    let tx = TransactionRequest::default()
        .from(from)
        .to(to)
        .value(value);
    
    let gas = provider.estimate_gas(&tx).await.unwrap();
    println!("Estimated gas: {}", gas);
    
    // 获取当前 Gas 价格
    let gas_price = provider.get_gas_price().await.unwrap();
    println!("Gas price: {} wei", gas_price);
    
    // 计算总费用
    let total = gas * gas_price;
    println!("Total cost: {} wei", total);
}

/// 示例10: 签名验证 (EIP-191)
fn verify_signature(message: &str, signature: &str, expected_address: &str) -> bool {
    // EIP-191 签名
    let prefixed = format!("\x19Ethereum Signed Message:\n{}{}", message.len(), message);
    let hash = keccak256(prefixed.as_bytes());
    
    // 使用 ecrecover 恢复地址
    // 这里简化处理
    println!("Message hash: {:x}", hash);
    
    true // 实际需要使用 ecrecover
}
