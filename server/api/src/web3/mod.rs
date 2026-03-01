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
    DirectContractCallInput, KeyCreateInput,
};
pub use server_service::web3::Web3KeyManagerService;
pub use server_service::web3::nft::{NFTInfo, NFTService};
pub use server_service::web3::{KLineService, IndicatorService, Candlestick, TimePeriod, TradingPair, TechnicalAnalysis, VwapData, AtrData};
pub use server_service::web3::price_fetcher::{PriceService, RealPriceData, CoinSearchResult};
pub use server_service::web3::swap_service::{SwapService, SwapQuote, SwapTransaction, SwapRoute, TokenSwapInfo};
pub use server_service::web3::order::{
    CreateOrderInput, OrderInfo, OrderListFilter, CancelOrderInput,
    OrderType, OrderSide, OrderStatus, TimeInForce,
    calculate_slippage, validate_order_input,
};
pub use server_service::web3::bridge::{
    ChainInfo, TokenInfo, BridgeQuote, BridgeTransaction, BridgeProtocol,
    BridgeTransactionRequest, get_supported_chains, get_bridge_tokens, get_bridge_protocols,
};
pub use server_service::web3::bridge;
pub use server_service::web3::address_book::{
    AddressBookService, CreateAddressInput, UpdateAddressInput, AddressBookEntry,
    TAddressBookService,
};
pub use server_service::web3::gas_analytics::{
    GasAnalyticsService, TGasAnalyticsService,
    GasAnalyticsInput, GasAnalyticsEntry, GasAnalyticsSummary,
    GasByHourInput, GasHourlyAnalytics,
    GasByDayOfWeekInput, GasDayOfWeekAnalytics,
    GasComparisonInput, GasComparisonResult,
    GasOptimizationSuggestion,
};

use serde::Deserialize;
use std::sync::Mutex;
use std::collections::HashMap;

// In-memory order storage (for demo - should use database in production)
static ORDERS: std::sync::LazyLock<Mutex<HashMap<String, OrderInfo>>> = 
    std::sync::LazyLock::new(|| Mutex::new(HashMap::new()));

/// Input for batch NFT owners query
#[derive(Debug, Deserialize)]
pub struct NFTBatchOwnersInput {
    pub token_ids: Vec<String>,
    pub chain_id: Option<u64>,
}

/// Input for NFT details query
#[derive(Debug, Deserialize)]
pub struct NFTDetailsInput {
    pub token_ids: Vec<String>,
    pub chain_id: Option<u64>,
}

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

    // ============ Key Manager API ============

    /// Create encrypted key
    pub async fn create_key(
        Json(input): Json<KeyCreateInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match Web3KeyManagerService::create_key(input).await {
            Ok(key) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": key,
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

    /// List encrypted keys
    pub async fn list_keys(
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match Web3KeyManagerService::list_keys().await {
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

    /// Delete encrypted key
    pub async fn delete_key(
        Path(id): Path<String>,
    ) -> Json<serde_json::Value> {
        match Web3KeyManagerService::delete_key(&id).await {
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

    // ============ Block Scanner API ============
    
    /// Get block by number
    pub async fn get_block(
        Path(block_number): Path<u64>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        use server_service::web3::block_scanner::BlockScanner;
        
        let scanner = BlockScanner::new(
            "https://eth.llamarpc.com".to_string(),
            100
        );
        
        match scanner.get_block(block_number).await {
            Ok(block) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": block,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }
    
    /// Get latest block number
    pub async fn get_latest_block(
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        use server_service::web3::block_scanner::BlockScanner;
        
        let scanner = BlockScanner::new(
            "https://eth.llamarpc.com".to_string(),
            100
        );
        
        match scanner.get_latest_block().await {
            Ok(block_num) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": { "blockNumber": block_num },
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }
    
    /// Get transaction receipt by hash
    pub async fn get_transaction_receipt(
        Path(tx_hash): Path<String>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        use server_service::web3::block_scanner::BlockScanner;
        
        let scanner = BlockScanner::new(
            "https://eth.llamarpc.com".to_string(),
            100
        );
        
        match scanner.get_receipt(&tx_hash).await {
            Ok(receipt) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": receipt,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }
    
    /// Scan blocks in range
    pub async fn scan_blocks(
        Path((from, to)): Path<(u64, u64)>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        use server_service::web3::block_scanner::BlockScanner;
        
        let scanner = BlockScanner::new(
            "https://eth.llamarpc.com".to_string(),
            100
        );
        
        match scanner.scan_blocks(from, to).await {
            Ok(result) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": result,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Get NFT owner (ERC721 ownerOf)
    pub async fn get_nft_owner(
        Path((contract, token_id)): Path<(String, String)>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        let chain_id: u64 = params
            .get("chainId")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1); // Default to Ethereum mainnet
        
        match NFTService::owner_of(&contract, &token_id, chain_id).await {
            Ok(owner) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": { "owner": owner, "token_id": token_id, "contract": contract },
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Get NFT tokenURI
    pub async fn get_nft_token_uri(
        Path((contract, token_id)): Path<(String, String)>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        let chain_id: u64 = params
            .get("chainId")
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);
        
        match NFTService::token_uri(&contract, &token_id, chain_id).await {
            Ok(token_uri) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": { "token_uri": token_uri, "token_id": token_id, "contract": contract },
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Fetch NFT metadata from tokenURI
    pub async fn get_nft_metadata(
        Path(token_uri): Path<String>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match NFTService::fetch_metadata(&token_uri).await {
            Ok(metadata) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": metadata,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Get batch NFT owners
    pub async fn get_nft_owners_batch(
        Path(contract): Path<String>,
        Json(input): Json<NFTBatchOwnersInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        let chain_id = input.chain_id.unwrap_or(1);
        
        match NFTService::get_owners_batch(&contract, &input.token_ids, chain_id).await {
            Ok(owners) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": owners,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Get multiple NFT details (owner + metadata)
    pub async fn get_nfts(
        Path(contract): Path<String>,
        Json(input): Json<NFTDetailsInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        let chain_id = input.chain_id.unwrap_or(1);
        
        match NFTService::get_nfts(&contract, &input.token_ids, chain_id).await {
            Ok(nfts) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": nfts,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Get real price from CoinGecko API
    pub async fn get_real_price(
        Path(symbol): Path<String>,
    ) -> Json<serde_json::Value> {
        let service = PriceService::new();
        
        match service.get_price(&symbol).await {
            Ok(data) => Json(serde_json::json!({
                "code": 200,
                "data": data,
                "msg": "success",
                "success": true
            })),
            Err(e) => Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))
        }
    }

    /// Get top coins by market cap
    pub async fn get_top_coins(
        Query(params): Query<TopCoinsParams>,
    ) -> Json<serde_json::Value> {
        let service = PriceService::new();
        let limit = params.limit.unwrap_or(20);
        
        match service.get_top_coins(limit).await {
            Ok(data) => Json(serde_json::json!({
                "code": 200,
                "data": data,
                "msg": "success",
                "success": true
            })),
            Err(e) => Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))
        }
    }

    /// Search coins
    pub async fn search_coins(
        Query(params): Query<SearchCoinsParams>,
    ) -> Json<serde_json::Value> {
        let service = PriceService::new();
        
        match service.search_coins(&params.query).await {
            Ok(data) => Json(serde_json::json!({
                "code": 200,
                "data": data,
                "msg": "success",
                "success": true
            })),
            Err(e) => Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))
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

    /// Get K-line (candlestick) data
    pub async fn get_kline(
        Path((base, quote, period)): Path<(String, String, String)>,
        Query(params): Query<KLineQueryParams>,
    ) -> Json<serde_json::Value> {
        let service = KLineService::new();
        
        let trading_pair = TradingPair {
            base: base.to_uppercase(),
            quote: quote.to_uppercase(),
            address: None,
            chain: "ethereum".to_string(),
        };
        
        let time_period = TimePeriod::from_str(&period).unwrap_or(TimePeriod::OneHour);
        let limit = params.limit.unwrap_or(100);
        
        let candlesticks = service.get_candlesticks(&trading_pair, time_period, limit);
        
        Json(serde_json::json!({
            "code": 200,
            "data": candlesticks,
            "msg": "success",
            "success": true
        }))
    }

    /// Get latest price for a trading pair
    pub async fn get_price(
        Path((base, quote)): Path<(String, String)>,
    ) -> Json<serde_json::Value> {
        let service = KLineService::new();
        
        let trading_pair = TradingPair {
            base: base.to_uppercase(),
            quote: quote.to_uppercase(),
            address: None,
            chain: "ethereum".to_string(),
        };
        
        let price = service.get_price(&trading_pair.base);
        let (change, change_percent) = service.get_24h_change(&trading_pair.base);
        
        Json(serde_json::json!({
            "code": 200,
            "data": {
                "base": trading_pair.base,
                "quote": trading_pair.quote,
                "price": price,
                "change24h": change,
                "changePercent": change_percent
            },
            "msg": "success",
            "success": true
        }))
    }

    /// Get technical indicators for a trading pair
    pub async fn get_indicators(
        Path((base, quote, period)): Path<(String, String, String)>,
    ) -> Json<serde_json::Value> {
        let kline_service = KLineService::new();
        
        let trading_pair = TradingPair {
            base: base.to_uppercase(),
            quote: quote.to_uppercase(),
            address: None,
            chain: "ethereum".to_string(),
        };
        
        let time_period = TimePeriod::from_str(&period).unwrap_or(TimePeriod::OneHour);
        let candlesticks = kline_service.get_candlesticks(&trading_pair, time_period, 100);
        
        let indicator_service = IndicatorService::new();
        let analysis = indicator_service.analyze(&candlesticks);
        
        Json(serde_json::json!({
            "code": 200,
            "data": analysis,
            "msg": "success",
            "success": true
        }))
    }
}

/// Query params for K-line
#[derive(Debug, Deserialize)]
pub struct KLineQueryParams {
    pub limit: Option<usize>,
}

/// Query params for top coins
#[derive(Debug, Deserialize)]
pub struct TopCoinsParams {
    pub limit: Option<usize>,
}

/// Query params for coin search
#[derive(Debug, Deserialize)]
pub struct SearchCoinsParams {
    pub query: String,
}

// ============ Swap API Handlers ============

/// Get available tokens for swap
pub async fn get_swap_tokens() -> Json<serde_json::Value> {
    let service = SwapService::new();
    let tokens = service.get_swap_tokens();
    
    Json(serde_json::json!({
        "code": 200,
        "data": tokens,
        "msg": "success",
        "success": true
    }))
}

/// Get swap quote
pub async fn get_swap_quote(
    Path((from, to, amount)): Path<(String, String, String)>,
) -> Json<serde_json::Value> {
    let service = SwapService::new();
    let quote = service.get_quote(&from, &to, &amount);
    
    Json(serde_json::json!({
        "code": 200,
        "data": quote,
        "msg": "success",
        "success": true
    }))
}

/// Get swap routes
pub async fn get_swap_routes(
    Path((from, to)): Path<(String, String)>,
) -> Json<serde_json::Value> {
    let service = SwapService::new();
    let routes = service.get_routes(&from, &to);
    
    Json(serde_json::json!({
        "code": 200,
        "data": routes,
        "msg": "success",
        "success": true
    }))
}

/// Input for building swap transaction
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildSwapInput {
    pub from_token: String,
    pub to_token: String,
    pub from_amount: String,
    pub to_amount_min: String,
    pub recipient: String,
    pub slippage: f64,
}

/// Build swap transaction
pub async fn build_swap_transaction(
    Json(input): Json<BuildSwapInput>,
) -> Json<serde_json::Value> {
    let service = SwapService::new();
    
    let quote = SwapQuote {
        from_token: input.from_token,
        to_token: input.to_token,
        from_amount: input.from_amount.clone(),
        to_amount: input.to_amount_min.clone(),
        to_amount_min: input.to_amount_min,
        price_impact: input.slippage,
        gas_estimate: "210000".to_string(),
        route: vec![],
    };
    
    let tx = service.build_transaction(&quote, &input.recipient);
    
    Json(serde_json::json!({
        "code": 200,
        "data": tx,
        "msg": "success",
        "success": true
    }))
}

// ============ Order API Handlers ============

/// Create a new order
pub async fn create_order(
    Json(input): Json<CreateOrderInput>,
) -> Json<serde_json::Value> {
    // Validate order input
    if let Err(e) = validate_order_input(&input) {
        return Json(serde_json::json!({
            "code": 400,
            "msg": format!("Invalid order input: {}", e),
            "success": false
        }));
    }

    // Generate order ID
    let order_id = format!("ord_{}", &uuid::Uuid::new_v4().to_string().replace("-", "")[..16]);
    
    // Calculate slippage if not provided
    let slippage_bps = input.slippage_bps.unwrap_or(50); // Default 0.5%

    let order = OrderInfo {
        id: order_id.clone(),
        user_id: input.user_id.clone(),
        order_type: input.order_type.clone(),
        side: input.side.clone(),
        status: OrderStatus::Pending,
        token_in: input.token_in.clone(),
        token_out: input.token_out.clone(),
        amount_in: input.amount_in.clone(),
        amount_out: None,
        limit_price: input.limit_price.clone(),
        stop_price: input.stop_price.clone(),
        filled_amount: None,
        average_price: None,
        time_in_force: input.time_in_force.clone().unwrap_or(TimeInForce::GTC),
        expire_at: input.expire_at.clone(),
        chain_id: input.chain_id.unwrap_or(1),
        slippage_bps,
        tx_hash: None,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: None,
    };

    // Store order
    if let Ok(mut orders) = ORDERS.lock() {
        orders.insert(order_id.clone(), order.clone());
    }

    Json(serde_json::json!({
        "code": 200,
        "data": order,
        "msg": "Order created successfully",
        "success": true
    }))
}

/// List orders with filters
pub async fn list_orders(
    Query(params): Query<OrderListFilter>,
) -> Json<serde_json::Value> {
    let orders = if let Ok(orders) = ORDERS.lock() {
        let mut result: Vec<OrderInfo> = orders.values().cloned().collect();
        
        // Apply filters
        if let Some(ref user_id) = params.user_id {
            result.retain(|o| o.user_id.as_ref() == Some(user_id));
        }
        if let Some(ref status) = params.status {
            result.retain(|o| o.status == *status);
        }
        if let Some(ref side) = params.side {
            result.retain(|o| o.side == *side);
        }
        if let Some(ref order_type) = params.order_type {
            result.retain(|o| o.order_type == *order_type);
        }
        
        // Sort by created_at descending
        result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        // Apply limit
        if let Some(limit) = params.limit {
            result.truncate(limit as usize);
        }
        
        result
    } else {
        vec![]
    };

    Json(serde_json::json!({
        "code": 200,
        "data": orders,
        "msg": "success",
        "success": true
    }))
}

/// Get order by ID
pub async fn get_order(
    Path(order_id): Path<String>,
) -> Json<serde_json::Value> {
    if let Ok(orders) = ORDERS.lock() {
        if let Some(order) = orders.get(&order_id) {
            return Json(serde_json::json!({
                "code": 200,
                "data": order,
                "msg": "success",
                "success": true
            }));
        }
    }

    Json(serde_json::json!({
        "code": 404,
        "msg": "Order not found",
        "success": false
    }))
}

/// Cancel an order
pub async fn cancel_order(
    Path(order_id): Path<String>,
) -> Json<serde_json::Value> {
    if let Ok(mut orders) = ORDERS.lock() {
        if let Some(order) = orders.get_mut(&order_id) {
            if order.status == OrderStatus::Pending || order.status == OrderStatus::Submitted {
                order.status = OrderStatus::Cancelled;
                order.updated_at = Some(chrono::Utc::now().to_rfc3339());
                
                return Json(serde_json::json!({
                    "code": 200,
                    "data": order.clone(),
                    "msg": "Order cancelled successfully",
                    "success": true
                }));
            } else {
                return Json(serde_json::json!({
                    "code": 400,
                    "msg": "Order cannot be cancelled in current status",
                    "success": false
                }));
            }
        }
    }

    Json(serde_json::json!({
        "code": 404,
        "msg": "Order not found",
        "success": false
    }))
}

/// Update order (e.g., submit to network)
pub async fn update_order(
    Path(order_id): Path<String>,
    Json(input): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    if let Ok(mut orders) = ORDERS.lock() {
        if let Some(order) = orders.get_mut(&order_id) {
            // Update fields
            if let Some(status) = input.get("status").and_then(|s| s.as_str()) {
                match status {
                    "submitted" => order.status = OrderStatus::Submitted,
                    "filled" => order.status = OrderStatus::Filled,
                    "failed" => order.status = OrderStatus::Failed,
                    "expired" => order.status = OrderStatus::Expired,
                    _ => {}
                }
            }
            if let Some(tx_hash) = input.get("txHash").and_then(|h| h.as_str()) {
                order.tx_hash = Some(tx_hash.to_string());
            }
            if let Some(amount_out) = input.get("amountOut").and_then(|a| a.as_str()) {
                order.amount_out = Some(amount_out.to_string());
            }
            if let Some(filled) = input.get("filledAmount").and_then(|f| f.as_str()) {
                order.filled_amount = Some(filled.to_string());
            }
            
            order.updated_at = Some(chrono::Utc::now().to_rfc3339());
            
            return Json(serde_json::json!({
                "code": 200,
                "data": order.clone(),
                "msg": "Order updated successfully",
                "success": true
            }));
        }
    }

    Json(serde_json::json!({
        "code": 404,
        "msg": "Order not found",
        "success": false
    }))
}

// ============ Bridge API ============

/// Bridge API endpoints
pub struct Web3BridgeApi;

impl Web3BridgeApi {
    /// Get supported chains for bridging
    pub async fn get_chains() -> Json<serde_json::Value> {
        let chains = get_supported_chains();
        Json(serde_json::json!({
            "code": 200,
            "data": chains,
            "msg": "success",
            "success": true
        }))
    }

    /// Get bridge tokens for a specific chain
    pub async fn get_tokens(
        Path(chain_id): Path<u64>,
    ) -> Json<serde_json::Value> {
        let tokens = get_bridge_tokens(chain_id);
        Json(serde_json::json!({
            "code": 200,
            "data": tokens,
            "msg": "success",
            "success": true
        }))
    }

    /// Get supported bridge protocols
    pub async fn get_protocols() -> Json<serde_json::Value> {
        let protocols = get_bridge_protocols();
        Json(serde_json::json!({
            "code": 200,
            "data": protocols,
            "msg": "success",
            "success": true
        }))
    }

    /// Get bridge quote
    pub async fn get_quote(
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Json<serde_json::Value> {
        let from_chain_id = params.get("fromChain")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(1);
        let to_chain_id = params.get("toChain")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(137);
        let from_token = params.get("fromToken").cloned().unwrap_or_else(|| "ETH".to_string());
        let to_token = params.get("toToken").cloned().unwrap_or_else(|| "MATIC".to_string());
        let amount = params.get("amount").cloned().unwrap_or_else(|| "1".to_string());

        let chains = get_supported_chains();
        let from_chain = chains.iter().find(|c| c.id == from_chain_id).cloned();
        let to_chain = chains.iter().find(|c| c.id == to_chain_id).cloned();

        if from_chain.is_none() || to_chain.is_none() {
            return Json(serde_json::json!({
                "code": 400,
                "data": null,
                "msg": "Invalid chain ID",
                "success": false
            }));
        }

        let from_chain = from_chain.unwrap();
        let to_chain = to_chain.unwrap();

        let from_tokens = get_bridge_tokens(from_chain_id);
        let to_tokens = get_bridge_tokens(to_chain_id);

        let from_token_info = from_tokens.iter().find(|t| t.symbol == from_token).cloned();
        let to_token_info = to_tokens.iter().find(|t| t.symbol == to_token).cloned();

        if from_token_info.is_none() || to_token_info.is_none() {
            return Json(serde_json::json!({
                "code": 400,
                "data": null,
                "msg": "Invalid token",
                "success": false
            }));
        }

        let from_token_info = from_token_info.unwrap();
        let to_token_info = to_token_info.unwrap();

        // Simulate quote calculation (in production, would query actual bridge APIs)
        let exchange_rate = if from_token == to_token { "1.0".to_string() } else { "0.85".to_string() };
        let to_amount = (amount.parse::<f64>().unwrap_or(1.0) * exchange_rate.parse::<f64>().unwrap_or(0.85)).to_string();
        
        let quote = BridgeQuote {
            from_chain: from_chain.clone(),
            to_chain: to_chain.clone(),
            from_token: from_token_info.clone(),
            to_token: to_token_info.clone(),
            from_amount: amount.clone(),
            to_amount: to_amount.clone(),
            exchange_rate: exchange_rate.clone(),
            estimated_time: "5-15 min".to_string(),
            estimated_gas: "0.002 ETH".to_string(),
            bridge_fee: "0.001 ETH".to_string(),
            protocol: "LayerZero".to_string(),
            route: vec![
                crate::web3::bridge::BridgeStep {
                    step_type: "send".to_string(),
                    protocol: "LayerZero".to_string(),
                    description: "Send tokens to LayerZero bridge".to_string(),
                },
                crate::web3::bridge::BridgeStep {
                    step_type: "receive".to_string(),
                    protocol: "LayerZero".to_string(),
                    description: "Receive tokens on destination chain".to_string(),
                },
            ],
        };

        Json(serde_json::json!({
            "code": 200,
            "data": quote,
            "msg": "success",
            "success": true
        }))
    }

    /// Build bridge transaction
    pub async fn build_transaction(
        Json(input): Json<BridgeTransactionRequest>,
    ) -> Json<serde_json::Value> {
        let chains = get_supported_chains();
        let from_chain = chains.iter().find(|c| c.id == input.from_chain_id).cloned();
        let to_chain = chains.iter().find(|c| c.id == input.to_chain_id).cloned();

        if from_chain.is_none() || to_chain.is_none() {
            return Json(serde_json::json!({
                "code": 400,
                "data": null,
                "msg": "Invalid chain ID",
                "success": false
            }));
        }

        let from_chain = from_chain.unwrap();
        let to_chain = to_chain.unwrap();

        let from_tokens = get_bridge_tokens(input.from_chain_id);
        let to_tokens = get_bridge_tokens(input.to_chain_id);

        let from_token_info = from_tokens.iter().find(|t| t.symbol == input.from_token).cloned();
        let to_token_info = to_tokens.iter().find(|t| t.symbol == input.to_token).cloned();

        if from_token_info.is_none() || to_token_info.is_none() {
            return Json(serde_json::json!({
                "code": 400,
                "data": null,
                "msg": "Invalid token",
                "success": false
            }));
        }

        let from_token_info = from_token_info.unwrap();
        let to_token_info = to_token_info.unwrap();

        let exchange_rate = if input.from_token == input.to_token { "1.0" } else { "0.85" };
        let to_amount = (input.amount.parse::<f64>().unwrap_or(1.0) * exchange_rate.parse::<f64>().unwrap_or(0.85)).to_string();

        let quote = BridgeQuote {
            from_chain: from_chain.clone(),
            to_chain: to_chain.clone(),
            from_token: from_token_info.clone(),
            to_token: to_token_info.clone(),
            from_amount: input.amount.clone(),
            to_amount: to_amount.clone(),
            exchange_rate: exchange_rate.to_string(),
            estimated_time: "5-15 min".to_string(),
            estimated_gas: "0.002 ETH".to_string(),
            bridge_fee: "0.001 ETH".to_string(),
            protocol: "LayerZero".to_string(),
            route: vec![],
        };

        // Build transaction data (simulated - in production would call actual bridge API)
        let tx_data = crate::web3::bridge::TransactionData {
            to: "0x0000000000000000000000000000000000000000".to_string(),
            data: "0x".to_string(),
            value: input.amount.clone(),
            gas_limit: "210000".to_string(),
            gas_price: None,
        };

        let transaction = BridgeTransaction {
            quote,
            tx_data,
            approval_address: Some("0x0000000000000000000000000000000000000001".to_string()),
        };

        Json(serde_json::json!({
            "code": 200,
            "data": transaction,
            "msg": "success",
            "success": true
        }))
    }

    /// Get bridge history for a wallet
    pub async fn get_history(
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Json<serde_json::Value> {
        let address = params.get("address").cloned().unwrap_or_default();
        
        // Return empty history (in production would query database)
        let history: Vec<crate::web3::bridge::BridgeHistory> = vec![];

        Json(serde_json::json!({
            "code": 200,
            "data": history,
            "msg": "success",
            "success": true,
            "address": address
        }))
    }
}

// ============ Token Approval Manager API ============

/// Token approval info
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenApproval {
    pub token_address: String,
    pub token_symbol: String,
    pub token_name: String,
    pub token_decimals: u8,
    pub spender: String,
    pub spender_name: Option<String>,
    pub allowance: String,
    pub is_infinite: bool,
    pub approved_at: i64,
    pub age_days: i64,
    pub risk_level: String,
}

/// Input for token approvals query
#[derive(Debug, Deserialize)]
pub struct TokenApprovalsInput {
    pub owner: String,
    pub chain_id: Option<u64>,
}

/// Known spender names (in production would use a database)
fn get_spender_name(address: &str) -> Option<String> {
    let known_spenders: std::collections::HashMap<&str, &str> = [
        ("0x7a250d5630b4cf539739df2c5dacb4c659f2488d", "Uniswap V2 Router"),
        ("0xe592427a0aece92de3edee1f18e0157c05861564", "Uniswap V3 Router"),
        ("0x3fc91a3cfd7c327db8c2d3ed1e9c3a1e7a1d8c3a", "Uniswap V3 Position Manager"),
        ("0xd9e1ce17f2641f24ae83637ab66a2cca9c378b9f", "SushiSwap Router"),
        ("0x10ed43c718714eb63d5aa57b78b54704e256024e", "PancakeSwap Router"),
        ("0xca143ce32fe78f1f7019d7d551a6402fc5350c73", "PancakeSwap Factory"),
        ("0x7d01a62c340e8d4de4b2c4f3e3c3e3c3e3c3e3c3", "Aave V3 Pool"),
        ("0x87870bca3f3f6335e32cd4f83c9f4ca3e3c3e3c3", "Aave V3"),
        ("0xae60a8e3e99c03c2e70b6e22f0c3e3c3e3c3e3c3", "Compound V3"),
        ("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984", "Uniswap V3 Factory"),
        ("0xc2edad668740f1aa29e859e50d6b5560d1c6be20", "Curve Finance"),
        ("0xa540074e41ea64b3afc23eab8e80d9aa695c8c67", "Curve Registry"),
        ("0x5c60da1bfe55ec7b88c9c2c3e3e3e3e3e3e3e3c3", "Yearn Vaults"),
        ("0x93c08f316fe9eb86c2beaf5b3e3e3e3e3e3e3e3c3", "Yearn"),
        ("0xba12222222228d8ba445958a75a0704d566bf2c8", "Balancer V2 Vault"),
        ("0x8fd312a03eb6c4c8c6e5d6e6e6e6e6e6e6e6e6e3", "Synthetix"),
        ("0x3f5ce5fbfe3e9af3971dd833d26ba9b5c936f0be", "Binance"),
        ("0xf977814e90da44bfa03b6295a0616a897441acec", "Binance Hot Wallet"),
    ].iter().cloned().collect();
    
    known_spenders.get(address.to_lowercase().as_str()).map(|s| s.to_string())
}

/// Determine risk level based on approval
fn calculate_risk_level(is_infinite: bool, age_days: i64) -> String {
    if is_infinite {
        "high".to_string()
    } else if age_days > 365 {
        "medium".to_string()
    } else {
        "low".to_string()
    }
}

impl Web3Api {
    /// Get token approvals for a wallet
    /// This queries common DeFi protocols for token approvals
    pub async fn get_token_approvals(
        Query(input): Query<TokenApprovalsInput>,
    ) -> Json<serde_json::Value> {
        let chain_id = input.chain_id.unwrap_or(1);
        let owner = input.owner.to_lowercase();
        
        // Common ERC20 tokens and their approval targets to check
        // In production, this would scan through event logs or use an indexer
        let common_tokens: Vec<(&str, &str, &str, u8)> = vec![
            // ETH Mainnet tokens
            ("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", "USDC", "USD Coin", 6),
            ("0x6b175474e89094c44da98b954eedeac495271d0f", "DAI", "Dai Stablecoin", 18),
            ("0x2260fac5e5542a773aa44fbcfedf7c193bc2c599", "WBTC", "Wrapped Bitcoin", 8),
            ("0xc00e94cb662c3520282e6f5717214004a7f26888", "COMP", "Compound", 18),
            ("0x514910771af9ca656af840dff83e8264ecf986ca", "LINK", "Chainlink", 18),
            ("0x7fc66500c84a76ad7e9c93437bfc5ac33e2ddae9", "AAVE", "Aave", 18),
            ("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984", "UNI", "Uniswap", 18),
            ("0x0d8775f648430679a709e98d2b0cb6250d2887ed", "BAT", "Basic Attention Token", 18),
            ("0x1985365e9f78359a9b6ad760e32412f4a445e896", "REP", "Augur", 18),
            ("0xdd974d5c2e2928dea5f71b9824b8b0698e4e1eab", "KNC", "Kyber Network", 18),
            ("0x80fb784b7ed66730e8b1dbd9820afd6c31fbb905", "ZRX", "0x", 18),
            ("0x408e41876cccdc0f92210600ef50372656052a20", "REN", "RenVM", 18),
            ("0xba7435a4b4c747e0101780073eeda872a69bdcd4", "HBTC", "Huobi BTC", 18),
            ("0x8798249c2e607446efb7ad49ec89dd1865f39f02", "xSUSHI", "SushiBar", 18),
            ("0x8798249c2e607446efb7ad49ec89dd1865f39f02", "SUSHI", "SushiSwap", 18),
        ];
        
        let common_spenders = vec![
            "0x7a250d5630b4cf539739df2c5dacb4c659f2488d", // Uniswap V2
            "0xe592427a0aece92de3edee1f18e0157c05861564", // Uniswap V3
            "0xd9e1ce17f2641f24ae83637ab66a2cca9c378b9f", // SushiSwap
            "0x10ed43c718714eb63d5aa57b78b54704e256024e", // PancakeSwap
            "0x7d01a62c340e8d4de4b2c4f3e3c3e3e3e3c3e3c3", // Aave
            "0xa540074e41ea64b3afc23eab8e80d9aa695c8c67", // Curve
            "0xba12222222228d8ba445958a75a0704d566bf2c8", // Balancer
        ];
        
        let mut approvals: Vec<TokenApproval> = vec![];
        let current_time = chrono::Utc::now().timestamp();
        
        // Generate demo approvals (in production would query blockchain)
        // For demo purposes, we'll generate some realistic-looking data
        for (i, (token_addr, symbol, name, decimals)) in common_tokens.iter().enumerate() {
            // Randomly assign some spenders to show variety
            if i % 3 == 0 {
                for (j, spender) in common_spenders.iter().enumerate() {
                    if j <= i % 4 {
                        let is_infinite = i % 5 == 0;
                        let approved_days_ago = (i as i64 * 30 + j as i64 * 7) % 730;
                        let approved_at = current_time - (approved_days_ago * 24 * 60 * 60);
                        
                        approvals.push(TokenApproval {
                            token_address: token_addr.to_string(),
                            token_symbol: symbol.to_string(),
                            token_name: name.to_string(),
                            token_decimals: *decimals,
                            spender: spender.to_string(),
                            spender_name: get_spender_name(spender),
                            allowance: if is_infinite {
                                "115792089237316195423570985008687907853269984665640564039457.584007913129639935".to_string()
                            } else {
                                format!("{}", (1000000u64 + i as u64 * 50000) * 10u64.pow(*decimals as u32))
                            },
                            is_infinite,
                            approved_at,
                            age_days: approved_days_ago,
                            risk_level: calculate_risk_level(is_infinite, approved_days_ago),
                        });
                    }
                }
            }
        }
        
        // Sort by risk level (high first)
        approvals.sort_by(|a, b| {
            let order = |r: &str| match r {
                "high" => 0,
                "medium" => 1,
                _ => 2,
            };
            order(&a.risk_level).cmp(&order(&b.risk_level))
        });

        Json(serde_json::json!({
            "code": 200,
            "data": {
                "owner": owner,
                "chain_id": chain_id,
                "approvals": approvals,
                "total": approvals.len(),
                "high_risk": approvals.iter().filter(|a| a.risk_level == "high").count(),
                "medium_risk": approvals.iter().filter(|a| a.risk_level == "medium").count(),
                "low_risk": approvals.iter().filter(|a| a.risk_level == "low").count(),
            },
            "msg": "success",
            "success": true
        }))
    }

    /// Get detailed info about a specific spender/protocol
    pub async fn get_spender_info(
        Path(spender): Path<String>,
    ) -> Json<serde_json::Value> {
        let spender_lower = spender.to_lowercase();
        
        // Protocol info (in production would be a database)
        let protocols = serde_json::json!([
            {
                "address": "0x7a250d5630b4cf539739df2c5dacb4c659f2488d",
                "name": "Uniswap V2 Router",
                "description": "Decentralized exchange protocol",
                "website": "https://uniswap.org",
                "category": "DEX",
                "risk_score": "low"
            },
            {
                "address": "0xe592427a0aece92de3edee1f18e0157c05861564",
                "name": "Uniswap V3 Router",
                "description": "Uniswap V3 protocol",
                "website": "https://uniswap.org",
                "category": "DEX",
                "risk_score": "low"
            },
            {
                "address": "0xd9e1ce17f2641f24ae83637ab66a2cca9c378b9f",
                "name": "SushiSwap Router",
                "description": "Decentralized exchange",
                "website": "https://sushi.com",
                "category": "DEX",
                "risk_score": "low"
            },
            {
                "address": "0x10ed43c718714eb63d5aa57b78b54704e256024e",
                "name": "PancakeSwap Router",
                "description": "Binance Smart Chain DEX",
                "website": "https://pancakeswap.finance",
                "category": "DEX",
                "risk_score": "low"
            },
            {
                "address": "0x7d01a62c340e8d4de4b2c4f3e3c3e3e3e3c3e3c3",
                "name": "Aave V3 Pool",
                "description": "Lending protocol",
                "website": "https://aave.com",
                "category": "Lending",
                "risk_score": "medium"
            }
        ]);
        
        // Find matching protocol
        let protocol = protocols.as_array()
            .and_then(|arr| arr.iter().find(|p| p["address"].as_str() == Some(&spender_lower)));

        Json(serde_json::json!({
            "code": 200,
            "data": protocol,
            "msg": "success",
            "success": true
        }))
    }

    /// Generate revocation transaction data
    pub async fn create_approval_revoke_tx(
        Json(input): Json<TokenApprovalsInput>,
    ) -> Json<serde_json::Value> {
        let owner = input.owner.clone();
        let _chain_id = input.chain_id.unwrap_or(1);
        
        // In production, would generate actual transaction data to set allowance to 0
        // This is a simulation for demo purposes
        
        Json(serde_json::json!({
            "code": 200,
            "data": {
                "to": owner, // Token address would go here
                "data": "0x095ea7b30000000000000000000000000000000000000000000000000000000000000000", // set allowance to 0
                "value": "0",
                "description": "Revoke token approval - set allowance to 0"
            },
            "msg": "Approval revoke transaction generated. Sign and send to revoke all approvals.",
            "success": true
        }))
    }
}

// ============ Address Book API ============

impl Web3Api {
    /// Create a new address book entry
    pub async fn create_address_book_entry(
        Extension(service): Extension<Arc<AddressBookService>>,
        Json(input): Json<CreateAddressInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        // For demo, use a default user_id
        let user_id = "default_user";
        match service.create_address(user_id, input).await {
            Ok(entry) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": entry,
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

    /// Update an address book entry
    pub async fn update_address_book_entry(
        Extension(service): Extension<Arc<AddressBookService>>,
        Json(input): Json<UpdateAddressInput>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.update_address(input).await {
            Ok(entry) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": entry,
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

    /// Delete an address book entry
    pub async fn delete_address_book_entry(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<AddressBookService>>,
    ) -> Json<serde_json::Value> {
        match service.delete_address(&id).await {
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

    /// Get all address book entries for a user
    pub async fn list_address_book_entries(
        Extension(service): Extension<Arc<AddressBookService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        let user_id = "default_user";
        match service.list_addresses(user_id).await {
            Ok(entries) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": entries,
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

    /// Search address book entries
    pub async fn search_address_book(
        Query(params): Query<AddressSearchParams>,
        Extension(service): Extension<Arc<AddressBookService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        let user_id = "default_user";
        match service.search_addresses(user_id, &params.q).await {
            Ok(entries) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": entries,
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

    /// Toggle favorite status
    pub async fn toggle_address_favorite(
        Path(id): Path<String>,
        Extension(service): Extension<Arc<AddressBookService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.toggle_favorite(&id).await {
            Ok(entry) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": entry,
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

/// Query params for address search
#[derive(Debug, Deserialize)]
pub struct AddressSearchParams {
    q: String,
}

// ============ Gas Analytics API ============

impl Web3Api {
    /// Get gas analytics for an address
    pub async fn get_gas_analytics(
        Query(params): Query<GasAnalyticsInput>,
        Extension(service): Extension<Arc<GasAnalyticsService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_gas_analytics(params).await {
            Ok(data) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": data,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Get gas analytics summary
    pub async fn get_gas_summary(
        Query(params): Query<GasAnalyticsInput>,
        Extension(service): Extension<Arc<GasAnalyticsService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_gas_summary(params).await {
            Ok(data) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": data,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Get gas analytics by hour
    pub async fn get_gas_by_hour(
        Query(params): Query<GasByHourInput>,
        Extension(service): Extension<Arc<GasAnalyticsService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_gas_by_hour(params).await {
            Ok(data) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": data,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Get gas analytics by day of week
    pub async fn get_gas_by_day_of_week(
        Query(params): Query<GasByDayOfWeekInput>,
        Extension(service): Extension<Arc<GasAnalyticsService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_gas_by_day_of_week(params).await {
            Ok(data) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": data,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Get gas optimization suggestions
    pub async fn get_gas_suggestions(
        Query(params): Query<GasAnalyticsInput>,
        Extension(service): Extension<Arc<GasAnalyticsService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.get_gas_suggestions(params).await {
            Ok(data) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": data,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }

    /// Compare gas usage between periods
    pub async fn compare_gas_periods(
        Query(params): Query<GasComparisonInput>,
        Extension(service): Extension<Arc<GasAnalyticsService>>,
    ) -> Result<Json<serde_json::Value>, axum::response::Response> {
        match service.compare_gas_periods(params).await {
            Ok(data) => Ok(Json(serde_json::json!({
                "code": 200,
                "data": data,
                "msg": "success",
                "success": true
            }))),
            Err(e) => Ok(Json(serde_json::json!({
                "code": 500,
                "data": null,
                "msg": e,
                "success": false
            }))),
        }
    }
}
