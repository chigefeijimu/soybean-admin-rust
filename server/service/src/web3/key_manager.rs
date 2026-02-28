// =========================================
// 私钥管理模块
// 用于安全存储私钥并发送交易
// =========================================

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

// 派生密钥的迭代次数
const PBKDF2_ITERATIONS: u32 = 100_000;
// 盐的长度
const SALT_LENGTH: usize = 16;
// Nonce 的长度 (AES-GCM 使用 12 字节)
const NONCE_LENGTH: usize = 12;

#[derive(Debug, Serialize, Deserialize)]
/// 加密后的私钥结构
pub struct EncryptedPrivateKey {
    /// Base64 编码的盐
    pub salt: String,
    /// Base64 编码的加密私钥
    pub ciphertext: String,
    /// Base64 编码的 Nonce
    pub nonce: String,
}

/// 私钥加密器
pub struct PrivateKeyEncryptor {
    key: [u8; 32], // AES-256 需要 32 字节密钥
}

impl PrivateKeyEncryptor {
    /// 使用密码创建加密器
    pub fn new(password: &str) -> Self {
        let mut key = [0u8; 32];
        
        // 使用固定盐进行密钥派生（简化实现）
        // 在生产环境中，应该使用随机盐并存储
        let salt = b"soybean-admin-web3-salt";
        pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
        
        Self { key }
    }
    
    /// 加密私钥
    pub fn encrypt(&self, private_key: &str) -> Result<EncryptedPrivateKey, String> {
        // 生成随机盐和 nonce
        let mut salt = [0u8; SALT_LENGTH];
        let mut nonce_bytes = [0u8; NONCE_LENGTH];
        OsRng.fill_bytes(&mut salt);
        OsRng.fill_bytes(&mut nonce_bytes);
        
        // 创建 AES-GCM 密码
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;
        
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // 加密私钥
        let ciphertext = cipher
            .encrypt(nonce, private_key.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;
        
        Ok(EncryptedPrivateKey {
            salt: BASE64.encode(salt),
            ciphertext: BASE64.encode(ciphertext),
            nonce: BASE64.encode(nonce_bytes),
        })
    }
    
    /// 解密私钥
    pub fn decrypt(&self, encrypted: &EncryptedPrivateKey) -> Result<String, String> {
        let _salt = BASE64.decode(&encrypted.salt)
            .map_err(|e| format!("Failed to decode salt: {}", e))?;
        let ciphertext = BASE64.decode(&encrypted.ciphertext)
            .map_err(|e| format!("Failed to decode ciphertext: {}", e))?;
        let nonce_bytes = BASE64.decode(&encrypted.nonce)
            .map_err(|e| format!("Failed to decode nonce: {}", e))?;
        
        // 创建 AES-GCM 密码
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;
        
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // 解密私钥
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| format!("Decryption failed: {}", e))?;
        
        String::from_utf8(plaintext)
            .map_err(|e| format!("Failed to convert to string: {}", e))
    }
}

/// 简单的私钥验证函数
pub fn is_valid_private_key(key: &str) -> bool {
    let key = key.trim();
    // 私钥应该是 64 个十六进制字符 (不带 0x) 或 66 个字符 (带 0x)
    let key_without_prefix = key.strip_prefix("0x").unwrap_or(key);
    key_without_prefix.len() == 64 && key_without_prefix.chars().all(|c| c.is_ascii_hexdigit())
}

/// 从私钥获取地址
pub fn private_key_to_address(private_key: &str) -> Result<String, String> {
    let key = private_key.strip_prefix("0x").unwrap_or(private_key);
    let key_bytes = hex::decode(key)
        .map_err(|e| format!("Invalid private key hex: {}", e))?;
    
    // 使用 alloy 的 signers 来获取地址
    use alloy::signers::local::PrivateKeySigner;
    
    let signer = PrivateKeySigner::from_slice(&key_bytes)
        .map_err(|_| "Invalid private key")?;
    
    Ok(format!("0x{:x}", signer.address()))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encrypt_decrypt() {
        let encryptor = PrivateKeyEncryptor::new("test-password");
        let private_key = "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";
        
        let encrypted = encryptor.encrypt(private_key).unwrap();
        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        
        assert_eq!(private_key, decrypted);
    }
    
    #[test]
    fn test_valid_private_key() {
        assert!(is_valid_private_key("0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"));
        assert!(is_valid_private_key("abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"));
        assert!(!is_valid_private_key("invalid"));
        assert!(!is_valid_private_key("0x123"));
    }
}
