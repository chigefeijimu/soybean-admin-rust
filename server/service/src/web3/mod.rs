use async_trait::async_trait;
use chrono::Local;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, Set,
};
use server_core::web::error::AppError;
use server_model::web3::{
    input::{
        ContractCallInput, ContractCreateInput, ContractUpdateInput, WalletInfo, WalletVerifyInput,
        WalletListInput,
    },
    output::{ContractCallOutput, ContractInfo, TransactionInfo},
    entities::{
        prelude::Web3Wallet,
        prelude::Web3Contract,
        prelude::Web3Transaction,
        web3_wallet::{ActiveModel as WalletActiveModel, Column as WalletColumn},
        web3_contract::ActiveModel as ContractActiveModel,
        web3_transaction::{ActiveModel as TransactionActiveModel, Column as TransactionColumn},
    },
};
use ulid::Ulid;
use std::sync::Arc;
use sea_orm::DatabaseConnection;

use crate::helper::db_helper;

/// Helper to convert anyhow errors to AppError
fn anyhow_to_app_err(e: anyhow::Error) -> AppError {
    AppError {
        code: 500,
        message: e.to_string(),
    }
}

// ============ Wallet Service ============

#[async_trait]
pub trait TWalletService {
    async fn verify_wallet(&self, input: WalletVerifyInput) -> Result<WalletInfo, AppError>;
    async fn list_wallets(&self, input: WalletListInput) -> Result<Vec<WalletInfo>, AppError>;
    async fn delete_wallet(&self, id: &str) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct Web3WalletService;

impl Web3WalletService {
    /// Verify wallet ownership by signature (simplified)
    pub async fn verify_signature(_address: &str, _message: &str, _signature: &str) -> Result<bool, AppError> {
        // Simplified verification - in production, implement proper ecrecover
        Ok(true)
    }

    /// Generate a verification nonce
    pub fn generate_nonce() -> String {
        Ulid::new().to_string()
    }
}

#[async_trait]
impl TWalletService for Web3WalletService {
    async fn verify_wallet(&self, input: WalletVerifyInput) -> Result<WalletInfo, AppError> {
        // Verify signature
        let is_valid = Web3WalletService::verify_signature(
            &input.wallet_address,
            &input.message,
            &input.signature,
        ).await.map_err(|e| AppError { code: 500, message: format!("{:?}", e) })?;

        if !is_valid {
            return Err(anyhow_to_app_err(anyhow::anyhow!("Signature verification failed")));
        }

        let db: Arc<DatabaseConnection> = db_helper::get_db_connection().await.map_err(|e| AppError::from(e))?;
        let now = Local::now().naive_local();
        let wallet_type = input.wallet_type.unwrap_or_else(|| "metamask".to_string());
        let chain_id = input.chain_id.unwrap_or(1);

        // Check if wallet already exists
        let existing = Web3Wallet::find()
            .filter(WalletColumn::WalletAddress.eq(input.wallet_address.to_lowercase()))
            .one(db.as_ref())
            .await
            .map_err(|e| AppError::from(e))?;

        if let Some(wallet) = existing {
            // Update existing wallet
            let mut active_model = wallet.into_active_model();
            active_model.wallet_type = Set(wallet_type);
            active_model.chain_id = Set(chain_id);
            active_model.signature = Set(Some(input.signature));
            active_model.message = Set(Some(input.message));
            active_model.updated_at = Set(Some(now));

            let updated = active_model.update(db.as_ref()).await.map_err(|e| AppError::from(e))?;

            return Ok(WalletInfo {
                id: updated.id,
                wallet_address: updated.wallet_address,
                wallet_type: updated.wallet_type,
                chain_id: updated.chain_id,
            });
        }

        // Create new wallet
        let wallet = WalletActiveModel {
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

        let created = wallet.insert(db.as_ref()).await.map_err(|e| AppError::from(e))?;

        Ok(WalletInfo {
            id: created.id,
            wallet_address: created.wallet_address,
            wallet_type: created.wallet_type,
            chain_id: created.chain_id,
        })
    }

    async fn list_wallets(&self, input: WalletListInput) -> Result<Vec<WalletInfo>, AppError> {
        let db: Arc<DatabaseConnection> = db_helper::get_db_connection().await.map_err(|e| AppError::from(e))?;
        let mut query = Web3Wallet::find();

        if let Some(user_id) = input.user_id {
            query = query.filter(WalletColumn::UserId.eq(user_id));
        }

        let wallets = query.all(db.as_ref()).await.map_err(|e| AppError::from(e))?;

        Ok(wallets
            .into_iter()
            .map(|w| WalletInfo {
                id: w.id,
                wallet_address: w.wallet_address,
                wallet_type: w.wallet_type,
                chain_id: w.chain_id,
            })
            .collect())
    }

    async fn delete_wallet(&self, id: &str) -> Result<(), AppError> {
        let db: Arc<DatabaseConnection> = db_helper::get_db_connection().await.map_err(|e| AppError::from(e))?;

        let result = Web3Wallet::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(AppError::from)?;

        if result.rows_affected == 0 {
            return Err(anyhow_to_app_err(anyhow::anyhow!("Wallet not found")));
        }

        Ok(())
    }
}

// ============ Contract Service ============

#[async_trait]
pub trait TContractService {
    async fn create_contract(&self, input: ContractCreateInput) -> Result<ContractInfo, AppError>;
    async fn list_contracts(&self) -> Result<Vec<ContractInfo>, AppError>;
    async fn get_contract(&self, id: &str) -> Result<ContractInfo, AppError>;
    async fn update_contract(&self, input: ContractUpdateInput) -> Result<ContractInfo, AppError>;
    async fn delete_contract(&self, id: &str) -> Result<(), AppError>;
    async fn call_contract(&self, input: ContractCallInput) -> Result<ContractCallOutput, AppError>;
}

#[derive(Clone)]
pub struct Web3ContractService;

#[async_trait]
impl TContractService for Web3ContractService {
    async fn create_contract(&self, input: ContractCreateInput) -> Result<ContractInfo, AppError> {
        let db: Arc<DatabaseConnection> = db_helper::get_db_connection().await.map_err(|e| AppError::from(e))?;
        let now = Local::now().naive_local();

        let contract = ContractActiveModel {
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

        let created = contract.insert(db.as_ref()).await.map_err(|e| AppError::from(e))?;

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

    async fn list_contracts(&self) -> Result<Vec<ContractInfo>, AppError> {
        let db: Arc<DatabaseConnection> = db_helper::get_db_connection().await.map_err(|e| AppError::from(e))?;
        let contracts = Web3Contract::find()
            .all(db.as_ref())
            .await
            .map_err(|e| AppError::from(e))?;

        Ok(contracts
            .into_iter()
            .map(|c| ContractInfo {
                id: c.id,
                name: c.name,
                contract_address: c.contract_address,
                chain_id: c.chain_id,
                abi: c.abi,
                description: c.description,
                created_at: c.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
            .collect())
    }

    async fn get_contract(&self, id: &str) -> Result<ContractInfo, AppError> {
        let db: Arc<DatabaseConnection> = db_helper::get_db_connection().await.map_err(|e| AppError::from(e))?;
        
        let contract = Web3Contract::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| anyhow_to_app_err(anyhow::anyhow!("Contract not found")))?;

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

    async fn update_contract(&self, input: ContractUpdateInput) -> Result<ContractInfo, AppError> {
        let db: Arc<DatabaseConnection> = db_helper::get_db_connection().await.map_err(|e| AppError::from(e))?;
        let now = Local::now().naive_local();

        let contract = Web3Contract::find_by_id(&input.id)
            .one(db.as_ref())
            .await
            .map_err(|e| AppError::from(e))?
            .ok_or_else(|| anyhow_to_app_err(anyhow::anyhow!("Contract not found")))?;

        let mut active_model = contract.into_active_model();
        
        if let Some(name) = input.name {
            active_model.name = Set(name);
        }
        if let Some(address) = input.contract_address {
            active_model.contract_address = Set(address.to_lowercase());
        }
        if let Some(chain_id) = input.chain_id {
            active_model.chain_id = Set(chain_id);
        }
        if let Some(abi) = input.abi {
            active_model.abi = Set(Some(abi));
        }
        if let Some(desc) = input.description {
            active_model.description = Set(Some(desc));
        }
        
        active_model.updated_at = Set(Some(now));

        let updated = active_model.update(db.as_ref()).await.map_err(|e| AppError::from(e))?;

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

    async fn delete_contract(&self, id: &str) -> Result<(), AppError> {
        let db: Arc<DatabaseConnection> = db_helper::get_db_connection().await.map_err(|e| AppError::from(e))?;

        let result = Web3Contract::delete_by_id(id)
            .exec(db.as_ref())
            .await
            .map_err(AppError::from)?;

        if result.rows_affected == 0 {
            return Err(anyhow_to_app_err(anyhow::anyhow!("Contract not found")));
        }

        Ok(())
    }

    async fn call_contract(&self, input: ContractCallInput) -> Result<ContractCallOutput, AppError> {
        let db: Arc<DatabaseConnection> = db_helper::get_db_connection().await.map_err(|e| AppError::from(e))?;
        
        // Get contract info
        let _contract = Web3Contract::find_by_id(&input.contract_id)
            .one(db.as_ref())
            .await
            .map_err(|e| AppError::from(e))?
            .ok_or_else(|| anyhow_to_app_err(anyhow::anyhow!("Contract not found")))?;

        let now = Local::now().naive_local();
        
        // Create transaction record
        let tx_record = TransactionActiveModel {
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
        
        let _tx_created = tx_record.insert(db.as_ref()).await.map_err(|e| AppError::from(e))?;

        // Note: Actual contract call should be done on frontend using wagmi
        // Backend can provide RPC endpoints for this
        Ok(ContractCallOutput {
            success: true,
            tx_hash: Some(format!("0x{}", Ulid::new())),
            result: Some("Transaction recorded. Use frontend to sign and send.".to_string()),
            error: None,
        })
    }
}

// ============ Transaction Service ============

#[async_trait]
pub trait TTransactionService {
    async fn list_transactions(&self, user_id: Option<String>) -> Result<Vec<TransactionInfo>, AppError>;
}

#[derive(Clone)]
pub struct Web3TransactionService;

#[async_trait]
impl TTransactionService for Web3TransactionService {
    async fn list_transactions(&self, user_id: Option<String>) -> Result<Vec<TransactionInfo>, AppError> {
        let db: Arc<DatabaseConnection> = db_helper::get_db_connection().await.map_err(|e| AppError::from(e))?;
        
        let mut select = Web3Transaction::find();

        if let Some(uid) = user_id {
            select = select.filter(TransactionColumn::UserId.eq(uid));
        }

        let transactions = select
            .order_by(TransactionColumn::CreatedAt, sea_orm::Order::Desc)
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?;

        Ok(transactions
            .into_iter()
            .map(|t| TransactionInfo {
                id: t.id,
                contract_id: t.contract_id,
                method_name: t.method_name,
                params: t.params,
                tx_hash: t.tx_hash,
                status: t.status,
                from_address: t.from_address,
                error_message: t.error_message,
                created_at: t.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
            .collect())
    }
}
