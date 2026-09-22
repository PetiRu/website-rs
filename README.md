# website-rs

Secure Rust building blocks for modern web apps, APIs, and protected services.

<div align="center">
  <img src="https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust" alt="Rust 1.75+" />
  <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="MIT License" />
  <img src="https://img.shields.io/badge/Status-Security%20Focused-success.svg" alt="Security Focused" />
  <img src="https://img.shields.io/badge/Workspace-Modular-8A2BE2.svg" alt="Modular Workspace" />
</div>

`website-rs` is a modular Rust workspace for building safer web services. It combines the most common pieces needed in real-world API and website infrastructure:

- authenticated encryption for sensitive data
- request validation and safe API handling
- route-based dispatch and cache-aware logic
- website protection policies and HTTP hardening
- typed configuration and observability
- clean separation of concerns for easy framework integration

## Overview

This project is designed for teams that want:

- strong defaults without a giant monolith
- reusable Rust building blocks
- easy integration with Actix, Axum, or custom server layers
- secure patterns for API construction and web protection

## Project structure

```text
website-rs/
├── crates/
│   ├── encryption/
│   ├── api/
│   ├── cache/
│   ├── router/
│   ├── website-protection/
│   ├── config/
│   └── observability/
├── examples/
│   └── secure-service/
├── docs/
│   ├── README.md
│   ├── ARCHITECTURE.md
│   ├── README.hu.md
│   ├── README.es.md
│   ├── README.fr.md
│   ├── README.de.md
│   ├── README.pt.md
│   ├── README.ja.md
│   ├── README.ko.md
│   ├── README.ar.md
│   ├── README.ru.md
│   └── README.zh.md
├── website-protection/
│   └── README.md
├── cache/
│   └── README.md
├── router/
│   └── README.md
├── .github/
│   ├── workflows/
│   └── dependabot.yml
├── Cargo.toml
├── LICENSE
├── SECURITY.md
├── CONTRIBUTING.md
├── rustfmt.toml
├── README.md
├── .gitignore
└── docs/README.md
```

## Security-first design

The workspace is structured around small, auditable building blocks instead of a single giant server abstraction.

Core principles:

- protect sensitive data with authenticated encryption
- enforce request and payload limits early
- validate origins and apply security headers
- limit abuse with rate limiting
- cache only what is safe to cache
- log behavior, not secrets
- use TLS and a secret manager in production

## Key modules

| Module | Purpose |
| --- | --- |
| `website-encryption` | AES-256-GCM encryption and authenticated payload handling |
| `website-api` | request/response types, enforcement, and shared API boundaries |
| `website-cache` | in-memory TTL cache and bounded storage behavior |
| `website-router` | route matching and stable cache key generation |
| `website-protection` | headers, origins, rate limiting, and HTTP safeguards |
| `website-config` | typed environment configuration |
| `website-observability` | tracing and structured logging helpers |

## Quick start

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## Example

```rust
use website_encryption::{Key, Sealer};

fn main() -> Result<(), website_encryption::Error> {
    let key = Key::from_bytes([42u8; 32]);
    let sealer = Sealer::new(key);

    let token = sealer.seal(b"sensitive website data", b"session:user-42")?;
    let plaintext = sealer.open(&token, b"session:user-42")?;

    println!("{}", String::from_utf8_lossy(&plaintext));
    Ok(())
}
```

## Recommended request flow

1. Parse the HTTP request
2. Validate method and request size
3. Enforce security headers and origin rules
4. Apply rate limiting
5. Resolve the routing target
6. Cache only safe responses
7. Encrypt sensitive values with purpose-specific associated data
8. Trace the request without logging secrets

## Documentation

- [Documentation index](docs/README.md)
- [Architecture guide](docs/ARCHITECTURE.md)
- [Contributing guide](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

## Multilingual documentation

- [English](README.md)
- [Hungarian](docs/README.hu.md)
- [Spanish](docs/README.es.md)
- [French](docs/README.fr.md)
- [German](docs/README.de.md)
- [Portuguese](docs/README.pt.md)
- [Japanese](docs/README.ja.md)
- [Korean](docs/README.ko.md)
- [Arabic](docs/README.ar.md)
- [Russian](docs/README.ru.md)
- [Chinese (Simplified)](docs/README.zh.md)

## Development checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Security guidance

- store keys outside the repository
- use TLS/HTTPS in production
- rotate secrets on a controlled schedule
- never trust client-side validation as a security boundary
- avoid logging raw tokens, cookies, passwords, or secrets

## License

MIT. See [LICENSE](LICENSE).
