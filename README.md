# website-rs

[![CI](https://github.com/PetiRu/website-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/PetiRu/website-rs/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A modular, security-first Rust toolkit for web services: authenticated data encryption, typed API requests, cache-aware routing, and practical website protection primitives.

> **Status:** Early-stage foundation. APIs may change. This project is not a replacement for a security review, TLS, a secret manager, or a production-grade reverse proxy.

## Modules

| Crate | Purpose |
| --- | --- |
| `website-encryption` | AES-256-GCM authenticated encryption with explicit key handling |
| `website-api` | Small framework-agnostic request/response types and body limits |
| `website-cache` | Pluggable cache trait and bounded in-memory implementation |
| `website-router` | Deterministic route matching and cache-aware dispatch |
| `website-protection` | Security headers, origin checks, and rate limiting primitives |
| `website-config` | Environment-backed, typed service configuration |
| `website-observability` | Tracing initialization helpers |

## Quick start

```bash
cargo test --workspace
cargo run --example secure-service
```

```rust
use website_encryption::{Key, Sealer};

let key = Key::from_bytes([7u8; 32]);
let sealer = Sealer::new(key);
let token = sealer.seal(b"private payload", b"user:42")?;
let plaintext = sealer.open(&token, b"user:42")?;
assert_eq!(plaintext, b"private payload");
# Ok::<(), website_encryption::Error>(())
```

## Security defaults

- AES-256-GCM provides confidentiality **and integrity**; no custom cryptography is used.
- Nonces are generated with the operating system random source.
- Authentication context (AAD) binds ciphertext to its intended purpose.
- Request bodies have explicit limits and rate limiting uses monotonic time.
- Secrets should come from a secret manager or environment injection, never source control.
- Use HTTPS/TLS at the edge and rotate encryption keys with a documented migration plan.

The encryption crate deliberately does not serialize keys, print secrets, or provide insecure fallback modes. See each crate's documentation for limitations.

## Workspace layout

```text
crates/
├── encryption/          # authenticated encryption
├── api/                 # request and response primitives
├── cache/               # cache abstraction and memory cache
├── router/              # route matching and cache selection
├── website-protection/  # headers, origins, rate limits
├── config/              # typed environment configuration
└── observability/       # structured tracing setup
examples/                # small integration examples
docs/                    # design notes
```

## Contributing

Run `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace` before opening a pull request. Please report security issues privately rather than opening a public issue; see `SECURITY.md`.

## License

MIT. See [LICENSE](LICENSE).
