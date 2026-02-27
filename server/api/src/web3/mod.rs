use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension,
};
use server_core::web::{error::AppError, res::Res, validator::ValidatedJson};
use server_service::web3::{
    ContractCallInput, ContractCreateInput, ContractUpdateInput, 
    TContractService, TTransactionService, TWalletService,
    WalletListInput, WalletVerifyInput,
    Web3ContractService, Web3TransactionService, Web3WalletService,
};

pub struct Web3Api;

// ============ Wallet API ============

impl Web3Api {
    /// Verify wallet ownership
    pub async fn verify_wallet(
        Extension(service): Extension<Arc<Web3WalletService>>,
        ValidatedJson(input): ValidatedJson<WalletVerifyInput>,
    ) -> Result<Res<server_model::web3::output::WalletInfo, AppError>> {
        service.verify_wallet(input).await.map(Res::new_data)
    }

    /// List wallets
    pub async fn list_wallets(
        Query(params): Query<WalletListInput>,
        Extension(service): Extension<Arc<Web3WalletService>>,
    ) -> Result<Res<Vec<server_model::web3::output::WalletInfo>, AppError>> {
        service.list_wallets(params).await.map(Res::new_data)
    }

    /// Delete wallet
    pub async fn delete_wallet(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3WalletService>>,
    ) -> Res<bool> {
        match service.delete_wallet(&id).await {
            Ok(_) => Res::new_data(true),
            Err(e) => Res::new_error(e),
        }
    }
}

// ============ Contract API ============

pub struct Web3ContractApi;

impl Web3ContractApi {
    /// Create contract
    pub async fn create_contract(
        Extension(service): Extension<Arc<Web3ContractService>>,
        ValidatedJson(input): ValidatedJson<ContractCreateInput>,
    ) -> Result<Res<server_model::web3::output::ContractInfo, AppError>> {
        service.create_contract(input).await.map(Res::new_data)
    }

    /// List contracts
    pub async fn list_contracts(
        Extension(service): Extension<Arc<Web3ContractService>>,
    ) -> Result<Res<Vec<server_model::web3::output::ContractInfo>, AppError>> {
        service.list_contracts().await.map(Res::new_data)
    }

    /// Get contract
    pub async fn get_contract(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3ContractService>>,
    ) -> Result<Res<server_model::web3::output::ContractInfo, AppError>> {
        service.get_contract(&id).await.map(Res::new_data)
    }

    /// Update contract
    pub async fn update_contract(
        Extension(service): Extension<Arc<Web3ContractService>>,
        ValidatedJson(input): ValidatedJson<ContractUpdateInput>,
    ) -> Result<Res<server_model::web3::output::ContractInfo, AppError>> {
        service.update_contract(input).await.map(Res::new_data)
    }

    /// Delete contract
    pub async fn delete_contract(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3ContractService>>,
    ) -> Res<bool> {
        match service.delete_contract(&id).await {
            Ok(_) => Res::new_data(true),
            Err(e) => Res::new_error(e),
        }
    }

    /// Call contract method
    pub async fn call_contract(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3ContractService>>,
        ValidatedJson(input): ValidatedJson<ContractCallInput>,
    ) -> Result<Res<server_model::web3::output::ContractCallOutput, AppError>> {
        service.call_contract(input).await.map(Res::new_data)
    }
}

// ============ Transaction API ============

pub struct Web3TransactionApi;

impl Web3TransactionApi {
    /// List transactions
    pub async fn list_transactions(
        Query(user_id): Query<Option<String>>,
        Extension(service): Extension<Arc<Web3TransactionService>>,
    ) -> Result<Res<Vec<server_model::web3::output::TransactionInfo>>, AppError>> {
        service.list_transactions(user_id).await.map(Res::new_data)
    }
}
