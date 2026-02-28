use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use server_service::web3::{
    ContractCallInput, ContractCreateInput, ContractUpdateInput, 
    TContractService, TTransactionService, TWalletService, TMarketDataService,
    WalletBalanceInput, WalletListInput, WalletVerifyInput,
    Web3ContractService, Web3TransactionService, Web3WalletService,
    Web3MarketDataService, TokenBalanceInput, TransactionReceipt,
    DirectContractCallInput,
};

pub struct Web3Api;

// ============ Wallet API ============

impl Web3Api {
    /// Verify wallet ownership
    pub async fn verify_wallet(
        Extension(service): Extension<Arc<Web3WalletService>>,
        Json(input): Json<WalletVerifyInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.verify_wallet(input).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// List wallets
    pub async fn list_wallets(
        Query(params): Query<WalletListInput>,
        Extension(service): Extension<Arc<Web3WalletService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.list_wallets(params).await {
            Ok(list) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": list,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Delete wallet
    pub async fn delete_wallet(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3WalletService>>,
    ) -> Json<serde_json::Value> {
        match service.delete_wallet(&id).await {
            Ok(_) => Json(serde_json::json!({
                "code": 200,
                "data": true,
                "msg": "success",
                "success": true
            })),
            Err(e) => Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            })),
        }
    }

    /// Get wallet balance
    pub async fn get_balance(
        Extension(service): Extension<Arc<Web3WalletService>>,
        Json(input): Json<WalletBalanceInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_balance(&input.address, input.chain_id.unwrap_or(1)).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }
}

// ============ Contract API ============

pub struct Web3ContractApi;

impl Web3ContractApi {
    /// Create contract
    pub async fn create_contract(
        Extension(service): Extension<Arc<Web3ContractService>>,
        Json(input): Json<ContractCreateInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.create_contract(input).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// List contracts
    pub async fn list_contracts(
        Extension(service): Extension<Arc<Web3ContractService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.list_contracts().await {
            Ok(list) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": list,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Get contract
    pub async fn get_contract(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3ContractService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_contract(&id).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Update contract
    pub async fn update_contract(
        Extension(service): Extension<Arc<Web3ContractService>>,
        Json(input): Json<ContractUpdateInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.update_contract(input).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Delete contract
    #[allow(unused_variables)]
    pub async fn delete_contract(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<Web3ContractService>>,
    ) -> Json<serde_json::Value> {
        match service.delete_contract(&id).await {
            Ok(_) => Json(serde_json::json!({
                "code": 200,
                "data": true,
                "msg": "success",
                "success": true
            })),
            Err(e) => Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            })),
        }
    }

    /// Call contract method
    #[allow(unused_variables)]
    pub async fn call_contract(
        Path(contract_id): Path<String>,
        Extension(service): Extension<Arc<Web3ContractService>>,
        mut input: Json<ContractCallInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        // Use contract_id from path if input doesn't have it
        if input.0.contract_id.is_empty() {
            input.0.contract_id = contract_id;
        }
        match service.call_contract(input.0).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Get token balances for an address
    pub async fn get_token_balances(
        Extension(service): Extension<Arc<Web3ContractService>>,
        Json(input): Json<TokenBalanceInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_token_balances(input).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Direct contract call by address (no contract ID needed)
    pub async fn call_contract_direct(
        Extension(service): Extension<Arc<Web3ContractService>>,
        Json(input): Json<DirectContractCallInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.call_contract_direct(input).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }
}

// ============ Transaction API ============

pub struct Web3TransactionApi;

impl Web3TransactionApi {
    /// List transactions
    pub async fn list_transactions(
        Query(userid): Query<Option<String>>,
        Extension(service): Extension<Arc<Web3TransactionService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.list_transactions(userid).await {
            Ok(list) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": list,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Parse transaction receipt and extract events
    pub async fn parse_receipt(
        Extension(service): Extension<Arc<Web3TransactionService>>,
        Json(receipt): Json<TransactionReceipt>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.parse_receipt(receipt).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }
}

// ============ Market Data API ============

pub struct Web3MarketDataApi;

impl Web3MarketDataApi {
    /// Get token price by symbol
    pub async fn get_token_price(
        Path(symbol): Path<String>,
        Extension(service): Extension<Arc<Web3MarketDataService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_token_price(&symbol).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Get all token prices
    pub async fn get_all_prices(
        Extension(service): Extension<Arc<Web3MarketDataService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_all_prices().await {
            Ok(list) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": list,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Get market overview
    pub async fn get_market_overview(
        Extension(service): Extension<Arc<Web3MarketDataService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_market_overview().await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Get gas price for a chain
    pub async fn get_gas_price(
        Path(chain_id): Path<u64>,
        Extension(service): Extension<Arc<Web3MarketDataService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_gas_price(chain_id).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Get DeFi protocols
    pub async fn get_defi_protocols(
        Extension(service): Extension<Arc<Web3MarketDataService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_defi_protocols().await {
            Ok(list) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": list,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Get price history
    pub async fn get_price_history(
        Path((symbol, days)): Path<(String, u32)>,
        Extension(service): Extension<Arc<Web3MarketDataService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_price_history(&symbol, days).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Search tokens
    pub async fn search_tokens(
        Path(query): Path<String>,
        Extension(service): Extension<Arc<Web3MarketDataService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.search_tokens(&query).await {
            Ok(list) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": list,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Get trending tokens
    pub async fn get_trending(
        Extension(service): Extension<Arc<Web3MarketDataService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_trending().await {
            Ok(list) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": list,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Get top gainers
    pub async fn get_top_gainers(
        Extension(service): Extension<Arc<Web3MarketDataService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_top_gainers().await {
            Ok(list) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": list,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }

    /// Get top losers
    pub async fn get_top_losers(
        Extension(service): Extension<Arc<Web3MarketDataService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_top_losers().await {
            Ok(list) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": list,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e.message,
                "success": false
            }))),
        }
    }
}
