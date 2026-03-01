// =========================================
// Batch Transfer Service
// 批量转账服务 - 支持ERC20代币和原生ETH批量转账
// =========================================

use serde::{Deserialize, Serialize};

/// 批量转账输入
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchTransferInput {
    /// 私钥（加密存储时使用）
    pub private_key: Option<String>,
    /// 转账列表
    pub transfers: Vec<TransferItem>,
    /// 链ID
    pub chain_id: Option<u64>,
    /// Gas价格（gwei）
    pub gas_price: Option<String>,
    /// 是否并行发送（批量转账时）
    pub parallel: Option<bool>,
}

/// 单笔转账项
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferItem {
    /// 收款地址
    pub to: String,
    /// 转账金额（字符串，支持高精度）
    pub amount: String,
    /// 代币地址（空字符串表示原生ETH）
    pub token_address: Option<String>,
}

/// 批量转账结果
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchTransferResult {
    /// 总笔数
    pub total_count: usize,
    /// 成功笔数
    pub success_count: usize,
    /// 失败笔数
    pub failed_count: usize,
    /// 总花费Gas
    pub total_gas_used: String,
    /// 交易哈希列表
    pub transactions: Vec<TransferTransaction>,
}

/// 单笔转账交易结果
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferTransaction {
    /// 收款地址
    pub to: String,
    /// 金额
    pub amount: String,
    /// 交易哈希
    pub tx_hash: Option<String>,
    /// 状态
    pub status: String,
    /// 错误信息
    pub error: Option<String>,
}

/// 批量转账预估
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchTransferEstimate {
    /// 总笔数
    pub total_count: usize,
    /// 总金额
    pub total_amount: String,
    /// 预估Gas总量
    pub estimated_gas: String,
    /// 预估Gas费用
    pub estimated_fee: String,
    /// 建议的Gas价格
    pub suggested_gas_price: String,
}

/// 批量转账服务
pub struct BatchTransferService;

impl BatchTransferService {
    /// 预估批量转账费用
    pub async fn estimate(input: &BatchTransferInput) -> Result<BatchTransferEstimate, String> {
        let _chain_id = input.chain_id.unwrap_or(1);
        let transfers = &input.transfers;
        let total_count = transfers.len();
        
        // 估算每笔转账的Gas（ETH转账约21000，ERC20约65000）
        let gas_per_transfer: u64 = 65000;
        let estimated_gas = (gas_per_transfer * total_count as u64).to_string();
        
        // 获取当前Gas价格（简化版本，使用默认值）
        let suggested_gas_price = "30"; // gwei
        let gas_price_wei = 30_000_000_000u64; // 30 gwei in wei
        
        let estimated_fee = (gas_price_wei * estimated_gas.parse::<u64>().unwrap_or(0)).to_string();
        
        // 计算总金额
        let mut total_amount = String::from("0");
        for transfer in transfers {
            // 简化：假设amount已经是正确的格式
            if total_amount == "0" {
                total_amount = transfer.amount.clone();
            } else {
                // 实际应该使用大数加法
                total_amount = format!("{} + {}", total_amount, transfer.amount);
            }
        }
        
        Ok(BatchTransferEstimate {
            total_count,
            total_amount,
            estimated_gas,
            estimated_fee,
            suggested_gas_price: suggested_gas_price.to_string(),
        })
    }
    
    /// 验证转账列表
    pub fn validate_transfers(transfers: &[TransferItem]) -> Result<(), String> {
        if transfers.is_empty() {
            return Err("Transfer list is empty".to_string());
        }
        
        if transfers.len() > 100 {
            return Err("Maximum 100 transfers per batch".to_string());
        }
        
        for (i, transfer) in transfers.iter().enumerate() {
            // 验证地址格式
            if !transfer.to.starts_with("0x") || transfer.to.len() != 42 {
                return Err(format!("Invalid address at index {}: {}", i, transfer.to));
            }
            
            // 验证金额
            if transfer.amount.parse::<f64>().is_err() || transfer.amount.parse::<f64>().unwrap() <= 0.0 {
                return Err(format!("Invalid amount at index {}: {}", i, transfer.amount));
            }
        }
        
        Ok(())
    }
}

/// 解析CSV格式的批量转账列表
pub fn parse_csv_transfers(csv_content: &str) -> Result<Vec<TransferItem>, String> {
    let mut transfers = Vec::new();
    
    for (i, line) in csv_content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // 跳过空行和注释
        }
        
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        
        if parts.len() < 2 {
            return Err(format!("Invalid line {}: expected address,amount", i + 1));
        }
        
        let to = parts[0].to_string();
        let amount = parts[1].to_string();
        let token_address = if parts.len() > 2 && !parts[2].is_empty() {
            Some(parts[2].to_string())
        } else {
            None
        };
        
        // 验证地址
        if !to.starts_with("0x") || to.len() != 42 {
            return Err(format!("Invalid address on line {}: {}", i + 1, to));
        }
        
        transfers.push(TransferItem {
            to,
            amount,
            token_address,
        });
    }
    
    if transfers.is_empty() {
        return Err("No valid transfers found".to_string());
    }
    
    Ok(transfers)
}

/// 批量转账请求结构（支持CSV格式）
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchTransferRequest {
    /// 私钥
    pub private_key: String,
    /// 转账列表（JSON格式）或CSV格式
    pub transfers: Vec<TransferItem>,
    /// CSV格式的转账列表
    pub csv_transfers: Option<String>,
    /// 链ID
    pub chain_id: Option<u64>,
    /// Gas价格（gwei）
    pub gas_price: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_transfers() {
        let transfers = vec![
            TransferItem {
                to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0aB".to_string(),
                amount: "0.1".to_string(),
                token_address: None,
            },
        ];
        
        assert!(BatchTransferService::validate_transfers(&transfers).is_ok());
    }
    
    #[test]
    fn test_parse_csv() {
        let csv = "0x742d35Cc6634C0532925a3b844Bc9e7595f0aB,0.1\n0x1234567890123456789012345678901234567890,1.5";
        
        let transfers = parse_csv_transfers(csv).unwrap();
        assert_eq!(transfers.len(), 2);
    }
}
