# Soybean Admin Rust - Web3 Edition

基于 [soybean-admin-rust](https://github.com/chigefeijimu/soybean-admin-rust) 添加了 Web3 功能。

## 功能特性

### Web3 模块
- ✅ 钱包连接与管理
- ✅ 智能合约管理 (CRUD)
- ✅ 合约交互 (读写)
- ✅ 交易记录
- ✅ NFT 支持 (ERC721/ERC1155)
- ✅ DeFi 示例 (Uniswap, Aave)
- ✅ 区块监听示例

### 技术栈
- **后端**: Rust, Axum, SeaORM, PostgreSQL
- **前端**: Vue3, TypeScript, Wagmi, Viem
- **数据库**: PostgreSQL 18.x
- **缓存**: Redis (Valkey)

## 快速开始

### 1. 安装依赖

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# 安装 PostgreSQL
dnf install -y postgresql-server
postgresql-setup --initdb
systemctl start postgresql

# 安装 Redis
dnf install -y redis
systemctl start redis
```

### 2. 配置数据库

```bash
# 创建数据库
sudo -u postgres psql -c "CREATE DATABASE soybean_admin;"
```

### 3. 配置环境变量

```bash
cp .env.example .env
# 编辑 .env 文件，配置数据库连接
```

### 4. 运行迁移

```bash
cargo run --bin migration
```

### 5. 启动服务

```bash
# 开发模式
cargo run

# 或构建发布版本
cargo build --release
./target/release/server
```

## API 端点

### Web3 API
| 方法 | 路径 | 描述 |
|------|------|------|
| POST | /api/web3/wallet/verify | 验证钱包 |
| GET | /api/web3/wallet/list | 钱包列表 |
| DELETE | /api/web3/wallet/:id | 删除钱包 |
| POST | /api/web3/contract | 创建合约 |
| GET | /api/web3/contract/list | 合约列表 |
| GET | /api/web3/contract/:id | 合约详情 |
| PUT | /api/web3/contract/:id | 更新合约 |
| DELETE | /api/web3/contract/:id | 删除合约 |
| POST | /api/web3/contract/:id/call | 调用合约 |
| GET | /api/web3/transaction/list | 交易记录 |

## 前端运行

```bash
cd frontend
pnpm install
pnpm dev
```

访问 http://localhost:5173

## 示例代码

查看 `examples/` 目录：
- `alloy_example.rs` - 基础 Alloy 用法
- `alloy_complete.rs` - 完整合约交互
- `defi_examples.rs` - DeFi 合约示例
- `block_listener.rs` - 区块监听示例

## 技术文档

- [Alloy 文档](https://docs.rs/alloy/latest/alloy/)
- [Wagmi 文档](https://wagmi.sh/)
- [SeaORM 文档](https://www.sea-ql.org/SeaORM/)

## License

MIT
