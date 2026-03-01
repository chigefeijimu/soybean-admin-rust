// =========================================
// NFT Service Module
// =========================================

use serde::{Deserialize, Serialize};
use crate::web3::contract_call_impl::execute_contract_read;
use crate::web3::PROVIDER_POOL;

/// NFT 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTInfo {
    pub token_id: String,
    pub contract_address: String,
    pub owner: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub collection: Option<String>,
    pub token_uri: Option<String>,
    pub attributes: Option<Vec<NFTAttribute>>,
}

/// NFT 属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTAttribute {
    pub trait_type: String,
    pub value: String,
}

/// ERC721 所有者查询结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerOfResult {
    pub owner: String,
    pub token_id: String,
}

/// NFT 服务
pub struct NFTService;

impl NFTService {
    /// 查询 ERC721 代币的 ownerOf
    pub async fn owner_of(contract: &str, token_id: &str, chain_id: u64) -> Result<String, String> {
        let provider = PROVIDER_POOL.get_provider(chain_id).await
            .map_err(|e| e.to_string())?;
        
        // 调用 ownerOf 方法
        let result = execute_contract_read(
            &provider.rpc_url,
            contract,
            "ownerOf",
            vec![token_id.to_string()],
        ).await
            .map_err(|e| e.to_string())?;
        
        // 解析返回地址 (bytes32 转 address)
        if result.len() >= 32 {
            let address = &result[24..64]; // 移除前面的 padding
            let addr_str = address.trim_start_matches('0');
            Ok(format!("0x{}", addr_str))
        } else if result.len() >= 40 {
            let address = &result[24..];
            Ok(format!("0x{}", address))
        } else {
            Ok(result)
        }
    }
    
    /// 查询 ERC721 代币的 tokenURI
    pub async fn token_uri(contract: &str, token_id: &str, chain_id: u64) -> Result<String, String> {
        let provider = PROVIDER_POOL.get_provider(chain_id).await
            .map_err(|e| e.to_string())?;
        
        // 调用 tokenURI 方法
        let result = execute_contract_read(
            &provider.rpc_url,
            contract,
            "tokenURI",
            vec![token_id.to_string()],
        ).await
            .map_err(|e| e.to_string())?;
        
        // 解析返回的字符串 (格式: 0x + offset + length + string)
        // 简化处理：直接返回原始结果
        if result.starts_with("0x") && result.len() > 64 {
            // 这是 bytes 编码，需要解码
            let data = &result[2..];
            if data.len() >= 64 {
                let offset = usize::from_str_radix(&data[0..64], 16).unwrap_or(0);
                if data.len() > offset * 2 + 64 {
                    let len = usize::from_str_radix(&data[64..96], 16).unwrap_or(0);
                    let string_start = 96 + offset * 2;
                    if data.len() >= string_start + len * 2 {
                        let string_data = &data[string_start..string_start + len * 2];
                        return Ok(decode_string(string_data));
                    }
                }
            }
        }
        
        // 如果解析失败，尝试直接返回结果
        Ok(result)
    }
    
    /// 批量查询 NFT 持有者
    pub async fn get_owners_batch(contract: &str, token_ids: &[String], chain_id: u64) -> Result<Vec<OwnerOfResult>, String> {
        let mut results = Vec::new();
        
        for token_id in token_ids {
            match Self::owner_of(contract, token_id, chain_id).await {
                Ok(owner) => {
                    results.push(OwnerOfResult {
                        owner,
                        token_id: token_id.clone(),
                    });
                }
                Err(e) => {
                    tracing::warn!("Failed to get owner for token {}: {}", token_id, e);
                }
            }
        }
        
        Ok(results)
    }
    
    /// 从 tokenURI 获取 NFT 元数据
    pub async fn fetch_metadata(token_uri: &str) -> Result<NFTInfo, String> {
        // 支持 ipfs:// 和 https:// 协议
        let url = if token_uri.starts_with("ipfs://") {
            let ipfs_hash = token_uri.trim_start_matches("ipfs://");
            // 处理 ipfs 路径格式
            if ipfs_hash.starts_with("ipfs/") {
                format!("https://ipfs.io/{}", ipfs_hash)
            } else {
                format!("https://ipfs.io/ipfs/{}", ipfs_hash)
            }
        } else {
            token_uri.to_string()
        };
        
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        
        let metadata: serde_json::Value = response
            .json()
            .await
            .map_err(|e| e.to_string())?;
        
        let attributes = metadata.get("attributes")
            .and_then(|a| a.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|attr| {
                        let trait_type = attr.get("trait_type")?.as_str()?.to_string();
                        let value = attr.get("value")?.as_str()?.to_string();
                        Some(NFTAttribute { trait_type, value })
                    })
                    .collect()
            });
        
        Ok(NFTInfo {
            token_id: metadata.get("id")
                .or_else(|| metadata.get("token_id"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            contract_address: "".to_string(),
            owner: "".to_string(),
            name: metadata.get("name").and_then(|v| v.as_str()).map(String::from),
            description: metadata.get("description").and_then(|v| v.as_str()).map(String::from),
            image_url: metadata.get("image").and_then(|v| v.as_str()).map(String::from),
            collection: metadata.get("collection")
                .or_else(|| metadata.get("name"))
                .and_then(|v| v.as_str())
                .map(String::from),
            token_uri: Some(token_uri.to_string()),
            attributes,
        })
    }
    
    /// 查询多个 NFT 的详细信息
    pub async fn get_nfts(contract: &str, token_ids: &[String], chain_id: u64) -> Result<Vec<NFTInfo>, String> {
        let mut nfts = Vec::new();
        
        for token_id in token_ids {
            // 获取 tokenURI
            let token_uri = match Self::token_uri(contract, token_id, chain_id).await {
                Ok(uri) => uri,
                Err(e) => {
                    tracing::warn!("Failed to get tokenURI for {}: {}", token_id, e);
                    continue;
                }
            };
            
            // 获取元数据
            match Self::fetch_metadata(&token_uri).await {
                Ok(mut info) => {
                    info.token_id = token_id.clone();
                    info.contract_address = contract.to_string();
                    nfts.push(info);
                }
                Err(e) => {
                    tracing::warn!("Failed to fetch metadata for {}: {}", token_id, e);
                }
            }
        }
        
        Ok(nfts)
    }
}

/// 解码 bytes 编码的字符串
fn decode_string(data: &str) -> String {
    // 尝试将 hex 数据解码为字符串
    match hex::decode(data) {
        Ok(bytes) => {
            String::from_utf8_lossy(&bytes).to_string()
        }
        Err(_) => {
            // 如果解码失败，返回原始 hex
            format!("0x{}", data)
        }
    }
}
