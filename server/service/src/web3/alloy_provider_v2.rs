// =========================================
// Alloy-based Provider Implementation
// Simplified version using alloy for basic operations
// =========================================

use alloy::primitives::U256;

// Chain configurations with proper RPC URLs
pub mod chains {

    #[derive(Debug, Clone)]
    pub struct ChainInfo {
        pub chain_id: u64,
        pub name: &'static str,
        pub rpc_url: &'static str,
        pub explorer: &'static str,
    }

    impl ChainInfo {
        pub const ETHEREUM: Self = Self {
            chain_id: 1,
            name: "Ethereum Mainnet",
            rpc_url: "https://eth.llamarpc.com",
            explorer: "https://etherscan.io",
        };

        pub const SEPOLIA: Self = Self {
            chain_id: 11155111,
            name: "Sepolia Testnet",
            rpc_url: "https://sepolia.infura.io/v3/public",
            explorer: "https://sepolia.etherscan.io",
        };

        pub const POLYGON: Self = Self {
            chain_id: 137,
            name: "Polygon",
            rpc_url: "https://polygon.llamarpc.com",
            explorer: "https://polygonscan.com",
        };

        pub const ARBITRUM: Self = Self {
            chain_id: 42161,
            name: "Arbitrum One",
            rpc_url: "https://arb1.arbitrum.io/rpc",
            explorer: "https://arbiscan.io",
        };

        pub const OPTIMISM: Self = Self {
            chain_id: 10,
            name: "Optimism",
            rpc_url: "https://mainnet.optimism.io",
            explorer: "https://optimistic.etherscan.io",
        };

        pub fn from_id(chain_id: u64) -> Option<Self> {
            match chain_id {
                1 => Some(Self::ETHEREUM),
                11155111 => Some(Self::SEPOLIA),
                137 => Some(Self::POLYGON),
                42161 => Some(Self::ARBITRUM),
                10 => Some(Self::OPTIMISM),
                _ => None,
            }
        }
    }
}

/// Alloy-powered Web3 Provider wrapper
#[derive(Clone)]
pub struct AlloyProvider {
    rpc_url: String,
    chain_info: chains::ChainInfo,
}

impl AlloyProvider {
    /// Create a new alloy provider from RPC URL
    pub fn new(rpc_url: &str, chain_id: u64) -> Self {
        let chain_info = chains::ChainInfo::from_id(chain_id)
            .unwrap_or(chains::ChainInfo::ETHEREUM);
        
        Self {
            rpc_url: rpc_url.to_string(),
            chain_info,
        }
    }

    /// Get RPC URL
    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }

    /// Get chain ID
    pub fn chain_id(&self) -> u64 {
        self.chain_info.chain_id
    }

    /// Get chain name
    pub fn chain_name(&self) -> &str {
        self.chain_info.name
    }

    /// Get explorer URL
    pub fn explorer_url(&self) -> &str {
        self.chain_info.explorer
    }

    /// Get native balance for an address
    pub async fn get_balance(&self, address: &str) -> Result<U256, Box<dyn std::error::Error + Send + Sync>> {
        use reqwest::Client;
        use serde_json::{json, Value};
        
        let client = Client::new();
        
        let request = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBalance",
            "params": [address, "latest"],
            "id": 1
        });
        
        let response = client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let result = json["result"]
            .as_str()
            .ok_or("No result")?;
        
        let balance = U256::from_str_radix(&result[2..], 16)?;
        
        Ok(balance)
    }

    /// Get nonce for an address
    pub async fn get_nonce(&self, address: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use reqwest::Client;
        use serde_json::{json, Value};
        
        let client = Client::new();
        
        let request = json!({
            "jsonrpc": "2.0",
            "method": "eth_getTransactionCount",
            "params": [address, "latest"],
            "id": 1
        });
        
        let response = client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let result = json["result"]
            .as_str()
            .ok_or("No result")?;
        
        let nonce = u64::from_str_radix(&result[2..], 16)?;
        
        Ok(nonce)
    }

    /// Get gas price
    pub async fn get_gas_price(&self) -> Result<U256, Box<dyn std::error::Error + Send + Sync>> {
        use reqwest::Client;
        use serde_json::{json, Value};
        
        let client = Client::new();
        
        let request = json!({
            "jsonrpc": "2.0",
            "method": "eth_gasPrice",
            "params": [],
            "id": 1
        });
        
        let response = client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let result = json["result"]
            .as_str()
            .ok_or("No result")?;
        
        let gas_price = U256::from_str_radix(&result[2..], 16)?;
        
        Ok(gas_price)
    }

    /// Get current block number
    pub async fn get_block_number(&self) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use reqwest::Client;
        use serde_json::{json, Value};
        
        let client = Client::new();
        
        let request = json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 1
        });
        
        let response = client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;
        
        let json: Value = response.json().await?;
        
        let result = json["result"]
            .as_str()
            .ok_or("No result")?;
        
        let block = u64::from_str_radix(&result[2..], 16)?;
        
        Ok(block)
    }
}

/// Provider pool for multiple chains
#[derive(Clone)]
pub struct ProviderPool {
    providers: std::collections::HashMap<u64, AlloyProvider>,
}

impl ProviderPool {
    pub fn new() -> Self {
        Self {
            providers: std::collections::HashMap::new(),
        }
    }

    pub fn get_provider(&self, chain_id: u64) -> Option<AlloyProvider> {
        self.providers.get(&chain_id).cloned()
    }

    pub fn add_provider(&mut self, provider: AlloyProvider) {
        self.providers.insert(provider.chain_id(), provider);
    }

    /// Get or create provider for chain
    pub fn get_or_create(&mut self, chain_id: u64) -> AlloyProvider {
        if let Some(provider) = self.providers.get(&chain_id) {
            return provider.clone();
        }
        
        let chain_info = chains::ChainInfo::from_id(chain_id)
            .unwrap_or(chains::ChainInfo::ETHEREUM);
        let provider = AlloyProvider::new(chain_info.rpc_url, chain_id);
        self.providers.insert(chain_id, provider.clone());
        provider
    }
}

impl Default for ProviderPool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_info() {
        let eth = chains::ChainInfo::from_id(1).unwrap();
        assert_eq!(eth.chain_id, 1);
        assert_eq!(eth.name, "Ethereum Mainnet");

        let sepolia = chains::ChainInfo::from_id(11155111).unwrap();
        assert_eq!(sepolia.chain_id, 11155111);
    }

    #[test]
    fn test_provider_pool() {
        let mut pool = ProviderPool::new();
        let provider = AlloyProvider::new("https://eth.llamarpc.com", 1);
        pool.add_provider(provider);

        let retrieved = pool.get_provider(1);
        assert!(retrieved.is_some());
    }
}
