# website-rs

A secure, modular Rust workspace for modern web services with:

- authenticated encryption
- request validation and safe API handling
- cache-aware routing
- website protection primitives
- structured observability
- security-first defaults

[![CI](https://github.com/PetiRu/website-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/PetiRu/website-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Why website-rs?

`website-rs` is designed for teams building backend services, APIs, or web-facing apps where security, maintainability, and modularity matter. It brings together the foundation blocks you usually have to assemble manually:

- encrypted data handling
- request validation and limits
- route dispatch and response caching
- security policy helpers
- typed configuration
- easy integration with any Rust web framework

## Project overview

```text
website-rs
├── crates/
│   ├── encryption/          # AES-256-GCM secure payload handling
│   ├── api/                 # request/response primitives
│   ├── cache/               # memory cache and TTL logic
│   ├── router/              # route matching and cache key generation
│   ├── website-protection/  # headers, origin checks, rate limiting
│   ├── config/             # environment driven configuration
│   └── observability/      # tracing setup
├── examples/
│   └── secure-service/     # secure example app
├── docs/                   # docs and language translations
├── .github/                # CI and dependency automation
├── Cargo.toml              # workspace root
├── README.md               # project overview
├── LICENSE                 # MIT license
├── SECURITY.md            # security disclosure policy
├── CONTRIBUTING.md         # development guide
└── rustfmt.toml           # formatting policy
```

## Core modules

| Crate | Purpose |
| --- | --- |
| `website-encryption` | Authenticated encryption using AES-256-GCM |
| `website-api` | Request and response building blocks with limits |
| `website-cache` | TTL-based cache abstraction and bounded memory cache |
| `website-router` | Route matching and deterministic cache keys |
| `website-protection` | Security headers, origin checks, and rate limiting |
| `website-config` | Typed environment configuration |
| `website-observability` | Logging and tracing initialization |

## Security-first design

This project intentionally favors small, auditable building blocks instead of one giant server framework.

Recommended architecture:

1. Accept requests at the network boundary.
2. Validate and cap request sizes.
3. Apply origin, header, and rate-limit checks.
4. Resolve the route.
5. Cache only responses that are safe to cache.
6. Encrypt sensitive data with explicit associated data.
7. Log metadata only; never log keys, credentials, or raw secrets.

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

## Quick start

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Language documentation

English is the default. The project documentation is also provided in these languages:

- [Hungarian](docs/README.hu.md)
- [Spanish](docs/README.es.md)
- [French](docs/README.fr.md)
- [German](docs/README.de.md)
- [Portuguese](docs/README.pt.md)
- [Japanese](docs/README.ja.md)
- [Korean](docs/README.ko.md)
- [Arabic](docs/README.ar.md)
- [Chinese (Simplified)](docs/README.zh.md)
- [Russian](docs/README.ru.md)

## Security notes

- Keep encryption keys outside version control.
- Use TLS everywhere in production.
- Rotate keys with a controlled migration process.
- Never trust client-side validation as a security boundary.
- Prefer a real secret manager for production secrets.

## Roadmap

- add framework adapters for Actix and Axum
- add example middleware stacks
- add a secure session abstraction
- add API contract validation helpers
- add more production-grade security utilities

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md) before making changes.

## License

MIT. See [LICENSE](LICENSE).
