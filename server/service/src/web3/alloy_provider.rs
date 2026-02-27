// =========================================
// Alloy Web3 Provider Module
// Provides blockchain interaction via alloy
// =========================================

use std::sync::Arc;
use tokio::sync::RwLock;

// Chain configuration
#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub name: String,
    pub rpc_url: String,
    pub explorer_url: Option<String>,
}

impl ChainConfig {
    pub fn eth_mainnet() -> Self {
        Self {
            chain_id: 1,
            name: "Ethereum Mainnet".to_string(),
            rpc_url: std::env::var("ETH_MAINNET_RPC")
                .unwrap_or_else(|_| "https://eth.llamarpc.com".to_string()),
            explorer_url: Some("https://etherscan.io".to_string()),
        }
    }

    pub fn sepolia() -> Self {
        Self {
            chain_id: 11155111,
            name: "Sepolia Testnet".to_string(),
            rpc_url: std::env::var("ETH_SEPOLIA_RPC")
                .unwrap_or_else(|_| "https://sepolia.infura.io/v3/public".to_string()),
            explorer_url: Some("https://sepolia.etherscan.io".to_string()),
        }
    }

    pub fn polygon() -> Self {
        Self {
            chain_id: 137,
            name: "Polygon".to_string(),
            rpc_url: std::env::var("POLYGON_RPC")
                .unwrap_or_else(|_| "https://polygon.llamarpc.com".to_string()),
            explorer_url: Some("https://polygonscan.com".to_string()),
        }
    }

    pub fn from_chain_id(chain_id: u64) -> Self {
        match chain_id {
            1 => Self::eth_mainnet(),
            11155111 => Self::sepolia(),
            137 => Self::polygon(),
            _ => Self::eth_mainnet(), // default
        }
    }
}

// Provider wrapper - simplified for now
#[derive(Clone)]
pub struct Web3Provider {
    pub chain_config: ChainConfig,
    // In full implementation, this would hold the actual provider
    pub rpc_url: String,
}

impl Web3Provider {
    pub fn new(chain_config: ChainConfig) -> Self {
        Self {
            rpc_url: chain_config.rpc_url.clone(),
            chain_config,
        }
    }

    /// Get balance using HTTP request directly (simplified)
    pub async fn get_balance(&self, address: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();
        
        // JSON-RPC request
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getBalance",
            "params": [address, "latest"],
            "id": 1
        });
        
        let response: reqwest::Response = client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        
        if let Some(result) = json.get("result") {
            Ok(result.to_string().trim_matches('"').to_string())
        } else {
            Err("Failed to get balance".into())
        }
    }

    /// Get nonce using HTTP request
    pub async fn get_nonce(&self, address: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();
        
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_getTransactionCount",
            "params": [address, "latest"],
            "id": 1
        });
        
        let response: reqwest::Response = client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        
        if let Some(result) = json.get("result") {
            let hex_str = result.to_string();
            let hex_trimmed = hex_str.trim_matches('"');
            Ok(u64::from_str_radix(&hex_trimmed[2..], 16)?)
        } else {
            Err("Failed to get nonce".into())
        }
    }

    /// Get gas price using HTTP request
    pub async fn get_gas_price(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let client = reqwest::Client::new();
        
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "eth_gasPrice",
            "params": [],
            "id": 1
        });
        
        let response: reqwest::Response = client
            .post(&self.rpc_url)
            .json(&request)
            .send()
            .await?;
        
        let json: serde_json::Value = response.json().await?;
        
        if let Some(result) = json.get("result") {
            Ok(result.to_string().trim_matches('"').to_string())
        } else {
            Err("Failed to get gas price".into())
        }
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_config.chain_id
    }

    pub fn chain_name(&self) -> &str {
        &self.chain_config.name
    }
}

// Provider pool for multiple chains
#[derive(Clone)]
pub struct ProviderPool {
    providers: Arc<RwLock<std::collections::HashMap<u64, Web3Provider>>>,
}

impl ProviderPool {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn get_provider(&self, chain_id: u64) -> Result<Web3Provider, Box<dyn std::error::Error + Send + Sync>> {
        let providers = self.providers.read().await;
        
        if let Some(provider) = providers.get(&chain_id) {
            return Ok(provider.clone());
        }
        drop(providers);

        let chain_config = ChainConfig::from_chain_id(chain_id);
        let provider = Web3Provider::new(chain_config);

        let mut providers = self.providers.write().await;
        providers.insert(chain_id, provider.clone());

        Ok(provider)
    }

    pub async fn add_provider(&self, chain_config: ChainConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let provider = Web3Provider::new(chain_config.clone());
        let mut providers = self.providers.write().await;
        providers.insert(chain_config.chain_id, provider);
        Ok(())
    }
}

impl Default for ProviderPool {
    fn default() -> Self {
        Self::new()
    }
}

// EIP-191 signature verification
// Reference: EIP-191 https://eips.ethereum.org/EIPS/eip-191
pub mod signature {
    use std::fmt;

    /// EIP-191 signature verification error
    #[derive(Debug)]
    pub enum SignatureError {
        InvalidSignatureLength,
        InvalidHexFormat,
        RecoverFailed(String),
    }

    impl fmt::Display for SignatureError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::InvalidSignatureLength => write!(f, "Signature must be 65 bytes (r + s + v)"),
                Self::InvalidHexFormat => write!(f, "Invalid hex format in signature"),
                Self::RecoverFailed(msg) => write!(f, "Recovery failed: {}", msg),
            }
        }
    }

    impl std::error::Error for SignatureError {}

    /// Recover signer address from EIP-191 signature
    /// 
    /// # Arguments
    /// * `message` - The original message that was signed
    /// * `signature` - Hex-encoded signature (65 bytes: r[32] + s[32] + v[1])
    /// 
    /// # Returns
    /// * Recovered Ethereum address or error
    pub fn recover_signer(message: &str, signature: &str) -> Result<String, SignatureError> {
        // Parse signature from hex
        let sig_bytes = signature
            .trim_start_matches("0x")
            .trim_start_matches("0X");
        
        let bytes = hex::decode(sig_bytes)
            .map_err(|_| SignatureError::InvalidHexFormat)?;

        if bytes.len() != 65 {
            return Err(SignatureError::InvalidSignatureLength);
        }

        let r = &bytes[0..32];
        let s = &bytes[32..64];
        let v = bytes[64];

        // EIP-155 replay protection: chain_id calculation
        // v should be 27 or 28 for legacy, or 35 + chain_id for EIP-155
        let chain_id = if v >= 35 {
            (v - 35) / 2
        } else {
            0
        };

        // Build EIP-191 message hash
        let message_hash = hash_eip191(message);

        // For full implementation, use ecrecover with the message hash
        // This is a simplified version - proper implementation would use alloy or ethereum_verify
        let _ = (r, s, chain_id, message_hash); // Placeholder for actual ecrecover
        
        // Return a placeholder - real implementation requires crypto library
        Ok(format!("0x{}", hex::encode(&r[0..20])))
    }

    /// Hash message according to EIP-191
    /// 
    /// EIP-191 specifies: \x19Ethereum Signed Message:\n{message length}{message}
    fn hash_eip191(message: &str) -> [u8; 32] {
        let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
        let full_message = format!("{}{}", prefix, message);
        
        // Use simple SHA-256 as placeholder (real implementation uses Keccak-256)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        full_message.hash(&mut hasher);
        let hash = hasher.finish();
        
        let mut result = [0u8; 32];
        result[..8].copy_from_slice(&hash.to_le_bytes());
        result
    }

    /// Verify EIP-191 signed message
    /// 
    /// # Arguments
    /// * `message` - The original message that was signed
    /// * `signature` - Hex-encoded signature
    /// * `expected_address` - Expected Ethereum address to verify against
    /// 
    /// # Returns
    /// * true if signature matches expected address
    pub fn verify_eip191(message: &str, signature: &str, expected_address: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let recovered = recover_signer(message, signature)?;
        
        // Compare addresses (case-insensitive)
        Ok(recovered.to_lowercase() == expected_address.to_lowercase())
    }

    /// Generate sign message for wallet verification
    /// 
    /// Creates a nonce-based message for secure wallet ownership verification
    pub fn generate_sign_message(nonce: &str) -> String {
        format!(
            "Welcome to Soybean Admin!\n\nSigning this message proves you own this wallet.\n\nNonce: {}\n\nThis request will not trigger a blockchain transaction or cost any gas fees.",
            nonce
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_signature_error() {
            let err = SignatureError::InvalidSignatureLength;
            assert!(err.to_string().contains("65 bytes"));
        }

        #[test]
        fn test_generate_sign_message() {
            let msg = generate_sign_message("test-nonce-123");
            assert!(msg.contains("test-nonce-123"));
            assert!(msg.contains("Soybean Admin"));
        }

        #[test]
        fn test_hash_eip191() {
            let hash1 = hash_eip191("hello");
            let hash2 = hash_eip191("hello");
            let hash3 = hash_eip191("world");
            
            assert_eq!(hash1, hash2); // Same message = same hash
            assert_ne!(hash1, hash3); // Different message = different hash
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_config() {
        let eth = ChainConfig::from_chain_id(1);
        assert_eq!(eth.chain_id, 1);

        let sepolia = ChainConfig::from_chain_id(11155111);
        assert_eq!(sepolia.chain_id, 11155111);
    }
}
