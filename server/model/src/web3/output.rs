// Output DTOs for Web3 module

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletInfo {
    pub id: String,
    pub wallet_address: String,
    pub wallet_type: String,
    pub chain_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletListOutput {
    pub list: Vec<WalletInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractInfo {
    pub id: String,
    pub name: String,
    pub contract_address: String,
    pub chain_id: i32,
    pub abi: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractListOutput {
    pub list: Vec<ContractInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractDetailOutput {
    pub info: ContractInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractCallOutput {
    pub success: bool,
    pub tx_hash: Option<String>,
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    pub id: String,
    pub contract_id: Option<String>,
    pub method_name: String,
    pub params: Option<String>,
    pub tx_hash: Option<String>,
    pub status: String,
    pub from_address: Option<String>,
    pub error_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionListOutput {
    pub list: Vec<TransactionInfo>,
}
