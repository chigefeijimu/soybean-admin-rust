use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletVerifyInput {
    pub wallet_address: String,
    pub signature: String,
    pub message: String,
    pub wallet_type: Option<String>,
    pub chain_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletVerifyOutput {
    pub success: bool,
    pub message: String,
    pub wallet: Option<WalletInfo>,
}

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
pub struct WalletListInput {
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletListOutput {
    pub list: Vec<WalletInfo>,
}
