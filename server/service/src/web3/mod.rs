// =========================================
// Web3 Service Module
// =========================================

use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use std::sync::Arc;
use sea_orm::DatabaseConnection;

use crate::helper::db_helper;

// ============ Error Type ============
#[derive(Debug)]
pub struct ServiceError {
    pub code: i32,
    pub message: String,
}

impl ServiceError {
    pub fn new(msg: &str) -> Self {
        Self { code: 500, message: msg.to_string() }
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ServiceError {}

// ============ Input Types ============
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletVerifyInput {
    pub wallet_address: String,
    pub signature: String,
    pub message: String,
    pub wallet_type: Option<String>,
    pub chain_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletListInput {
    pub user_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractCreateInput {
    pub name: String,
    pub contract_address: String,
    pub chain_id: i32,
    pub abi: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractUpdateInput {
    pub id: String,
    pub name: Option<String>,
    pub contract_address: Option<String>,
    pub chain_id: Option<i32>,
    pub abi: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractCallInput {
    pub contract_id: String,
    pub method_name: String,
    pub params: Option<String>,
    pub from_address: Option<String>,
    pub value: Option<String>,
}

// ============ Output Types ============
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

// ============ Helper ============
async fn get_db() -> Result<Arc<DatabaseConnection>, ServiceError> {
    match db_helper::get_db_connection().await {
        Ok(conn) => Ok(conn),
        Err(e) => Err(ServiceError::new(&format!("DB error: {:?}", e))),
    }
}

// ============ Wallet Service ============
#[async_trait]
pub trait TWalletService {
    async fn verify_wallet(&self, input: WalletVerifyInput) -> Result<WalletInfo, ServiceError>;
    async fn list_wallets(&self, input: WalletListInput) -> Result<Vec<WalletInfo>, ServiceError>;
    async fn delete_wallet(&self, id: &str) -> Result<(), ServiceError>;
}

#[derive(Clone)]
pub struct Web3WalletService;

#[async_trait]
impl TWalletService for Web3WalletService {
    async fn verify_wallet(&self, input: WalletVerifyInput) -> Result<WalletInfo, ServiceError> {
        let db = get_db().await?;
        let now = Local::now().naive_local();
        let wallet_type = input.wallet_type.unwrap_or_else(|| "metamask".to_string());
        let chain_id = input.chain_id.unwrap_or(1);

        // Check if wallet exists
        let existing = server_model::web3::entities::prelude::Web3Wallet::find()
            .filter(server_model::web3::entities::web3_wallet::Column::WalletAddress.eq(input.wallet_address.to_lowercase()))
            .one(db.as_ref())
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        if let Some(wallet) = existing {
            let mut am = wallet.into_active_model();
            am.wallet_type = Set(wallet_type);
            am.chain_id = Set(chain_id);
            am.signature = Set(Some(input.signature));
            am.message = Set(Some(input.message));
            am.updated_at = Set(Some(now));

            let updated = am.update(db.as_ref()).await
                .map_err(|e| ServiceError::new(&e.to_string()))?;

            return Ok(WalletInfo {
                id: updated.id,
                wallet_address: updated.wallet_address,
                wallet_type: updated.wallet_type,
                chain_id: updated.chain_id,
            });
        }

        // Create new wallet
        let wallet = server_model::web3::entities::web3_wallet::ActiveModel {
            id: Set(Ulid::new().to_string()),
            user_id: Set(None),
            wallet_address: Set(input.wallet_address.to_lowercase()),
            wallet_type: Set(wallet_type),
            chain_id: Set(chain_id),
            signature: Set(Some(input.signature)),
            message: Set(Some(input.message)),
            created_at: Set(now),
            updated_at: Set(None),
        };

        let created = wallet.insert(db.as_ref()).await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(WalletInfo {
            id: created.id,
            wallet_address: created.wallet_address,
            wallet_type: created.wallet_type,
            chain_id: created.chain_id,
        })
    }

    async fn list_wallets(&self, input: WalletListInput) -> Result<Vec<WalletInfo>, ServiceError> {
        let db = get_db().await?;
        let mut query = server_model::web3::entities::prelude::Web3Wallet::find();

        if let Some(user_id) = input.user_id {
            query = query.filter(server_model::web3::entities::web3_wallet::Column::UserId.eq(user_id));
        }

        let wallets = query.all(db.as_ref()).await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(wallets.into_iter().map(|w| WalletInfo {
            id: w.id,
            wallet_address: w.wallet_address,
            wallet_type: w.wallet_type,
            chain_id: w.chain_id,
        }).collect())
    }

    async fn delete_wallet(&self, id: &str) -> Result<(), ServiceError> {
        let db = get_db().await?;
        let result = server_model::web3::entities::prelude::Web3Wallet::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        if result.rows_affected == 0 {
            return Err(ServiceError::new("Wallet not found"));
        }
        Ok(())
    }
}

// ============ Contract Service ============
#[async_trait]
pub trait TContractService {
    async fn create_contract(&self, input: ContractCreateInput) -> Result<ContractInfo, ServiceError>;
    async fn list_contracts(&self) -> Result<Vec<ContractInfo>, ServiceError>;
    async fn get_contract(&self, id: &str) -> Result<ContractInfo, ServiceError>;
    async fn update_contract(&self, input: ContractUpdateInput) -> Result<ContractInfo, ServiceError>;
    async fn delete_contract(&self, id: &str) -> Result<(), ServiceError>;
    async fn call_contract(&self, input: ContractCallInput) -> Result<ContractCallOutput, ServiceError>;
}

#[derive(Clone)]
pub struct Web3ContractService;

#[async_trait]
impl TContractService for Web3ContractService {
    async fn create_contract(&self, input: ContractCreateInput) -> Result<ContractInfo, ServiceError> {
        let db = get_db().await?;
        let now = Local::now().naive_local();

        let contract = server_model::web3::entities::web3_contract::ActiveModel {
            id: Set(Ulid::new().to_string()),
            name: Set(input.name),
            contract_address: Set(input.contract_address.to_lowercase()),
            chain_id: Set(input.chain_id),
            abi: Set(input.abi),
            description: Set(input.description),
            created_by: Set(None),
            created_at: Set(now),
            updated_at: Set(None),
        };

        let created = contract.insert(db.as_ref()).await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(ContractInfo {
            id: created.id,
            name: created.name,
            contract_address: created.contract_address,
            chain_id: created.chain_id,
            abi: created.abi,
            description: created.description,
            created_at: created.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    async fn list_contracts(&self) -> Result<Vec<ContractInfo>, ServiceError> {
        let db = get_db().await?;
        let contracts = server_model::web3::entities::prelude::Web3Contract::find()
            .all(db.as_ref())
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(contracts.into_iter().map(|c| ContractInfo {
            id: c.id,
            name: c.name,
            contract_address: c.contract_address,
            chain_id: c.chain_id,
            abi: c.abi,
            description: c.description,
            created_at: c.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }).collect())
    }

    async fn get_contract(&self, id: &str) -> Result<ContractInfo, ServiceError> {
        let db = get_db().await?;
        let contract = server_model::web3::entities::prelude::Web3Contract::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?
            .ok_or_else(|| ServiceError::new("Contract not found"))?;

        Ok(ContractInfo {
            id: contract.id,
            name: contract.name,
            contract_address: contract.contract_address,
            chain_id: contract.chain_id,
            abi: contract.abi,
            description: contract.description,
            created_at: contract.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    async fn update_contract(&self, input: ContractUpdateInput) -> Result<ContractInfo, ServiceError> {
        let db = get_db().await?;
        let now = Local::now().naive_local();

        let contract = server_model::web3::entities::prelude::Web3Contract::find_by_id(&input.id)
            .one(db.as_ref())
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?
            .ok_or_else(|| ServiceError::new("Contract not found"))?;

        let mut am = contract.into_active_model();
        if let Some(name) = input.name { am.name = Set(name); }
        if let Some(addr) = input.contract_address { am.contract_address = Set(addr.to_lowercase()); }
        if let Some(cid) = input.chain_id { am.chain_id = Set(cid); }
        if let Some(abi) = input.abi { am.abi = Set(Some(abi)); }
        if let Some(desc) = input.description { am.description = Set(Some(desc)); }
        am.updated_at = Set(Some(now));

        let updated = am.update(db.as_ref()).await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(ContractInfo {
            id: updated.id,
            name: updated.name,
            contract_address: updated.contract_address,
            chain_id: updated.chain_id,
            abi: updated.abi,
            description: updated.description,
            created_at: updated.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    async fn delete_contract(&self, id: &str) -> Result<(), ServiceError> {
        let db = get_db().await?;
        let result = server_model::web3::entities::prelude::Web3Contract::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        if result.rows_affected == 0 {
            return Err(ServiceError::new("Contract not found"));
        }
        Ok(())
    }

    async fn call_contract(&self, input: ContractCallInput) -> Result<ContractCallOutput, ServiceError> {
        let db = get_db().await?;
        
        // Verify contract exists
        let _ = server_model::web3::entities::prelude::Web3Contract::find_by_id(&input.contract_id)
            .one(db.as_ref())
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?
            .ok_or_else(|| ServiceError::new("Contract not found"))?;

        let now = Local::now().naive_local();
        
        // Record transaction
        let tx = server_model::web3::entities::web3_transaction::ActiveModel {
            id: Set(Ulid::new().to_string()),
            user_id: Set(None),
            contract_id: Set(Some(input.contract_id.clone())),
            method_name: Set(input.method_name.clone()),
            params: Set(input.params.clone()),
            tx_hash: Set(None),
            status: Set("pending".to_string()),
            from_address: Set(input.from_address.clone()),
            error_message: Set(None),
            created_at: Set(now),
            updated_at: Set(None),
        };

        tx.insert(db.as_ref()).await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(ContractCallOutput {
            success: true,
            tx_hash: Some(format!("0x{}", Ulid::new())),
            result: Some("Transaction recorded. Use frontend to sign.".to_string()),
            error: None,
        })
    }
}

// ============ Transaction Service ============
#[async_trait]
pub trait TTransactionService {
    async fn list_transactions(&self, user_id: Option<String>) -> Result<Vec<TransactionInfo>, ServiceError>;
}

#[derive(Clone)]
pub struct Web3TransactionService;

#[async_trait]
impl TTransactionService for Web3TransactionService {
    async fn list_transactions(&self, user_id: Option<String>) -> Result<Vec<TransactionInfo>, ServiceError> {
        let db = get_db().await?;
        let mut select = server_model::web3::entities::prelude::Web3Transaction::find();

        if let Some(uid) = user_id {
            select = select.filter(server_model::web3::entities::web3_transaction::Column::UserId.eq(uid));
        }

        let txs = select
            .order_by(server_model::web3::entities::web3_transaction::Column::CreatedAt, sea_orm::Order::Desc)
            .all(db.as_ref())
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(txs.into_iter().map(|t| TransactionInfo {
            id: t.id,
            contract_id: t.contract_id,
            method_name: t.method_name,
            params: t.params,
            tx_hash: t.tx_hash,
            status: t.status,
            from_address: t.from_address,
            error_message: t.error_message,
            created_at: t.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }).collect())
    }
}
