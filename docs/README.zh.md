# website-rs

一个面向安全的模块化 Rust 工作区，用于现代 Web 服务和 API 后端。

## 概览

`website-rs` 将安全 Web 应用中常见的基础能力整合在一起：

- 认证加密
- 请求校验与大小限制
- 带缓存的路由
- Web 安全防护
- 类型化配置
- 结构化可观测性

## 核心模块

- `website-encryption`: AES-256-GCM 加密
- `website-api`: 请求/响应原语
- `website-cache`: TTL 内存缓存
- `website-router`: 路由解析和缓存键
- `website-protection`: 安全头、来源校验、限流
- `website-config`: 环境变量配置
- `website-observability`: 日志和 tracing

## 快速开始

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## 安全原则

- 密钥不要存进 Git
- 生产环境必须使用 HTTPS/TLS
- 通过受控方式轮换和迁移密钥
- 客户端校验不是安全边界

## 链接

- [主 README](../README.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- [SECURITY](../SECURITY.md)
