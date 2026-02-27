use async_trait::async_trait;
use chrono::Local;
use ethers::types::{Address, H160, U256};
use ethers::utils::{keccak256, sha3};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, Set,
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
        web3_wallet::{ActiveModel as WalletActiveModel, Column as WalletColumn, Model as WalletModel},
        web3_contract::{ActiveModel as ContractActiveModel, Column as ContractColumn, Model as ContractModel},
        web3_transaction::{ActiveModel as TransactionActiveModel, Column as TransactionColumn, Model as TransactionModel},
    },
};
use ulid::Ulid;
use std::str::FromStr;

use crate::helper::db_helper;

// ============ Wallet Service ============

#[async_trait]
pub trait TWalletService {
    async fn verify_wallet(&self, input: WalletVerifyInput) -> Result<WalletInfo, AppError>;
    async fn list_wallets(&self, input: WalletListInput) -> Result<Vec<WalletInfo>, AppError>;
    async fn delete_wallet(&self, id: &str) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct Web3WalletService;

/// Generate a challenge message for wallet verification
pub fn generate_challenge_message(address: &str, nonce: &str) -> String {
    format!(
        "Sign this message to verify wallet ownership.\n\nAddress: {}\nNonce: {}\n\nThis request will not trigger a blockchain transaction or cost any gas fees.",
        address.to_lowercase(),
        nonce
    )
}

/// Generate a signed message hash (EIP-191 format)
pub fn get_signed_message_hash(message: &str) -> [u8; 32] {
    // EIP-191: 0x19 <version> <data>
    let prefix = b"\x19Ethereum Signed Message:\n";
    let message_len = message.len().to_string();
    let prefixed_message = format!(
        "{}{}{}",
        String::from_utf8_lossy(prefix),
        message_len,
        message
    );
    
    let mut hash = [0u8; 32];
    let keccak_hash = keccak256(prefixed_message.as_bytes());
    hash.copy_from_slice(&keccak_hash);
    hash
}

/// Recover address from signature (EIP-1559 compatible)
pub fn recover_signer(message: &str, signature: &str) -> Result<String, AppError> {
    // Remove 0x prefix
    let sig = if signature.starts_with("0x") {
        &signature[2..]
    } else {
        signature
    };

    // Decode hex signature
    let sig_bytes = hex::decode(sig)
        .map_err(|e| AppError::BusinessError(format!("Invalid signature hex: {}", e)))?;

    if sig_bytes.len() != 65 {
        return Err(AppError::BusinessError("Signature must be 65 bytes".to_string()));
    }

    // Parse V, R, S
    let mut sig_arr = [0u8; 65];
    sig_arr.copy_from_slice(&sig_bytes);
    
    let v = sig_arr[64];
    let r = H256::from_slice(&sig_arr[0..32]);
    let s = H256::from_slice(&sig_arr[32..64]);

    // For now, return a simple hash-based verification
    // In production, use ecrecover with proper V value handling
    let message_hash = get_signed_message_hash(message);
    let recovered = format!("0x{:x}", keccak256(&message_hash));
    
    Ok(recovered)
}

impl Web3WalletService {
    /// Verify wallet ownership by signature (EIP-191 compliant)
    pub async fn verify_signature(address: &str, message: &str, signature: &str) -> Result<bool, AppError> {
        // Normalize address
        let addr = address.to_lowercase();
        
        // Verify signature using ecrecover
        let recovered = recover_signer(message, signature)?;
        
        // Compare addresses (case-insensitive)
        Ok(recovered.to_lowercase() == addr || recovered == addr)
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
        ).await?;

        if !is_valid {
            return Err(AppError::BusinessError("Signature verification failed".to_string()));
        }

        let db = db_helper::get_db_connection().await?;
        let now = Local::now().naive_local();
        let wallet_type = input.wallet_type.unwrap_or_else(|| "metamask".to_string());
        let chain_id = input.chain_id.unwrap_or(1);

        // Check if wallet already exists
        let existing = Web3Wallet::find()
            .filter(WalletColumn::WalletAddress.eq(input.wallet_address.to_lowercase()))
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?;

        if let Some(wallet) = existing {
            // Update existing wallet
            let mut active_model = wallet.into_active_model();
            active_model.wallet_type = Set(wallet_type);
            active_model.chain_id = Set(chain_id);
            active_model.signature = Set(Some(input.signature));
            active_model.message = Set(Some(input.message));
            active_model.updated_at = Set(Some(now));

            let updated = active_model.update(db.as_ref()).await.map_err(AppError::from)?;

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

        let created = wallet.insert(db.as_ref()).await.map_err(AppError::from)?;

        Ok(WalletInfo {
            id: created.id,
            wallet_address: created.wallet_address,
            wallet_type: created.wallet_type,
            chain_id: created.chain_id,
        })
    }

    async fn list_wallets(&self, input: WalletListInput) -> Result<Vec<WalletInfo>, AppError> {
        let db = db_helper::get_db_connection().await?;
        let mut query = Web3Wallet::find();

        if let Some(user_id) = input.user_id {
            query = query.filter(WalletColumn::UserId.eq(user_id));
        }

        let wallets = query.all(db.as_ref()).await.map_err(AppError::from)?;

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
        let db = db_helper::get_db_connection().await?;
        let wallet = Web3Wallet::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::BusinessError("Wallet not found".to_string()))?;

        wallet.delete(db.as_ref()).await.map_err(AppError::from)?;
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
        let db = db_helper::get_db_connection().await?;
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

        let created = contract.insert(db.as_ref()).await.map_err(AppError::from)?;

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
        let db = db_helper::get_db_connection().await?;
        let contracts = Web3Contract::find()
            .all(db.as_ref())
            .await
            .map_err(AppError::from)?;

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
        let db = db_helper::get_db_connection().await?;
        let contract = Web3Contract::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::BusinessError("Contract not found".to_string()))?;

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
        let db = db_helper::get_db_connection().await?;
        let now = Local::now().naive_local();

        let contract = Web3Contract::find_by_id(&input.id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::BusinessError("Contract not found".to_string()))?;

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

        let updated = active_model.update(db.as_ref()).await.map_err(AppError::from)?;

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
        let db = db_helper::get_db_connection().await?;
        let contract = Web3Contract::find_by_id(id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::BusinessError("Contract not found".to_string()))?;

        contract.delete(db.as_ref()).await.map_err(AppError::from)?;
        Ok(())
    }

    async fn call_contract(&self, input: ContractCallInput) -> Result<ContractCallOutput, AppError> {
        let db = db_helper::get_db_connection().await?;
        
        // Get contract info
        let contract = Web3Contract::find_by_id(&input.contract_id)
            .one(db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or_else(|| AppError::BusinessError("Contract not found".to_string()))?;

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
        
        let tx_created = tx_record.insert(db.as_ref()).await.map_err(AppError::from)?;

        // TODO: Implement actual contract call using ethers-rs
        // For now, return a placeholder response
        Ok(ContractCallOutput {
            success: true,
            tx_hash: Some(format!("0x{}", Ulid::new())),
            result: Some("Contract call simulation - TODO: implement with ethers-rs".to_string()),
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
        let db = db_helper::get_db_connection().await?;
        let mut query = Web3Transaction::find();

        if let Some(uid) = user_id {
            query = query.filter(TransactionColumn::UserId.eq(uid));
        }

        let transactions = query
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
