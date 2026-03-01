use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::web3::{Web3Api, Web3ContractApi, Web3TransactionApi, Web3MarketDataApi, get_swap_tokens, get_swap_quote, get_swap_routes, build_swap_transaction, create_order, list_orders, get_order, cancel_order, update_order};
use server_global::global::{add_route, RouteInfo};

pub struct Web3Router;

impl Web3Router {
    pub async fn init_web3_router() -> Router {
        let base_path = "/web3";
        
        // Wallet routes
        let wallet_routes = vec![
            RouteInfo::new(
                &format!("{}/wallet/verify", base_path),
                Method::POST,
                "Web3Api",
                "Verify wallet ownership",
            ),
            RouteInfo::new(
                &format!("{}/wallet/list", base_path),
                Method::GET,
                "Web3Api",
                "List wallets",
            ),
            RouteInfo::new(
                &format!("{}/wallet/:id", base_path),
                Method::DELETE,
                "Web3Api",
                "Delete wallet",
            ),
            RouteInfo::new(
                &format!("{}/wallet/balance", base_path),
                Method::POST,
                "Web3Api",
                "Get wallet balance",
            ),
        ];

        // Contract routes
        let contract_routes = vec![
            RouteInfo::new(
                &format!("{}/contract", base_path),
                Method::POST,
                "Web3ContractApi",
                "Create contract",
            ),
            RouteInfo::new(
                &format!("{}/contract/list", base_path),
                Method::GET,
                "Web3ContractApi",
                "List contracts",
            ),
            RouteInfo::new(
                &format!("{}/contract/:id", base_path),
                Method::GET,
                "Web3ContractApi",
                "Get contract",
            ),
            RouteInfo::new(
                &format!("{}/contract/:id", base_path),
                Method::PUT,
                "Web3ContractApi",
                "Update contract",
            ),
            RouteInfo::new(
                &format!("{}/contract/:id", base_path),
                Method::DELETE,
                "Web3ContractApi",
                "Delete contract",
            ),
            RouteInfo::new(
                &format!("{}/contract/:id/call", base_path),
                Method::POST,
                "Web3ContractApi",
                "Call contract method",
            ),
            RouteInfo::new(
                &format!("{}/contract/token-balances", base_path),
                Method::POST,
                "Web3ContractApi",
                "Get token balances for address",
            ),
        ];

        // Transaction routes
        let transaction_routes = vec![
            RouteInfo::new(
                &format!("{}/transaction/list", base_path),
                Method::GET,
                "Web3TransactionApi",
                "List transactions",
            ),
            RouteInfo::new(
                &format!("{}/transaction/parse-receipt", base_path),
                Method::POST,
                "Web3TransactionApi",
                "Parse transaction receipt and extract events",
            ),
        ];

        // Market data routes
        let market_data_routes = vec![
            RouteInfo::new(
                &format!("{}/market/price/:symbol", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get token price by symbol",
            ),
            RouteInfo::new(
                &format!("{}/market/prices", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get all token prices",
            ),
            RouteInfo::new(
                &format!("{}/market/overview", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get market overview",
            ),
            RouteInfo::new(
                &format!("{}/market/gas/:chainId", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get gas price for chain",
            ),
            RouteInfo::new(
                &format!("{}/market/defi", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get DeFi protocols",
            ),
            RouteInfo::new(
                &format!("{}/market/history/:symbol/:days", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get price history",
            ),
            // K-line and trading routes
            RouteInfo::new(
                &format!("{}/kline/:base/:quote/:period", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get K-line candlestick data",
            ),
            RouteInfo::new(
                &format!("{}/price/:base/:quote", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get current price for trading pair",
            ),
            RouteInfo::new(
                &format!("{}/indicators/:base/:quote/:period", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get technical indicators",
            ),
            RouteInfo::new(
                &format!("{}/market/search/:query", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Search tokens",
            ),
            RouteInfo::new(
                &format!("{}/market/trending", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get trending tokens",
            ),
            RouteInfo::new(
                &format!("{}/market/gainers", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get top gainers",
            ),
            RouteInfo::new(
                &format!("{}/market/losers", base_path),
                Method::GET,
                "Web3MarketDataApi",
                "Get top losers",
            ),
        ];

        // Key manager routes
        let key_routes = vec![
            RouteInfo::new(
                &format!("{}/key", base_path),
                Method::POST,
                "Web3Api",
                "Create encrypted key",
            ),
            RouteInfo::new(
                &format!("{}/key/list", base_path),
                Method::GET,
                "Web3Api",
                "List encrypted keys",
            ),
            RouteInfo::new(
                &format!("{}/key/:id", base_path),
                Method::DELETE,
                "Web3Api",
                "Delete encrypted key",
            ),
        ];

        // Block scanner routes
        let block_scanner_routes = vec![
            RouteInfo::new(
                &format!("{}/block/:blockNumber", base_path),
                Method::GET,
                "Web3Api",
                "Get block by number",
            ),
            RouteInfo::new(
                &format!("{}/block/latest", base_path),
                Method::GET,
                "Web3Api",
                "Get latest block number",
            ),
            RouteInfo::new(
                &format!("{}/transaction/receipt/:txHash", base_path),
                Method::GET,
                "Web3Api",
                "Get transaction receipt",
            ),
            RouteInfo::new(
                &format!("{}/scan/:from/:to", base_path),
                Method::GET,
                "Web3Api",
                "Scan blocks in range",
            ),
        ];

        // NFT routes
        let nft_routes = vec![
            RouteInfo::new(
                &format!("{}/nft/:contract/:tokenId/owner", base_path),
                Method::GET,
                "Web3Api",
                "Get NFT owner",
            ),
            RouteInfo::new(
                &format!("{}/nft/:contract/:tokenId/token-uri", base_path),
                Method::GET,
                "Web3Api",
                "Get NFT token URI",
            ),
            RouteInfo::new(
                &format!("{}/nft/metadata/:tokenUri", base_path),
                Method::GET,
                "Web3Api",
                "Get NFT metadata",
            ),
            RouteInfo::new(
                &format!("{}/nft/:contract/owners", base_path),
                Method::POST,
                "Web3Api",
                "Get batch NFT owners",
            ),
            RouteInfo::new(
                &format!("{}/nft/:contract/details", base_path),
                Method::POST,
                "Web3Api",
                "Get multiple NFT details",
            ),
        ];

        // Add all routes
        for route in wallet_routes.into_iter()
            .chain(contract_routes.into_iter())
            .chain(transaction_routes.into_iter())
            .chain(market_data_routes.into_iter())
            .chain(key_routes.into_iter()) 
            .chain(block_scanner_routes.into_iter())
            .chain(nft_routes.into_iter())
        {
            add_route(route).await;
        }

        Router::new()
            // Wallet routes
            .route("/wallet/verify", post(Web3Api::verify_wallet))
            .route("/wallet/list", get(Web3Api::list_wallets))
            .route("/wallet/{id}", delete(Web3Api::delete_wallet))
            .route("/wallet/balance", post(Web3Api::get_balance))
            // Contract routes
            .route("/contract", post(Web3ContractApi::create_contract))
            .route("/contract/list", get(Web3ContractApi::list_contracts))
            .route("/contract/{id}", get(Web3ContractApi::get_contract))
            .route("/contract/{id}", put(Web3ContractApi::update_contract))
            .route("/contract/{id}", delete(Web3ContractApi::delete_contract))
            .route("/contract/{id}/call", post(Web3ContractApi::call_contract))
            .route("/contract/token-balances", post(Web3ContractApi::get_token_balances))
            .route("/contract/call-direct", post(Web3ContractApi::call_contract_direct))
            // Transaction routes
            .route("/transaction/list", get(Web3TransactionApi::list_transactions))
            .route("/transaction/parse-receipt", post(Web3TransactionApi::parse_receipt))
            // Market data routes
            .route("/market/price/{symbol}", get(Web3MarketDataApi::get_token_price))
            .route("/market/prices", get(Web3MarketDataApi::get_all_prices))
            .route("/market/overview", get(Web3MarketDataApi::get_market_overview))
            .route("/market/gas/{chainId}", get(Web3MarketDataApi::get_gas_price))
            .route("/market/defi", get(Web3MarketDataApi::get_defi_protocols))
            .route("/market/history/{symbol}/{days}", get(Web3MarketDataApi::get_price_history))
            // K-line and trading routes
            .route("/kline/{base}/{quote}/{period}", get(Web3MarketDataApi::get_kline))
            .route("/price/{base}/{quote}", get(Web3MarketDataApi::get_price))
            .route("/indicators/{base}/{quote}/{period}", get(Web3MarketDataApi::get_indicators))
            // Real price from CoinGecko
            .route("/price/real/{symbol}", get(Web3Api::get_real_price))
            .route("/price/top", get(Web3Api::get_top_coins))
            .route("/price/search", get(Web3Api::search_coins))
            .route("/market/search/{query}", get(Web3MarketDataApi::search_tokens))
            .route("/market/trending", get(Web3MarketDataApi::get_trending))
            .route("/market/gainers", get(Web3MarketDataApi::get_top_gainers))
            .route("/market/losers", get(Web3MarketDataApi::get_top_losers))
            // Key manager routes
            .route("/key", post(Web3Api::create_key))
            .route("/key/list", get(Web3Api::list_keys))
            .route("/key/{id}", delete(Web3Api::delete_key))
            // Block scanner routes
            .route("/block/{blockNumber}", get(Web3Api::get_block))
            .route("/block/latest", get(Web3Api::get_latest_block))
            .route("/transaction/receipt/{txHash}", get(Web3Api::get_transaction_receipt))
            .route("/scan/{from}/{to}", get(Web3Api::scan_blocks))
            // NFT routes
            .route("/nft/{contract}/{tokenId}/owner", get(Web3Api::get_nft_owner))
            .route("/nft/{contract}/{tokenId}/token-uri", get(Web3Api::get_nft_token_uri))
            .route("/nft/metadata/{tokenUri}", get(Web3Api::get_nft_metadata))
            .route("/nft/{contract}/owners", post(Web3Api::get_nft_owners_batch))
            .route("/nft/{contract}/details", post(Web3Api::get_nfts))
            // Swap routes
            .route("/swap/tokens", get(get_swap_tokens))
            .route("/swap/quote/{from}/{to}/{amount}", get(get_swap_quote))
            .route("/swap/routes/{from}/{to}", get(get_swap_routes))
            .route("/swap/build", post(build_swap_transaction))
            // Order routes
            .route("/order", post(create_order))
            .route("/order/list", get(list_orders))
            .route("/order/{id}", get(get_order))
            .route("/order/{id}", put(update_order))
            .route("/order/{id}", delete(cancel_order))
    }
}
