use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ContractCreateInput {
    pub name: String,
    pub contract_address: String,
    pub chain_id: i32,
    pub abi: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractUpdateInput {
    pub id: String,
    pub name: Option<String>,
    pub contract_address: Option<String>,
    pub chain_id: Option<i32>,
    pub abi: Option<String>,
    pub description: Option<String>,
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
