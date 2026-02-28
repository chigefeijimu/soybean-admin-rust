# Soybean Admin Rust - Web3 模块技术文档

## 概述

本文档描述了 soybean-admin-rust 项目中 Web3 功能的实现细节。

## 目录结构

```
server/
├── service/src/web3/
│   ├── mod.rs              # Web3 服务层 (核心业务逻辑)
│   ├── alloy_provider.rs   # 区块链交互模块 (HTTP)
│   ├── alloy_provider_v2.rs # Alloy 官方库实现 (推荐)
│   └── contract_call_impl.rs  # 合约调用实现(预留)
├── api/src/web3/
│   └── mod.rs             # API 处理器
└── router/src/web3/
    └── mod.rs             # 路由定义
```

## 模块说明

### 1. alloy_provider.rs - 区块链交互

**功能**: 通过 HTTP JSON-RPC 与区块链节点通信

#### 主要结构

```rust
// 链配置
pub struct ChainConfig {
    pub chain_id: u64,           // 链 ID
    pub name: String,            // 链名称
    pub rpc_url: String,         // RPC 节点 URL
    pub explorer_url: Option<String>, // 浏览器 URL
}
```

#### 支持的链

| Chain ID | Name | Default RPC |
|----------|------|-------------|
| 1 | Ethereum Mainnet | https://eth.llamarpc.com |
| 11155111 | Sepolia Testnet | https://sepolia.infura.io |
| 137 | Polygon | https://polygon.llamarpc.com |

#### 主要方法

```rust
// 获取钱包余额
pub async fn get_balance(&self, address: &str) -> Result<String, ...>

// 获取nonce
pub async fn get_nonce(&self, address: &str) -> Result<u64, ...>

// 获取Gas价格
pub async fn get_gas_price(&self) -> Result<String, ...>
```

#### ProviderPool - 多链连接池

```rust
pub struct ProviderPool {
    providers: Arc<RwLock<HashMap<u64, Web3Provider>>>,
}

impl ProviderPool {
    // 获取指定链的Provider
    pub async fn get_provider(&self, chain_id: u64) -> Result<Web3Provider, ...>
    
    // 添加自定义链
    pub async fn add_provider(&self, chain_config: ChainConfig) -> Result<(), ...>
}
```

#### 签名验证 (EIP-191)

```rust
pub mod signature {
    // 验证签名
    pub fn verify_eip191(message: &str, signature: &str, expected_address: &str) -> Result<bool, ...>
    
    // 恢复签名者地址
    pub fn recover_signer(message: &str, signature: &str) -> Result<String, ...>
    
    // 生成签名消息
    pub fn generate_sign_message(nonce: &str) -> String
}
```

#### Alloy Provider V2 (官方库实现)

> **推荐**: 使用 `alloy_provider_v2.rs` 获取完整功能

```rust
use crate::web3::alloy_provider_v2::{AlloyProvider, pool::AlloyProviderPool};

// 支持更多链
pub enum ChainInfo {
    ETHEREUM,      // Chain ID: 1
    SEPOLIA,       // Chain ID: 11155111
    POLYGON,       // Chain ID: 137
    ARBITRUM,      // Chain ID: 42161
    OPTIMISM,      // Chain ID: 10
}

// 创建 Provider
let provider = AlloyProvider::new("https://eth.llamarpc.com", 1).await?;

// 获取余额
let balance = provider.get_balance("0x...").await?;

// 获取代币余额 (ERC20)
use crate::web3::alloy_provider_v2::erc20;
let token_balance = erc20::get_token_balance(&provider, token_addr, owner_addr).await?;
```

### 2. mod.rs - 服务层

#### Input Types

```rust
// 钱包验证输入
pub struct WalletVerifyInput {
    pub wallet_address: String,  // 钱包地址
    pub signature: String,       // 签名
    pub message: String,         // 签名消息
    pub wallet_type: Option<String>, // 钱包类型 (默认: metamask)
    pub chain_id: Option<i32>,    // 链 ID (默认: 1)
}

// 钱包余额查询
pub struct WalletBalanceInput {
    pub address: String,
    pub chain_id: Option<i32>,
}

// 合约创建
pub struct ContractCreateInput {
    pub name: String,
    pub contract_address: String,
    pub chain_id: i32,
    pub abi: Option<String>,
    pub description: Option<String>,
}

// 合约调用
pub struct ContractCallInput {
    pub contract_id: String,
    pub method_name: String,
    pub params: Option<String>,
    pub from_address: Option<String>,
    pub value: Option<String>,
}
```

#### Output Types

```rust
pub struct WalletInfo {
    pub id: String,
    pub wallet_address: String,
    pub wallet_type: String,
    pub chain_id: i32,
}

pub struct WalletBalance {
    pub address: String,
    pub balance: String,        // wei 单位的余额
    pub chain_id: u64,
    pub chain_name: String,
}

pub struct ContractInfo {
    pub id: String,
    pub name: String,
    pub contract_address: String,
    pub chain_id: i32,
    pub abi: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}

pub struct ContractCallOutput {
    pub success: bool,
    pub tx_hash: Option<String>,
    pub result: Option<String>,
    pub error: Option<String>,
}
```

#### Service Traits

```rust
// 钱包服务
#[async_trait]
pub trait TWalletService {
    async fn verify_wallet(&self, input: WalletVerifyInput) -> Result<WalletInfo, ServiceError>;
    async fn list_wallets(&self, input: WalletListInput) -> Result<Vec<WalletInfo>, ServiceError>;
    async fn delete_wallet(&self, id: &str) -> Result<(), ServiceError>;
    async fn get_balance(&self, address: &str, chain_id: i32) -> Result<WalletBalance, ServiceError>;
}

// 合约服务
#[async_trait]
pub trait TContractService {
    async fn create_contract(&self, input: ContractCreateInput) -> Result<ContractInfo, ServiceError>;
    async fn list_contracts(&self) -> Result<Vec<ContractInfo>, ServiceError>;
    async fn get_contract(&self, id: &str) -> Result<ContractInfo, ServiceError>;
    async fn update_contract(&self, input: ContractUpdateInput) -> Result<ContractInfo, ServiceError>;
    async fn delete_contract(&self, id: &str) -> Result<(), ServiceError>;
    async fn call_contract(&self, input: ContractCallInput) -> Result<ContractCallOutput, ServiceError>;
}

// 交易服务
#[async_trait]
pub trait TTransactionService {
    async fn list_transactions(&self, user_id: Option<String>) -> Result<Vec<TransactionInfo>, ServiceError>;
}
```

### 3. API 端点

#### 钱包相关

| Method | Path | Description |
|--------|------|-------------|
| POST | /web3/wallet/verify | 验证并保存钱包 |
| GET | /web3/wallet/list | 获取钱包列表 |
| DELETE | /web3/wallet/:id | 删除钱包 |
| POST | /web3/wallet/balance | 获取钱包余额 |

#### 合约相关

| Method | Path | Description |
|--------|------|-------------|
| POST | /web3/contract | 创建合约记录 |
| GET | /web3/contract/list | 获取合约列表 |
| GET | /web3/contract/:id | 获取合约详情 |
| PUT | /web3/contract/:id | 更新合约 |
| DELETE | /web3/contract/:id | 删除合约 |
| POST | /web3/contract/:id/call | 调用合约方法 |

#### 交易相关

| Method | Path | Description |
|--------|------|-------------|
| GET | /web3/transaction/list | 获取交易记录 |

### 4. 数据库模型

#### web3_wallet

| Field | Type | Description |
|-------|------|-------------|
| id | String (ULID) | 主键 |
| user_id | Option<String> | 用户ID |
| wallet_address | String | 钱包地址 |
| wallet_type | String | 钱包类型 |
| chain_id | i32 | 链ID |
| signature | Option<String> | 签名 |
| message | Option<String> | 签名消息 |
| created_at | NaiveDateTime | 创建时间 |
| updated_at | Option<NaiveDateTime> | 更新时间 |

#### web3_contract

| Field | Type | Description |
|-------|------|-------------|
| id | String (ULID) | 主键 |
| name | String | 合约名称 |
| contract_address | String | 合约地址 |
| chain_id | i32 | 链ID |
| abi | Option<String> | ABI JSON |
| description | Option<String> | 描述 |
| created_by | Option<String> | 创建者 |
| created_at | NaiveDateTime | 创建时间 |
| updated_at | Option<NaiveDateTime> | 更新时间 |

#### web3_transaction

| Field | Type | Description |
|-------|------|-------------|
| id | String (ULID) | 主键 |
| user_id | Option<String> | 用户ID |
| contract_id | Option<String> | 合约ID |
| method_name | String | 方法名 |
| params | Option<String> | 参数JSON |
| tx_hash | Option<String> | 交易哈希 |
| status | String | 状态 (pending/completed/failed) |
| from_address | Option<String> | 发送者地址 |
| error_message | Option<String> | 错误信息 |
| created_at | NaiveDateTime | 创建时间 |
| updated_at | Option<NaiveDateTime> | 更新时间 |

## 环境变量

```bash
# 区块链 RPC URLs (可选)
ETH_MAINNET_RPC=https://eth.llamarpc.com
ETH_SEPOLIA_RPC=https://sepolia.infura.io/v3/YOUR_KEY
POLYGON_RPC=https://polygon.llamarpc.com
```

## 使用示例

### 1. 验证钱包

```bash
curl -X POST http://localhost:10001/web3/wallet/verify \
  -H "Content-Type: application/json" \
  -d '{
    "walletAddress": "0x1234...",
    "signature": "0xabcd...",
    "message": "Welcome to Soybean Admin!...",
    "walletType": "metamask",
    "chainId": 1
  }'
```

### 2. 获取余额

```bash
curl -X POST http://localhost:10001/web3/wallet/balance \
  -H "Content-Type: application/json" \
  -d '{
    "address": "0x1234...",
    "chainId": 1
  }'
```

### 3. 创建合约记录

```bash
curl -X POST http://localhost:10001/web3/contract \
  -H "Content-Type: application/json" \
  -d '{
    "name": "USDC",
    "contractAddress": "0xA0b86a33E6441C4C4C4C4C4C4C4C4C4C4C4C4C",
    "chainId": 1,
    "abi": "[...]",
    "description": "USD Coin"
  }'
```

## 注意事项

1. **签名验证**: 当前实现为简化版本,完整的 ecrecover 需要密码学库
2. **合约调用**: 当前只记录交易,实际签名需前端配合
3. **RPC 节点**: 公开节点可能有速率限制,建议使用自己的节点
4. **安全性**: 私钥不存储在后端,所有签名操作在前端完成

## 依赖项

```toml
# Cargo.toml
alloy = { version = "1.7", features = ["rpc-client", "signers", "sol-types", "consensus"] }
alloy-primitives = "0.8"
reqwest = { version = "0.12", features = ["json"] }
serde_json = "1.0"
hex = "0.4"
lazy_static = "1.4"
```

## 更新日志

### 2026-02-28
- 初始版本
- 添加多链支持
- 添加钱包余额查询
- 添加 EIP-191 签名验证
