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
    ) -> Result<Res<serde_json::Value>, AppError> {
        let result = service.verify_wallet(input).await.map_err(|e| AppError { code: 500, message: e.message })?;
        Ok(Res {
            code: 200,
            data: Some(serde_json::to_value(result).unwrap_or(serde_json::Value::Null)),
            msg: "success".to_string(),
            success: true,
        })
    }

    /// List wallets
    pub async fn list_wallets(
        Query(params): Query<WalletListInput>,
        Extension(service): Extension<Arc<Web3WalletService>>,
    ) -> Result<Res<serde_json::Value>, AppError> {
        let result = service.list_wallets(params).await.map_err(|e| AppError { code: 500, message: e.message })?;
        Ok(Res {
            code: 200,
            data: Some(serde_json::to_value(result).unwrap_or(serde_json::Value::Null)),
            msg: "success".to_string(),
            success: true,
        })
    }

    /// Delete wallet
    pub async fn delete_wallet(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3WalletService>>,
    ) -> Res<serde_json::Value> {
        match service.delete_wallet(&id).await {
            Ok(_) => Res { code: 200, data: Some(serde_json::json!(true)), msg: "success".to_string(), success: true },
            Err(e) => Res { code: 500, data: None, msg: e.message, success: false },
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
    ) -> Result<Res<serde_json::Value>, AppError> {
        let result = service.create_contract(input).await.map_err(|e| AppError { code: 500, message: e.message })?;
        Ok(Res {
            code: 200,
            data: Some(serde_json::to_value(result).unwrap_or(serde_json::Value::Null)),
            msg: "success".to_string(),
            success: true,
        })
    }

    /// List contracts
    pub async fn list_contracts(
        Extension(service): Extension<Arc<Web3ContractService>>,
    ) -> Result<Res<serde_json::Value>, AppError> {
        let result = service.list_contracts().await.map_err(|e| AppError { code: 500, message: e.message })?;
        Ok(Res {
            code: 200,
            data: Some(serde_json::to_value(result).unwrap_or(serde_json::Value::Null)),
            msg: "success".to_string(),
            success: true,
        })
    }

    /// Get contract
    pub async fn get_contract(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3ContractService>>,
    ) -> Result<Res<serde_json::Value>, AppError> {
        let result = service.get_contract(&id).await.map_err(|e| AppError { code: 500, message: e.message })?;
        Ok(Res {
            code: 200,
            data: Some(serde_json::to_value(result).unwrap_or(serde_json::Value::Null)),
            msg: "success".to_string(),
            success: true,
        })
    }

    /// Update contract
    pub async fn update_contract(
        Extension(service): Extension<Arc<Web3ContractService>>,
        ValidatedJson(input): ValidatedJson<ContractUpdateInput>,
    ) -> Result<Res<serde_json::Value>, AppError> {
        let result = service.update_contract(input).await.map_err(|e| AppError { code: 500, message: e.message })?;
        Ok(Res {
            code: 200,
            data: Some(serde_json::to_value(result).unwrap_or(serde_json::Value::Null)),
            msg: "success".to_string(),
            success: true,
        })
    }

    /// Delete contract
    pub async fn delete_contract(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3ContractService>>,
    ) -> Res<serde_json::Value> {
        match service.delete_contract(&id).await {
            Ok(_) => Res { code: 200, data: Some(serde_json::json!(true)), msg: "success".to_string(), success: true },
            Err(e) => Res { code: 500, data: None, msg: e.message, success: false },
        }
    }

    /// Call contract method
    pub async fn call_contract(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3ContractService>>,
        ValidatedJson(input): ValidatedJson<ContractCallInput>,
    ) -> Result<Res<serde_json::Value>, AppError> {
        let result = service.call_contract(input).await.map_err(|e| AppError { code: 500, message: e.message })?;
        Ok(Res {
            code: 200,
            data: Some(serde_json::to_value(result).unwrap_or(serde_json::Value::Null)),
            msg: "success".to_string(),
            success: true,
        })
    }
}

// ============ Transaction API ============

pub struct Web3TransactionApi;

impl Web3TransactionApi {
    /// List transactions
    pub async fn list_transactions(
        Query(user_id): Query<Option<String>>,
        Extension(service): Extension<Arc<Web3TransactionService>>,
    ) -> Result<Res<serde_json::Value>, AppError> {
        let result = service.list_transactions(user_id).await.map_err(|e| AppError { code: 500, message: e.message })?;
        Ok(Res {
            code: 200,
            data: Some(serde_json::to_value(result).unwrap_or(serde_json::Value::Null)),
            msg: "success".to_string(),
            success: true,
        })
    }
}
