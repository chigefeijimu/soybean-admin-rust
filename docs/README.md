# Web3 项目文档索引

## 后端文档

- [WEB3.md](./backend/WEB3.md) - Rust 后端 Web3 模块技术文档

## 前端文档

- [WEB3.md](./frontend/WEB3.md) - Vue3 前端 Web3 模块技术文档

## 快速链接

### 后端 (Rust)
- 模块: `server/service/src/web3/`
- API: `server/api/src/web3/`
- 路由: `server/router/src/web3/`
- 数据库模型: `server/model/src/web3/`

### 前端 (Vue3)
- 组件: `frontend/src/components/web3/`
- Hooks: `frontend/src/hooks/`
- 状态: `frontend/src/stores/`
- 常量: `frontend/src/constants/`

### 数据库
- web3_wallet - 钱包表
- web3_contract - 合约表
- web3_transaction - 交易表

### API 端点
- POST /web3/wallet/verify - 验证钱包
- POST /web3/wallet/balance - 获取余额
- GET /web3/wallet/list - 钱包列表
- DELETE /web3/wallet/:id - 删除钱包

- POST /web3/contract - 创建合约
- GET /web3/contract/list - 合约列表
- GET /web3/contract/:id - 合约详情
- PUT /web3/contract/:id - 更新合约
- DELETE /web3/contract/:id - 删除合约
- POST /web3/contract/:id/call - 调用合约

- GET /web3/transaction/list - 交易列表

## 环境变量

```bash
# 后端
ETH_MAINNET_RPC=https://eth.llamarpc.com
ETH_SEPOLIA_RPC=https://sepolia.infura.io/v3/YOUR_KEY
POLYGON_RPC=https://polygon.llamarpc.com

# 前端
VITE_API_BASE_URL=http://localhost:10001
```

## 技术栈

### 后端
- Axum 0.8
- SeaORM 1.1
- Alloy 1.7 (Web3)
- PostgreSQL

### 前端
- Vue 3 + TypeScript
- Vite
- wagmi + viem
- Pinia
- TanStack Query

## 学习资源

- [EIP-191](https://eips.ethereum.org/EIPS/eip-191) - 签名标准
- [Alloy Docs](https://docs.rs/alloy/latest/alloy/)
- [wagmi](https://wagmi.sh/)
- [viem](https://viem.sh/)
