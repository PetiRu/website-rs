# website-rs

<div align="center">

# 🛡️ website-rs

### Secure, modular building blocks for modern Rust web services

[![CI](https://github.com/PetiRu/website-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/PetiRu/website-rs/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Security](https://img.shields.io/badge/security-first-success.svg)](SECURITY.md)

**Encryption · APIs · Routing · Caching · Protection · Observability**

</div>

---

## ✨ What is website-rs?

`website-rs` is a framework-agnostic Rust workspace for building safer APIs, websites, and backend services. It provides focused, composable primitives instead of hiding important behavior inside a large framework.

Use the crates independently or combine them into a request pipeline that is easy to inspect, test, and evolve.

> **Project status:** Early-stage foundation. Public APIs may change before 1.0. This project complements—not replaces—TLS, secret management, framework security controls, and professional security review.

## 📚 Contents

- [Features](#-features)
- [Architecture](#-architecture)
- [Quick start](#-quick-start)
- [Encryption example](#-encryption-example)
- [Request pipeline](#-recommended-request-pipeline)
- [Security principles](#-security-principles)
- [Documentation](#-documentation)
- [Development](#-development)
- [Roadmap](#-roadmap)
- [License](#-license)

## 🚀 Features

| Crate | Responsibility |
| --- | --- |
| `website-encryption` | AES-256-GCM authenticated encryption with associated data |
| `website-api` | Framework-neutral requests, responses, headers, and body limits |
| `website-cache` | Bounded in-memory cache with TTL support and a simple trait |
| `website-router` | Explicit route resolution and stable cache-key generation |
| `website-protection` | Security headers, exact origin checks, and rate limiting |
| `website-config` | Typed environment-based service configuration |
| `website-observability` | Consistent tracing initialization |

### Design goals

- **Secure by default:** authenticated encryption, bounded resources, and explicit failure paths.
- **Small and composable:** use only the crates your service needs.
- **Framework-neutral:** integrate with Axum, Actix Web, Warp, or a custom HTTP layer.
- **Easy to audit:** clear boundaries, focused APIs, and rejection-path tests.
- **Operationally aware:** configuration, tracing, caching, and security guidance belong together.

## 🧭 Architecture

```text
                    ┌──────────────────────────┐
                    │   HTTP framework/edge    │
                    └────────────┬─────────────┘
                                 │
                    ┌────────────▼─────────────┐
                    │ website-api              │  validate + limit
                    └────────────┬─────────────┘
                                 │
             ┌───────────────────▼───────────────────┐
             │ website-protection                    │
             │ headers · origins · rate limits       │
             └───────────────────┬───────────────────┘
                                 │
                    ┌────────────▼─────────────┐
                    │ website-router           │  resolve + key
                    └───────┬─────────┬────────┘
                            │         │
                 ┌──────────▼───┐ ┌──▼─────────────┐
                 │ website-cache │ │ application    │
                 │ safe TTL data │ │ business logic │
                 └──────────────┘ └──────┬─────────┘
                                         │
                              ┌──────────▼──────────┐
                              │ website-encryption  │
                              │ sensitive payloads  │
                              └─────────────────────┘
```

See the [architecture guide](docs/ARCHITECTURE.md) for boundaries and integration guidance.

## ⚡ Quick start

```bash
git clone https://github.com/PetiRu/website-rs.git
cd website-rs
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

Expected example output includes an encrypted payload length and its safely decrypted demonstration value.

## 🔐 Encryption example

```rust
use website_encryption::{Key, Sealer};

fn main() -> Result<(), website_encryption::Error> {
    // Demonstration key only. Load production keys from a secret manager.
    let sealer = Sealer::new(Key::from_bytes([42u8; 32]));
    let context = b"session:user-42:v1";

    let token = sealer.seal(b"sensitive website data", context)?;
    let plaintext = sealer.open(&token, context)?;

    assert_eq!(plaintext, b"sensitive website data");
    Ok(())
}
```

The encrypted format is `nonce || ciphertext || authentication tag`. Never reuse a key across unrelated trust domains without a deliberate key-management design, and never commit real keys.

## 🔄 Recommended request pipeline

1. Parse the request at the HTTP boundary.
2. Enforce method, framing, and body-size limits.
3. Apply origin policy, security headers, and rate limiting.
4. Authenticate and authorize the caller.
5. Resolve the route.
6. Read or write the cache only when the data is safe for that cache scope.
7. Encrypt sensitive values with purpose-specific associated data.
8. Emit structured telemetry without credentials, tokens, keys, or raw sensitive bodies.

## 🛡️ Security principles

- **Authenticated encryption:** confidentiality without sacrificing integrity.
- **No custom cryptography:** established Rust crypto crates provide the primitive.
- **Bounded resources:** body limits and bounded cache capacity help control abuse.
- **Explicit cache policy:** private or secret-bearing responses must not be cached by default.
- **Defense in depth:** use TLS, secure headers, input validation, authorization, and secret management together.
- **Fail closed:** lock failures and authentication failures do not silently grant access.

Read [SECURITY.md](SECURITY.md) before using the project in production.

## 🗂️ Repository layout

```text
crates/                 reusable Rust modules
examples/               runnable integration examples
docs/                   architecture and localized documentation
website-protection/     module overview and security notes
cache/                  cache overview and usage notes
router/                 routing overview and boundaries
.github/workflows/      continuous integration
```

## 🌍 Documentation

- [Documentation index](docs/README.md)
- [Architecture guide](docs/ARCHITECTURE.md)
- [Security policy](SECURITY.md)
- [Contributing guide](CONTRIBUTING.md)

### Read in your language

[English](README.md) · [Magyar / Hungarian](docs/README.hu.md) · [Español](docs/README.es.md) · [Français](docs/README.fr.md) · [Deutsch](docs/README.de.md) · [Português](docs/README.pt.md) · [日本語](docs/README.ja.md) · [한국어](docs/README.ko.md) · [العربية](docs/README.ar.md) · [Русский](docs/README.ru.md) · [简体中文](docs/README.zh.md)

## 🧪 Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Please add tests for new security-sensitive behavior and do not include credentials in issues, commits, logs, or examples.

## 🗺️ Roadmap

- [ ] Add Axum and Actix integration examples
- [ ] Add a production-oriented session abstraction
- [ ] Add configurable cache eviction policies
- [ ] Add broader API contract validation helpers
- [ ] Review public APIs before the 1.0 release

Ideas and focused improvements are welcome through issues and pull requests.

## 📄 License

Released under the [MIT License](LICENSE).
