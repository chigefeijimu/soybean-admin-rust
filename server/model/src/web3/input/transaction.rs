use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractCallInput {
    pub contract_id: String,
    pub method_name: String,
    pub params: Option<String>,
    pub from_address: Option<String>,
    pub value: Option<String>,
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
