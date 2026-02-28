// =========================================
// Web3 Service Module
// =========================================

pub mod alloy_provider;
pub mod alloy_provider_v2;
pub mod erc20;
pub mod contract_call_impl;
pub mod market_data;
pub mod receipt_parser;
pub mod transaction_decoder;

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
use crate::web3::alloy_provider::{ProviderPool, Web3Provider as Provider};

// Global provider pool
lazy_static::lazy_static! {
    pub static ref PROVIDER_POOL: ProviderPool = ProviderPool::new();
}

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
pub struct WalletBalanceInput {
    pub address: String,
    pub chain_id: Option<i32>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceInput {
    pub owner_address: String,
    pub token_addresses: Vec<String>,
    pub chain_id: Option<i32>,
}

/// Direct contract call input (by contract address, not ID)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectContractCallInput {
    pub contract_address: String,
    pub chain_id: Option<i32>,
    pub method_name: String,
    pub params: Option<String>,
    pub from_address: Option<String>,
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
pub struct WalletBalance {
    pub address: String,
    pub balance: String,
    pub chain_id: u64,
    pub chain_name: String,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub token_address: String,
    pub balance: String,
    pub decimals: u8,
    pub formatted_balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceList {
    pub owner_address: String,
    pub chain_id: u64,
    pub balances: Vec<TokenBalance>,
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
    async fn get_balance(&self, address: &str, chain_id: i32) -> Result<WalletBalance, ServiceError>;
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
        
        // Verify signature using EIP-191
        // If message and signature are provided, verify them
        if !input.signature.is_empty() && !input.message.is_empty() {
            let is_valid = alloy_provider::signature::verify_eip191(
                &input.message,
                &input.signature,
                &input.wallet_address.to_lowercase()
            ).unwrap_or(false);
            
            if !is_valid {
                tracing::warn!("Wallet signature verification failed for {}", input.wallet_address);
                // Don't block - allow save but log warning
            }
        }

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

    async fn get_balance(&self, address: &str, chain_id: i32) -> Result<WalletBalance, ServiceError> {
        let chain_id_u64 = chain_id as u64;
        
        // Get provider from pool
        let pool = PROVIDER_POOL.clone();
        let provider: Provider = pool.get_provider(chain_id_u64)
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        // Get balance from blockchain
        let balance: String = provider.get_balance(address)
            .await
            .map_err(|e| ServiceError::new(&format!("Failed to get balance: {}", e)))?;

        Ok(WalletBalance {
            address: address.to_string(),
            balance: balance.to_string(),
            chain_id: chain_id_u64,
            chain_name: provider.chain_name().to_string(),
        })
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
    async fn call_contract_direct(&self, input: DirectContractCallInput) -> Result<ContractCallOutput, ServiceError>;
    async fn get_token_balances(&self, input: TokenBalanceInput) -> Result<TokenBalanceList, ServiceError>;
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
        
        // Get contract from DB
        let contract = server_model::web3::entities::prelude::Web3Contract::find_by_id(&input.contract_id)
            .one(db.as_ref())
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?
            .ok_or_else(|| ServiceError::new("Contract not found"))?;

        let chain_id = contract.chain_id as u64;
        
        // Get provider from pool to get RPC URL
        let pool = PROVIDER_POOL.clone();
        let provider: Provider = pool.get_provider(chain_id)
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        // Validate contract address format
        if !contract.contract_address.starts_with("0x") || contract.contract_address.len() != 42 {
            return Err(ServiceError::new("Invalid contract address format"));
        }

        let now = Local::now().naive_local();
        
        // Clone params for use in async block
        let params_clone = input.params.clone();
        
        // Get RPC URL from provider
        let rpc_url = provider.rpc_url.clone();
        
        // Execute actual contract call using contract_call_impl
        let result: Result<String, Box<dyn std::error::Error + Send + Sync>> = async {
            match input.method_name.as_str() {
                // Read-only calls (view/pure functions)
                "name" | "symbol" | "decimals" | "totalSupply" | "balanceOf" | "allowance" => {
                    let call_result = contract_call_impl::execute_contract_read(
                        &rpc_url,
                        &contract.contract_address,
                        &input.method_name,
                        params_clone.map(|p| vec![p]).unwrap_or_default(),
                    ).await?;
                    Ok(call_result)
                },
                // Write operations - require wallet signature (return tx data)
                _ => {
                    Err(format!("Write operation '{}' requires wallet signature via frontend", input.method_name).into())
                }
            }
        }.await;

        let (success, tx_hash, error_msg, result_str) = match result {
            Ok(res) => (true, None, None, Some(res)),
            Err(e) => (false, None, Some(e.to_string()), None),
        };

        // Record transaction in DB
        let tx = server_model::web3::entities::web3_transaction::ActiveModel {
            id: Set(Ulid::new().to_string()),
            user_id: Set(None),
            contract_id: Set(Some(input.contract_id.clone())),
            method_name: Set(input.method_name.clone()),
            params: Set(input.params.clone()),
            tx_hash: Set(tx_hash.clone()),
            status: Set(if success { "completed" } else { "failed" }.to_string()),
            from_address: Set(input.from_address.clone()),
            error_message: Set(error_msg),
            created_at: Set(now),
            updated_at: Set(None),
        };

        tx.insert(db.as_ref()).await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(ContractCallOutput {
            success,
            tx_hash,
            result: result_str,
            error: None,
        })
    }

    async fn call_contract_direct(&self, input: DirectContractCallInput) -> Result<ContractCallOutput, ServiceError> {
        let chain_id = input.chain_id.unwrap_or(1) as u64;
        let now = Local::now().naive_local();
        
        // Get provider from pool
        let pool = PROVIDER_POOL.clone();
        let provider: Provider = pool.get_provider(chain_id)
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        let rpc_url = provider.rpc_url.clone();
        let contract_address = input.contract_address.to_lowercase();
        let params_clone = input.params.clone();
        
        // Execute contract call
        let result: Result<String, Box<dyn std::error::Error + Send + Sync>> = async {
            match input.method_name.as_str() {
                // Read-only calls (view/pure functions)
                "name" | "symbol" | "decimals" | "totalSupply" | "balanceOf" | "allowance" => {
                    let call_result = contract_call_impl::execute_contract_read(
                        &rpc_url,
                        &contract_address,
                        &input.method_name,
                        params_clone.map(|p| vec![p]).unwrap_or_default(),
                    ).await?;
                    Ok(call_result)
                },
                // Write operations - require wallet signature
                _ => {
                    Err(format!("Write operation '{}' requires wallet signature via frontend", input.method_name).into())
                }
            }
        }.await;

        let (success, tx_hash, error_msg, result_str) = match result {
            Ok(res) => (true, None, None, Some(res)),
            Err(e) => (false, None, Some(e.to_string()), None),
        };

        // Record transaction in DB (without contract_id since it's direct call)
        let tx = server_model::web3::entities::web3_transaction::ActiveModel {
            id: Set(Ulid::new().to_string()),
            user_id: Set(None),
            contract_id: Set(None),
            method_name: Set(input.method_name.clone()),
            params: Set(input.params.clone()),
            tx_hash: Set(tx_hash.clone()),
            status: Set(if success { "completed" } else { "failed" }.to_string()),
            from_address: Set(input.from_address.clone()),
            error_message: Set(error_msg),
            created_at: Set(now),
            updated_at: Set(None),
        };

        let db = get_db().await?;
        tx.insert(db.as_ref()).await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        Ok(ContractCallOutput {
            success,
            tx_hash,
            result: result_str,
            error: None,
        })
    }

    async fn get_token_balances(&self, input: TokenBalanceInput) -> Result<TokenBalanceList, ServiceError> {
        let chain_id = input.chain_id.unwrap_or(1) as u64;
        
        // Get provider from pool
        let pool = PROVIDER_POOL.clone();
        let provider: Provider = pool.get_provider(chain_id)
            .await
            .map_err(|e| ServiceError::new(&e.to_string()))?;

        let rpc_url = provider.rpc_url.clone();
        let owner = input.owner_address.to_lowercase();
        
        let mut balances = Vec::new();
        
        for token_addr in input.token_addresses {
            let token_addr_lower = token_addr.to_lowercase();
            
            // Get decimals first
            let decimals_result = contract_call_impl::execute_contract_read(
                &rpc_url,
                &token_addr_lower,
                "decimals",
                vec![]
            ).await;
            
            let decimals = decimals_result
                .ok()
                .and_then(|s| s.parse::<u8>().ok())
                .unwrap_or(18);
            
            // Get balance
            let balance_result = contract_call_impl::get_erc20_balance(
                &rpc_url,
                &token_addr_lower,
                &owner
            ).await;
            
            let (balance, formatted) = match balance_result {
                Ok(bal) => {
                    let formatted = contract_call_impl::format_token_balance(bal, decimals);
                    (bal.to_string(), formatted)
                },
                Err(_) => ("0".to_string(), "0".to_string()),
            };
            
            balances.push(TokenBalance {
                token_address: token_addr_lower,
                balance,
                decimals,
                formatted_balance: formatted,
            });
        }

        Ok(TokenBalanceList {
            owner_address: owner,
            chain_id,
            balances,
        })
    }
}

// ============ Transaction Service ============
pub use receipt_parser::{ParsedReceipt, TransactionReceipt};

#[async_trait]
pub trait TTransactionService {
    async fn list_transactions(&self, user_id: Option<String>) -> Result<Vec<TransactionInfo>, ServiceError>;
    async fn parse_receipt(&self, receipt: TransactionReceipt) -> Result<ParsedReceipt, ServiceError>;
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

    async fn parse_receipt(&self, receipt: TransactionReceipt) -> Result<ParsedReceipt, ServiceError> {
        Ok(receipt_parser::parse_receipt(receipt))
    }
}

// ============ Market Data Service ============
use market_data::{
    TokenPrice, MarketOverview, GasPrice, DeFiProtocol, PriceHistory,
    MarketDataService as MarketDataServiceImpl,
};

lazy_static::lazy_static! {
    static ref MARKET_DATA_SERVICE: MarketDataServiceImpl = MarketDataServiceImpl::new();
}

#[async_trait]
pub trait TMarketDataService {
    async fn get_token_price(&self, symbol: &str) -> Result<TokenPrice, ServiceError>;
    async fn get_all_prices(&self) -> Result<Vec<TokenPrice>, ServiceError>;
    async fn get_market_overview(&self) -> Result<MarketOverview, ServiceError>;
    async fn get_gas_price(&self, chain_id: u64) -> Result<GasPrice, ServiceError>;
    async fn get_defi_protocols(&self) -> Result<Vec<DeFiProtocol>, ServiceError>;
    async fn get_price_history(&self, symbol: &str, days: u32) -> Result<PriceHistory, ServiceError>;
    async fn search_tokens(&self, query: &str) -> Result<Vec<TokenPrice>, ServiceError>;
    async fn get_trending(&self) -> Result<Vec<TokenPrice>, ServiceError>;
    async fn get_top_gainers(&self) -> Result<Vec<TokenPrice>, ServiceError>;
    async fn get_top_losers(&self) -> Result<Vec<TokenPrice>, ServiceError>;
}

#[derive(Clone)]
pub struct Web3MarketDataService;

#[async_trait]
impl TMarketDataService for Web3MarketDataService {
    async fn get_token_price(&self, symbol: &str) -> Result<TokenPrice, ServiceError> {
        MARKET_DATA_SERVICE
            .get_price(symbol)
            .cloned()
            .ok_or_else(|| ServiceError::new("Token not found"))
    }

    async fn get_all_prices(&self) -> Result<Vec<TokenPrice>, ServiceError> {
        Ok(MARKET_DATA_SERVICE.get_all_prices())
    }

    async fn get_market_overview(&self) -> Result<MarketOverview, ServiceError> {
        Ok(MARKET_DATA_SERVICE.get_market_overview())
    }

    async fn get_gas_price(&self, chain_id: u64) -> Result<GasPrice, ServiceError> {
        Ok(MARKET_DATA_SERVICE.get_gas_price(chain_id))
    }

    async fn get_defi_protocols(&self) -> Result<Vec<DeFiProtocol>, ServiceError> {
        Ok(MARKET_DATA_SERVICE.get_defi_protocols())
    }

    async fn get_price_history(&self, symbol: &str, days: u32) -> Result<PriceHistory, ServiceError> {
        Ok(MARKET_DATA_SERVICE.get_price_history(symbol, days))
    }

    async fn search_tokens(&self, query: &str) -> Result<Vec<TokenPrice>, ServiceError> {
        Ok(MARKET_DATA_SERVICE.search_tokens(query))
    }

    async fn get_trending(&self) -> Result<Vec<TokenPrice>, ServiceError> {
        Ok(MARKET_DATA_SERVICE.get_trending())
    }

    async fn get_top_gainers(&self) -> Result<Vec<TokenPrice>, ServiceError> {
        Ok(MARKET_DATA_SERVICE.get_top_gainers())
    }

    async fn get_top_losers(&self) -> Result<Vec<TokenPrice>, ServiceError> {
        Ok(MARKET_DATA_SERVICE.get_top_losers())
    }
}
