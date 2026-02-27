use axum::{
    http::Method,
    routing::{delete, get, post, put},
    Router,
};
use server_api::web3::{Web3Api, Web3ContractApi, Web3TransactionApi};
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
                "验证钱包所有权",
            ),
            RouteInfo::new(
                &format!("{}/wallet/list", base_path),
                Method::GET,
                "Web3Api",
                "获取钱包列表",
            ),
            RouteInfo::new(
                &format!("{}/wallet/:id", base_path),
                Method::DELETE,
                "Web3Api",
                "删除钱包",
            ),
        ];

        // Contract routes
        let contract_routes = vec![
            RouteInfo::new(
                &format!("{}/contract", base_path),
                Method::POST,
                "Web3ContractApi",
                "创建合约",
            ),
            RouteInfo::new(
                &format!("{}/contract/list", base_path),
                Method::GET,
                "Web3ContractApi",
                "获取合约列表",
            ),
            RouteInfo::new(
                &format!("{}/contract/:id", base_path),
                Method::GET,
                "Web3ContractApi",
                "获取合约详情",
            ),
            RouteInfo::new(
                &format!("{}/contract/:id", base_path),
                Method::PUT,
                "Web3ContractApi",
                "更新合约",
            ),
            RouteInfo::new(
                &format!("{}/contract/:id", base_path),
                Method::DELETE,
                "Web3ContractApi",
                "删除合约",
            ),
            RouteInfo::new(
                &format!("{}/contract/:id/call", base_path),
                Method::POST,
                "Web3ContractApi",
                "调用合约方法",
            ),
        ];

        // Transaction routes
        let transaction_routes = vec![
            RouteInfo::new(
                &format!("{}/transaction/list", base_path),
                Method::GET,
                "Web3TransactionApi",
                "获取交易记录",
            ),
        ];

        // Add all routes
        for route in wallet_routes.into_iter()
            .chain(contract_routes.into_iter())
            .chain(transaction_routes.into_iter()) 
        {
            add_route(route).await;
        }

        let router = Router::new()
            // Wallet routes
            .route("/wallet/verify", post(Web3Api::verify_wallet))
            .route("/wallet/list", get(Web3Api::list_wallets))
            .route("/wallet/:id", delete(Web3Api::delete_wallet))
            // Contract routes
            .route("/contract", post(Web3ContractApi::create_contract))
            .route("/contract/list", get(Web3ContractApi::list_contracts))
            .route("/contract/:id", get(Web3ContractApi::get_contract))
            .route("/contract/:id", put(Web3ContractApi::update_contract))
            .route("/contract/:id", delete(Web3ContractApi::delete_contract))
            .route("/contract/:id/call", post(Web3ContractApi::call_contract))
            // Transaction routes
            .route("/transaction/list", get(Web3TransactionApi::list_transactions));

        router
    }
}
