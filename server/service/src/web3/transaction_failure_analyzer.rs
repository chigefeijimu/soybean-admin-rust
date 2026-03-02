// Transaction Failure Analyzer Service
// 分析交易失败的原因并提供解决方案

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 交易失败类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailureType {
    InsufficientFunds,          // 资金不足
    GasTooLow,                  // Gas设置过低
    GasTooHigh,                 // Gas设置过高（超过限制）
    NonceTooLow,                // Nonce太低
    NonceTooHigh,               // Nonce太高
    ContractReverted,           // 合约调用 revert
    TokenInsufficientBalance,   // 代币余额不足
    AllowanceExceeded,          // 授权额度超限
    SlippageExceeded,           // 滑点超过阈值
    DeadlineExceeded,           // 超过截止时间
    InsufficientLiquidity,      // 流动性不足
    InvalidChainId,             // 链ID无效
    Unknown,                    // 未知错误
}

/// 失败严重程度
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,        // 低风险，可重试
    Medium,     // 中等风险，需要调整参数
    High,       // 高风险，可能导致资金损失
    Critical,   // 严重错误，需要人工介入
}

/// 解决方案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    pub title: String,
    pub description: String,
    pub action_type: String,  // "retry", "adjust_gas", "adjust_nonce", "contact_support"
    pub estimated_cost: Option<String>,
}

/// 交易失败分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionFailureAnalysis {
    pub tx_hash: String,
    pub failure_type: FailureType,
    pub severity: Severity,
    pub error_message: String,
    pub solutions: Vec<Solution>,
    pub block_number: Option<u64>,
    pub gas_used: Option<u64>,
    pub revert_reason: Option<String>,
}

/// 交易失败统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureStatistics {
    pub total_failures: u64,
    pub failure_types: HashMap<String, u64>,
    pub average_gas_wasted: u64,
    pub most_common_failure: String,
    pub recommended_gas_price: String,
}

/// 分析交易失败原因
pub fn analyze_transaction_failure(
    tx_hash: &str,
    error_message: &str,
    gas_used: Option<u64>,
    revert_data: Option<String>,
) -> TransactionFailureAnalysis {
    let revert_data_clone = revert_data.clone();
    let (failure_type, severity, solutions) = parse_failure_reason(error_message, revert_data);
    
    let revert_reason = revert_data_clone.as_ref().and_then(|data| decode_revert_reason(data));
    
    TransactionFailureAnalysis {
        tx_hash: tx_hash.to_string(),
        failure_type,
        severity,
        error_message: error_message.to_string(),
        solutions,
        block_number: None,
        gas_used,
        revert_reason,
    }
}

/// 解析失败原因
fn parse_failure_reason(
    error_message: &str,
    revert_data: Option<String>,
) -> (FailureType, Severity, Vec<Solution>) {
    let error_lower = error_message.to_lowercase();
    
    // 检查是否是常见的错误类型
    if error_lower.contains("insufficient funds") || error_lower.contains("insufficient balance") {
        (
            FailureType::InsufficientFunds,
            Severity::High,
            vec![
                Solution {
                    title: "充值资金".to_string(),
                    description: "钱包余额不足，请充值足够的代币来支付交易费用和代币金额。".to_string(),
                    action_type: "adjust_amount".to_string(),
                    estimated_cost: None,
                }
            ],
        )
    } else if error_lower.contains("gas") && error_lower.contains("too low") {
        (
            FailureType::GasTooLow,
            Severity::Medium,
            vec![
                Solution {
                    title: "增加Gas费用".to_string(),
                    description: "将Gas价格设置为当前网络平均水平的1.2-1.5倍。".to_string(),
                    action_type: "adjust_gas".to_string(),
                    estimated_cost: Some("~$5-20".to_string()),
                },
                Solution {
                    title: "使用加速器".to_string(),
                    description: "使用交易加速器服务来优先处理您的交易。".to_string(),
                    action_type: "accelerate".to_string(),
                    estimated_cost: Some("~$10-50".to_string()),
                }
            ],
        )
    } else if error_lower.contains("nonce") && error_lower.contains("too low") {
        (
            FailureType::NonceTooLow,
            Severity::Medium,
            vec![
                Solution {
                    title: "等待交易确认".to_string(),
                    description: "等待之前的交易被确认，然后重试。".to_string(),
                    action_type: "wait".to_string(),
                    estimated_cost: None,
                },
                Solution {
                    title: "取消低Nonce交易".to_string(),
                    description: "使用相同的Nonce发送0值交易来取消挂起的交易。".to_string(),
                    action_type: "cancel_nonce".to_string(),
                    estimated_cost: Some("~$5-15".to_string()),
                }
            ],
        )
    } else if error_lower.contains("nonce") && error_lower.contains("too high") {
        (
            FailureType::NonceTooHigh,
            Severity::Low,
            vec![
                Solution {
                    title: "等待Nonce同步".to_string(),
                    description: "等待钱包Nonce与网络同步，或者手动设置正确的Nonce。".to_string(),
                    action_type: "adjust_nonce".to_string(),
                    estimated_cost: None,
                }
            ],
        )
    } else if error_lower.contains("revert") || error_lower.contains("execution reverted") {
        let revert_reason = revert_data.as_ref().and_then(|d| decode_revert_reason(d));
        (
            FailureType::ContractReverted,
            Severity::High,
            generate_revert_solutions(&revert_reason),
        )
    } else if error_lower.contains("slippage") || error_lower.contains("price impact") {
        (
            FailureType::SlippageExceeded,
            Severity::Medium,
            vec![
                Solution {
                    title: "增加滑点容差".to_string(),
                    description: "在交易设置中增加滑点容差（建议1-3%）。".to_string(),
                    action_type: "adjust_slippage".to_string(),
                    estimated_cost: None,
                },
                Solution {
                    title: "分批交易".to_string(),
                    description: "将大额交易拆分为多笔小交易。".to_string(),
                    action_type: "split_trade".to_string(),
                    estimated_cost: Some("额外Gas费用".to_string()),
                }
            ],
        )
    } else if error_lower.contains("deadline") || error_lower.contains("expired") {
        (
            FailureType::DeadlineExceeded,
            Severity::Low,
            vec![
                Solution {
                    title: "延长截止时间".to_string(),
                    description: "在交易设置中增加交易的截止时间。".to_string(),
                    action_type: "adjust_deadline".to_string(),
                    estimated_cost: None,
                }
            ],
        )
    } else if error_lower.contains("liquidity") || error_lower.contains("insufficient liquidity") {
        (
            FailureType::InsufficientLiquidity,
            Severity::High,
            vec![
                Solution {
                    title: "选择其他交易对".to_string(),
                    description: "该交易对流动性不足，请选择其他交易对或DEX。".to_string(),
                    action_type: "change_pair".to_string(),
                    estimated_cost: None,
                },
                Solution {
                    title: "分批交易".to_string(),
                    description: "将大额交易拆分为多笔小交易。".to_string(),
                    action_type: "split_trade".to_string(),
                    estimated_cost: Some("额外Gas费用".to_string()),
                }
            ],
        )
    } else if error_lower.contains("allowance") || error_lower.contains("approve") {
        (
            FailureType::AllowanceExceeded,
            Severity::Medium,
            vec![
                Solution {
                    title: "增加授权额度".to_string(),
                    description: "首先进行授权交易，增加代币的授权额度。".to_string(),
                    action_type: "approve".to_string(),
                    estimated_cost: Some("~$5-15".to_string()),
                }
            ],
        )
    } else if error_lower.contains("transfer amount exceeds balance") || error_lower.contains("token balance") {
        (
            FailureType::TokenInsufficientBalance,
            Severity::Medium,
            vec![
                Solution {
                    title: "检查代币余额".to_string(),
                    description: "确认钱包中有足够的代币余额。".to_string(),
                    action_type: "check_balance".to_string(),
                    estimated_cost: None,
                }
            ],
        )
    } else {
        // 未知错误
        (
            FailureType::Unknown,
            Severity::Critical,
            vec![
                Solution {
                    title: "联系支持团队".to_string(),
                    description: "此错误无法自动解决，请联系项目支持团队。".to_string(),
                    action_type: "contact_support".to_string(),
                    estimated_cost: None,
                },
                Solution {
                    title: "查看区块链浏览器".to_string(),
                    description: "在区块链浏览器上查看详细的交易失败信息。".to_string(),
                    action_type: "view_explorer".to_string(),
                    estimated_cost: None,
                }
            ],
        )
    }
}

/// 解码Revert原因
fn decode_revert_reason(data: &str) -> Option<String> {
    // 移除0x前缀并解析
    let data = data.trim_start_matches("0x");
    
    // 常见的Revert错误签名
    let known_errors: HashMap<&str, &str> = [
        ("08c379a0", "Execution reverted: Generic error"),
        ("4e487b71", "Execution reverted: Panic error"),
        ("0xf465d3d", "Insufficient balance"),
        ("0x9dea7d59", "Insufficient allowance"),
        ("0x23d2a1a8", "Transfer failed"),
        ("0x9dc29b85", "Slippage exceeded"),
        ("0xed83a4f7", "Deadline exceeded"),
        ("0xefb5b5a4", "Insufficient liquidity"),
        ("0x5e002a5e", "Invalid token"),
        ("0x8bb2a975", "Zero amount"),
    ].iter().cloned().collect();
    
    // 尝试匹配前4字节（函数选择器）
    if data.len() >= 8 {
        let selector = &data[0..8];
        if let Some(error) = known_errors.get(selector) {
            return Some(error.to_string());
        }
        
        // 尝试解码自定义错误消息
        if data.len() > 8 {
            // 32字节偏移 + 32字节长度 = 64字节
            if data.len() >= 64 {
                let msg_len = u64::from_str_radix(&data[64..72], 16).ok()?;
                let msg_start = 72;
                let msg_end = msg_start + (msg_len as usize) * 2;
                if msg_end <= data.len() {
                    let msg_bytes = hex::decode(&data[msg_start..msg_end]).ok()?;
                    return String::from_utf8(msg_bytes).ok();
                }
            }
        }
    }
    
    None
}

/// 根据Revert原因生成解决方案
fn generate_revert_solutions(revert_reason: &Option<String>) -> Vec<Solution> {
    if let Some(reason) = revert_reason {
        let reason_lower = reason.to_lowercase();
        
        if reason_lower.contains("balance") || reason_lower.contains("insufficient") {
            return vec![
                Solution {
                    title: "检查余额".to_string(),
                    description: "确保钱包中有足够的代币余额。".to_string(),
                    action_type: "check_balance".to_string(),
                    estimated_cost: None,
                },
                Solution {
                    title: "增加授权".to_string(),
                    description: "如果是与合约交互，可能需要先授权代币。".to_string(),
                    action_type: "approve".to_string(),
                    estimated_cost: Some("~$5-15".to_string()),
                }
            ];
        }
        
        if reason_lower.contains("slippage") || reason_lower.contains("price") {
            return vec![
                Solution {
                    title: "调整滑点".to_string(),
                    description: "增加滑点容差或等待更有利的价格。".to_string(),
                    action_type: "adjust_slippage".to_string(),
                    estimated_cost: None,
                }
            ];
        }
    }
    
    vec![
        Solution {
            title: "检查合约状态".to_string(),
            description: "确认合约正常运行且未暂停。".to_string(),
            action_type: "check_contract".to_string(),
            estimated_cost: None,
        },
        Solution {
            title: "重试交易".to_string(),
            description: "等待片刻后重试交易。".to_string(),
            action_type: "retry".to_string(),
            estimated_cost: Some("Gas费用".to_string()),
        }
    ]
}

/// 获取失败统计信息
pub fn get_failure_statistics(failures: &[TransactionFailureAnalysis]) -> FailureStatistics {
    let mut failure_types: HashMap<String, u64> = HashMap::new();
    let mut total_gas: u64 = 0;
    let mut gas_count: u64 = 0;
    
    for failure in failures {
        let type_name = format!("{:?}", failure.failure_type);
        *failure_types.entry(type_name).or_insert(0) += 1;
        
        if let Some(gas) = failure.gas_used {
            total_gas += gas;
            gas_count += 1;
        }
    }
    
    let most_common_failure = failure_types
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(k, _)| k.clone())
        .unwrap_or_else(|| "None".to_string());
    
    FailureStatistics {
        total_failures: failures.len() as u64,
        failure_types,
        average_gas_wasted: if gas_count > 0 { total_gas / gas_count } else { 0 },
        most_common_failure,
        recommended_gas_price: "查看当前Gas价格建议".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyze_insufficient_funds() {
        let result = analyze_transaction_failure(
            "0x123",
            "insufficient funds for transfer",
            Some(21000),
            None,
        );
        
        assert!(matches!(result.failure_type, FailureType::InsufficientFunds));
    }
    
    #[test]
    fn test_analyze_gas_too_low() {
        let result = analyze_transaction_failure(
            "0x456",
            "gas too low",
            Some(21000),
            None,
        );
        
        assert!(matches!(result.failure_type, FailureType::GasTooLow));
    }
    
    #[test]
    fn test_analyze_nonce_too_low() {
        let result = analyze_transaction_failure(
            "0x789",
            "nonce too low",
            Some(21000),
            None,
        );
        
        assert!(matches!(result.failure_type, FailureType::NonceTooLow));
    }
}
