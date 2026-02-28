//! Block Scanner Service
//! Provides block and transaction scanning capabilities

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use tokio::sync::RwLock;

/// Block information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub timestamp: u64,
    pub transactions: Vec<String>,
    pub gas_used: String,
    pub gas_limit: String,
    pub miner: String,
    pub difficulty: String,
    pub total_difficulty: String,
    pub size: String,
    pub nonce: String,
    pub extra_data: String,
}

/// Transaction receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub transaction_hash: String,
    pub block_number: u64,
    pub block_hash: String,
    pub from: String,
    pub to: String,
    pub cumulative_gas_used: String,
    pub gas_used: String,
    pub contract_address: Option<String>,
    pub logs: Vec<LogEntry>,
    pub status: bool,
    pub effective_gas_price: String,
}

/// Log entry from transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,
    pub log_index: u64,
}

/// Block scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub blocks: Vec<BlockInfo>,
    pub total_transactions: usize,
    pub start_block: u64,
    pub end_block: u64,
}

/// Filter for scanning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanFilter {
    pub from_block: u64,
    pub to_block: u64,
    pub addresses: Vec<String>,
    pub topics: Vec<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
}

impl Default for ScanFilter {
    fn default() -> Self {
        Self {
            from_block: 0,
            to_block: 0,
            addresses: vec![],
            topics: vec![],
            from_address: None,
            to_address: None,
        }
    }
}

/// Cache for block data (simple implementation using HashMap)
pub struct BlockCache {
    blocks: RwLock<std::collections::HashMap<u64, BlockInfo>>,
    receipts: RwLock<std::collections::HashMap<String, TransactionReceipt>>,
}

impl BlockCache {
    pub fn new(_capacity: usize) -> Self {
        Self {
            blocks: RwLock::new(std::collections::HashMap::new()),
            receipts: RwLock::new(std::collections::HashMap::new()),
        }
    }
    
    pub async fn get_block(&self, number: u64) -> Option<BlockInfo> {
        let blocks = self.blocks.read().await;
        blocks.get(&number).cloned()
    }
    
    pub async fn insert_block(&self, block: BlockInfo) {
        let mut blocks = self.blocks.write().await;
        blocks.insert(block.number, block);
    }
    
    pub async fn get_receipt(&self, tx_hash: &str) -> Option<TransactionReceipt> {
        let receipts = self.receipts.read().await;
        receipts.get(tx_hash).cloned()
    }
    
    pub async fn insert_receipt(&self, receipt: TransactionReceipt) {
        let mut receipts = self.receipts.write().await;
        receipts.insert(receipt.transaction_hash.clone(), receipt);
    }
}

/// Block scanner service
#[allow(dead_code)]
pub struct BlockScanner {
    cache: Arc<BlockCache>,
    rpc_url: String,
}

impl BlockScanner {
    pub fn new(rpc_url: String, cache_size: usize) -> Self {
        Self {
            cache: Arc::new(BlockCache::new(cache_size)),
            rpc_url,
        }
    }
    
    /// Get block by number
    pub async fn get_block(&self, number: u64) -> Result<BlockInfo, String> {
        // Check cache first
        if let Some(block) = self.cache.get_block(number).await {
            return Ok(block);
        }
        
        // In production, call RPC
        // For demo, return mock data
        let block = BlockInfo {
            number,
            hash: format!("0x{:064x}", number * 1000),
            parent_hash: format!("0x{:064x}", (number.saturating_sub(1)) * 1000),
            timestamp: 1700000000 + number * 12,
            transactions: vec![],
            gas_used: "15000000".to_string(),
            gas_limit: "30000000".to_string(),
            miner: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb1".to_string(),
            difficulty: "1000000".to_string(),
            total_difficulty: "1000000000".to_string(),
            size: "50000".to_string(),
            nonce: "0x0000000000000000".to_string(),
            extra_data: "0x".to_string(),
        };
        
        self.cache.insert_block(block.clone()).await;
        Ok(block)
    }
    
    /// Get transaction receipt
    pub async fn get_receipt(&self, tx_hash: &str) -> Result<TransactionReceipt, String> {
        if let Some(receipt) = self.cache.get_receipt(tx_hash).await {
            return Ok(receipt);
        }
        
        // Mock receipt
        let receipt = TransactionReceipt {
            transaction_hash: tx_hash.to_string(),
            block_number: 17000000,
            block_hash: "0xabc123".to_string(),
            from: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb1".to_string(),
            to: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
            cumulative_gas_used: "100000".to_string(),
            gas_used: "21000".to_string(),
            contract_address: None,
            logs: vec![],
            status: true,
            effective_gas_price: "10000000000".to_string(),
        };
        
        self.cache.insert_receipt(receipt.clone()).await;
        Ok(receipt)
    }
    
    /// Scan blocks in range
    pub async fn scan_blocks(&self, from: u64, to: u64) -> Result<ScanResult, String> {
        let mut blocks = Vec::new();
        let mut total_tx = 0;
        
        for block_num in from..=to {
            match self.get_block(block_num).await {
                Ok(block) => {
                    total_tx += block.transactions.len();
                    blocks.push(block);
                }
                Err(e) => {
                    eprintln!("Failed to get block {}: {}", block_num, e);
                }
            }
        }
        
        Ok(ScanResult {
            blocks,
            total_transactions: total_tx,
            start_block: from,
            end_block: to,
        })
    }
    
    /// Scan for transactions matching filter
    pub async fn scan_transactions(&self, filter: ScanFilter) -> Result<Vec<TransactionReceipt>, String> {
        let mut receipts = Vec::new();
        
        for block_num in filter.from_block..=filter.to_block {
            let block = match self.get_block(block_num).await {
                Ok(b) => b,
                Err(_) => continue,
            };
            
            for tx_hash in block.transactions {
                let receipt = match self.get_receipt(&tx_hash).await {
                    Ok(r) => r,
                    Err(_) => continue,
                };
                
                // Apply filters
                if let Some(ref from) = filter.from_address {
                    if receipt.from.to_lowercase() != from.to_lowercase() {
                        continue;
                    }
                }
                
                if let Some(ref to) = filter.to_address {
                    if receipt.to.to_lowercase() != to.to_lowercase() {
                        continue;
                    }
                }
                
                if !filter.addresses.is_empty() {
                    let addr_match = filter.addresses.iter().any(|a| 
                        receipt.to.to_lowercase() == a.to_lowercase() ||
                        receipt.contract_address.as_ref().map(|c| c.to_lowercase() == a.to_lowercase()).unwrap_or(false)
                    );
                    if !addr_match {
                        continue;
                    }
                }
                
                receipts.push(receipt);
            }
        }
        
        Ok(receipts)
    }
    
    /// Get latest block number
    pub async fn get_latest_block(&self) -> Result<u64, String> {
        // In production, call eth_blockNumber
        Ok(17000000)
    }
    
    /// Get block by hash
    pub async fn get_block_by_hash(&self, _hash: &str) -> Result<Option<BlockInfo>, String> {
        // In production, query by hash
        // For demo, return None
        Ok(None)
    }
    
    /// Get transaction by hash
    pub async fn get_transaction(&self, hash: &str) -> Result<Option<TransactionReceipt>, String> {
        // In production, call eth_getTransactionByHash
        Ok(self.get_receipt(hash).await.ok())
    }
}

/// Transaction analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAnalysis {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value_eth: f64,
    pub gas_used: u64,
    pub gas_price_gwei: f64,
    pub fee_eth: f64,
    pub timestamp: u64,
    pub is_contract_call: bool,
    pub is_erc20_transfer: bool,
    pub is_erc721_transfer: bool,
    pub decoded_method: Option<String>,
}

impl TransactionAnalysis {
    pub fn from_receipt(receipt: &TransactionReceipt, value_wei: u64, timestamp: u64) -> Self {
        let gas_used: u64 = receipt.gas_used.trim_start_matches("0x")
            .parse().unwrap_or(0);
        let gas_price: u64 = receipt.effective_gas_price.trim_start_matches("0x")
            .parse().unwrap_or(0);
        
        let value_eth = value_wei as f64 / 1e18;
        let gas_price_gwei = gas_price as f64 / 1e9;
        let fee_eth = (gas_used as f64 * gas_price as f64) / 1e18;
        
        // Detect transaction type
        let is_contract_call = receipt.to != "0x0000000000000000000000000000000000000000";
        let is_erc20_transfer = receipt.logs.iter().any(|log| {
            log.topics.len() >= 3 && 
            log.topics[0] == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        });
        let is_erc721_transfer = receipt.logs.iter().any(|log| {
            log.topics.len() >= 4 &&
            log.topics[0] == "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
        });
        
        Self {
            hash: receipt.transaction_hash.clone(),
            from: receipt.from.clone(),
            to: receipt.to.clone(),
            value_eth,
            gas_used,
            gas_price_gwei,
            fee_eth,
            timestamp,
            is_contract_call,
            is_erc20_transfer,
            is_erc721_transfer,
            decoded_method: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_block_cache() {
        let cache = BlockCache::new(10);
        
        let block = BlockInfo {
            number: 17000000,
            hash: "0xabc".to_string(),
            parent_hash: "0xdef".to_string(),
            timestamp: 1700000000,
            transactions: vec![],
            gas_used: "100".to_string(),
            gas_limit: "1000".to_string(),
            miner: "0x123".to_string(),
            difficulty: "10".to_string(),
            total_difficulty: "100".to_string(),
            size: "100".to_string(),
            nonce: "0x0".to_string(),
            extra_data: "0x".to_string(),
        };
        
        cache.insert_block(block.clone()).await;
        
        let retrieved = cache.get_block(17000000).await;
        assert!(retrieved.is_some());
    }
}
