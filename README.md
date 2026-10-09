# Kite x402 服务模板 (Rust + Axum)

将任意 HTTP API 包装为基于 x402 协议的付费服务，结算在 Kite 链上。

行为与官方 [Go/Gin](https://github.com/gokite-ai/kite-x402-services/tree/main/templates/go-gin) 和 [TypeScript/Express](https://github.com/gokite-ai/kite-x402-services/tree/main/templates/typescript-express) 模板完全一致。

## 快速开始

### 前提

- Rust 1.93+
- [kpass](https://docs.gokite.ai/) CLI（可选，用于获取钱包地址）

### 1. 克隆并配置

```bash
git clone https://github.com/qiaopengjun5162/kite-rust-axum-template.git
cd kite-rust-axum-template
cp .env.example .env
```

编辑 `.env`：

| 变量 | 说明 | 示例 |
| --- | --- | --- |
| `PAY_TO` | 收款钱包地址（必填） | `0xYourKiteWalletAddress` |
| `KITE_NETWORK` | 网络 | `mainnet` 或 `testnet` |
| `UPSTREAM_URL` | 要包装的上游 API（必填） | `https://api.open-meteo.com` |
| `PRICE_USD` | 每次调用价格（USD） | `0.001` |
| `SERVICE_DESCRIPTION` | 服务描述 | `Open-Meteo forecast` |
| `PORT` | 监听端口（默认 8080） | `8080` |

### 2. 运行

```bash
cargo run
```

### 3. 测试

```bash
# 健康检查（免费，不需要支付）
curl http://localhost:8080/healthz

# 未支付请求 → 402 Payment Required
curl -v http://localhost:8080/v1/forecast?latitude=52.52&longitude=13.41
```

## 架构

```
客户端                   模板服务                     上游API
  │                       │                          │
  │── GET /v1/forecast ──→│                          │
  │←── 402 Payment ──────│                          │
  │    Required           │                          │
  │                       │                          │
  │── GET /v1/forecast ──→│── 验证 PAYMENT-SIGNATURE→│  facilitator
  │    + PAYMENT-SIGNATURE│                          │
  │                       │── 转发请求 ──────────────→│
  │                       │←── 响应数据 ─────────────│
  │←── 200 OK + 数据 ────│                          │
  │                       │  结算付款（上游成功时）    │
```

## 模板结构

```
src/
├── main.rs      # 入口：路由、x402 中间件、服务器启动
├── kite.rs      # Kite 链配置（mainnet/testnet，价格标签）
├── proxy.rs     # 反向代理：转发请求到上游 API
└── env.rs       # 环境变量帮助函数
```

## 与官方模板的差异

| 特性 | Go/Gin | TS/Express | Rust/Axum (本模板) |
|------|--------|------------|-------------------|
| 框架 | Gin | Express | Axum |
| x402 SDK | `coinbase/x402/go` | `@x402/express` | `x402-axum` |
| 异步 | goroutine | async/await | tokio |
| 包管理器 | go.mod | npm | cargo/Cargo.toml |
| 静态类型 | 强类型 | TypeScript | 强类型 |
| 链配置 | `KiteChainByName()` | `kiteChainByName()` | `kite_chain_by_name()` |
| 反向代理 | `httputil.ReverseProxy` | `fetch()` | `reqwest` |
| 健康检查 | `/healthz` | `/healthz` | `/healthz` |
| 付费路由 | `/v1/*` | `/v1/*` | `/v1/*` |
| 上游不可用 | 502 Bad Gateway | 502 Bad Gateway | 502 Bad Gateway |
| 结算时机 | 上游成功时 | 上游成功时 | 上游成功时 |

## License

MIT
