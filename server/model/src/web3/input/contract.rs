use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
pub struct ContractCallInput {
    pub contract_id: String,
    pub method_name: String,
    pub params: Option<String>,
    pub from_address: Option<String>,
    pub value: Option<String>,
}
