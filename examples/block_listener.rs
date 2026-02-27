// =========================================
// 区块实时监听示例
// =========================================

use alloy::{
    providers::{Provider, Http, Ws},
    primitives::U64,
    rpc::types::Filter,
};

/// 简单轮询监听新区块
async fn poll_new_blocks() {
    let provider = Http::from("https://eth.llamarpc.com");
    
    let mut current_block = provider.get_block_number().await.unwrap();
    println!("Starting from block: {}", current_block);
    
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;
        
        let new_block = provider.get_block_number().await.unwrap();
        
        if new_block > current_block {
            println!("New block: {} (was {})", new_block, current_block);
            current_block = new_block;
        }
    }
}

/// 获取区块详情
async fn get_block_details() {
    let provider = Http::from("https://eth.llamarpc.com");
    
    let block_number = provider.get_block_number().await.unwrap();
    
    // 获取完整区块信息
    let block = provider
        .get_block(block_number, true)
        .await
        .unwrap()
        .unwrap();
    
    println!("Block #{}", block.number.unwrap());
    println!("Hash: {}", block.hash.unwrap());
    println!("Parent: {}", block.parent_hash);
    println!("Timestamp: {}", block.timestamp);
    println!("Transactions: {}", block.transactions.len());
    println!("Gas used: {}", block.gas_used);
}

/// 监听合约事件
async fn listen_contract_events() {
    let provider = Http::from("https://eth.llamarpc.com");
    
    // USDT Transfer 事件
    // Event signature: Transfer(address,address,uint256)
    // Topics: 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df35b9cdc
    
    let filter = Filter::new()
        .address("0xdAC17F958D2ee523a2206206994597C13D831ec7".parse().unwrap())
        .from_block(19000000u64);
    
    let logs = provider.get_logs(&filter).await.unwrap();
    
    for log in logs.iter().take(10) {
        println!("Block: {:?}", log.block_number);
        println!("From: {:?}", log.topics[1]);
        println!("To: {:?}", log.topics[2]);
        println!("Value: {:?}", log.data);
    }
}

/// 监听多个代币转账
async fn multi_token_listener() {
    let provider = Http::from("https://eth.llamarpc.com");
    
    let tokens = [
        "0xdAC17F958D2ee523a2206206994597C13D831ec7", // USDT
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // USDC
        "0x6B175474E89094C44Da98b954EesadcdEF9ce6CC", // DAI
    ];
    
    for token in tokens {
        let address = token.parse().unwrap();
        
        let filter = Filter::new()
            .address(address)
            .from_block(19000000u64);
        
        match provider.get_logs(&filter).await {
            Ok(logs) => {
                println!("{}: {} transfer events", token, logs.len());
            }
            Err(e) => {
                println!("Error fetching {}: {}", token, e);
            }
        }
    }
}

/// 交易池监控
async fn mempool_monitor() {
    // 注意: 这需要访问 pending 交易
    // 大多数 RPC 不支持，需使用特殊服务
    
    let provider = Http::from("https://eth.llamarpc.com");
    
    // 获取 pending 交易
    // let pending = provider.get_pending_transactions().await?;
    
    println!("Mempool monitoring requires WebSocket connection");
}

/// 自定义类型定义
mod types {
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BlockInfo {
        pub number: u64,
        pub hash: String,
        pub parent_hash: String,
        pub timestamp: u64,
        pub transactions: Vec<String>,
        pub gas_used: u64,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TransferEvent {
        pub block_number: u64,
        pub from: String,
        pub to: String,
        pub value: String,
        pub tx_hash: String,
    }
}
