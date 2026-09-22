# website-rs

A secure, modular, and production-minded Rust workspace for modern web services.

<div align="center">
  <img src="https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust" alt="Rust 1.75+" />
  <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="MIT License" />
  <img src="https://img.shields.io/badge/Status-Active-success.svg" alt="Active" />
</div>

`website-rs` brings together the building blocks most web services need but rarely want to reimplement from scratch:

- authenticated encryption for sensitive data
- request validation and body-size enforcement
- deterministic route resolution
- cache-aware response handling
- HTTP hardening primitives
- typed configuration and tracing

## Why this project exists

Modern web infrastructure needs more than a framework. It needs safe defaults, modular boundaries, and clear security practices.

This workspace is intentionally small and composable so you can plug it into Actix, Axum, Warp, or any other Rust server stack without locking into a single framework.

## Architecture at a glance

```text
website-rs/
├── crates/
│   ├── encryption/          # AES-256-GCM authenticated encryption
│   ├── api/                 # request/response primitives and limits
│   ├── cache/               # TTL cache and bounded in-memory store
│   ├── router/              # route matching and cache keys
│   ├── website-protection/  # security headers, origin checks, rate limiting
│   ├── config/             # environment-driven configuration
│   └── observability/       # tracing and logging helpers
├── examples/
│   └── secure-service/     # example application
├── docs/
│   ├── README.md           # documentation index
│   ├── ARCHITECTURE.md     # design guide
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
├── .github/
│   ├── workflows/
│   └── dependabot.yml
├── Cargo.toml
├── LICENSE
├── SECURITY.md
├── CONTRIBUTING.md
├── rustfmt.toml
├── README.md
└── .gitignore
```

## Feature highlights

### Security-first foundation

- AES-256-GCM encryption with associated data
- explicit key handling with no unsafe magic defaults
- request limit enforcement to control abuse
- hardened HTTP headers and origin checks
- rate limiting for abuse and brute-force protection

### Web-ready primitives

- clean request and response models
- exact route matching and cache-key generation
- TTL-based in-memory cache
- observability hooks for tracing and logs

### Modular design

The workspace is intentionally split into independent crates so each layer can evolve, test, and reuse without creating a monolithic server implementation.

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

## Recommended usage flow

1. Parse the incoming HTTP request.
2. Enforce request-size limits.
3. Validate origin and method.
4. Apply security headers and rate limiting.
5. Resolve the route and authorize the request.
6. Use the cache only for safe responses.
7. Encrypt sensitive values with contextual associated data.
8. Trace the request and never log secrets.

## Documentation

- [Documentation index](docs/README.md)
- [Architecture guide](docs/ARCHITECTURE.md)
- [Contribution guide](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

## Multilingual documentation

This project includes readable documentation in multiple languages:

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

- store secrets in a secret manager, not in source control
- use TLS/HTTPS in production
- rotate encryption keys on a schedule
- avoid trusting client-side validation as a security boundary
- never log raw credentials, tokens, or plaintext secrets

## License

MIT. See [LICENSE](LICENSE).
