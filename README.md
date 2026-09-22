# 🌐 website-rs

<div align="center">

# 🛡️ website-rs

### Security-first building blocks for websites, APIs, and web services

[![CI](https://github.com/PetiRu/website-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/PetiRu/website-rs/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.10%2B-3776AB?logo=python&logoColor=white)](packages/python/README.md)
[![JavaScript](https://img.shields.io/badge/JavaScript-Web%20Crypto-F7DF1E?logo=javascript&logoColor=black)](packages/javascript/README.md)
[![TypeScript](https://img.shields.io/badge/TypeScript-5%2B-3178C6?logo=typescript&logoColor=white)](packages/typescript/README.md)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**Encryption · API limits · Routing · Caching · Protection · CAPTCHA · Observability**

</div>

---

## What is website-rs?

`website-rs` is a framework-agnostic security toolkit for new and existing websites. The core implementation is Rust, with lightweight companion packages for Python, JavaScript, and TypeScript applications.

Use one module, or compose a complete request pipeline. Nothing requires Axum, Actix, Django, Flask, Express, Fastify, or a particular runtime.

> **Status:** Early-stage foundation. APIs may change before 1.0. These packages complement—not replace—TLS, secret management, authorization, framework protections, or a professional security review.

## Highlights

- 🔐 **Authenticated encryption** with AES-256-GCM in Rust and Web Crypto in JavaScript/TypeScript
- 🧱 **Bounded inputs** with explicit request body limits
- 🛡️ **Web hardening** with security headers, origin checks, and rate limiting
- 🧩 **Pluggable CAPTCHA** with one-time custom challenges and a store interface
- ⚡ **Cache-aware routing** with deterministic cache keys
- 🔌 **Easy adoption** through standalone crates and language packages
- 📚 **Readable docs** in 15 languages, including Hungarian

## Architecture

```text
HTTP framework / serverless edge
              │
              ▼
       Request validation ── body limits, method checks
              │
              ▼
       Website protection ── headers, origins, rate limits, CAPTCHA
              │
              ▼
       Router + cache ────── dispatch and safe response caching
              │
              ▼
       Application logic
              │
              ▼
       Encryption + observability
```

## Packages by language

| Language | Package | Best for |
| --- | --- | --- |
| Rust | `website-encryption`, `website-api`, `website-protection`, `website-captcha` | high-performance services and shared infrastructure |
| Python | `website-rs-security` | Flask, Django, FastAPI, and Python APIs |
| JavaScript | `@websafers/security` | Node.js, Express, Fastify, and browser-compatible utilities |
| TypeScript | `@websafers/security` | typed Node.js and full-stack TypeScript applications |

The language packages provide compatible integration ideas, not automatic wire compatibility. Read each package's security notes before production use.

## Quick start

### Rust workspace

```bash
git clone https://github.com/PetiRu/website-rs.git
cd website-rs
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

### Python

```bash
pip install -e packages/python
```

```python
from website_rs_security import MemoryCaptcha, security_headers

captcha = MemoryCaptcha()
challenge, answer_for_demo = captcha.issue_math()
assert captcha.verify(challenge.token, answer_for_demo)
headers = security_headers()
```

### JavaScript

```bash
cd packages/javascript
npm install
npm test
```

### TypeScript

```bash
cd packages/typescript
npm install
npm test
```

## Rust encryption example

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

## Custom CAPTCHA

`website-captcha` is deliberately a primitive rather than a misleading one-size-fits-all CAPTCHA product. It supports one-time challenges and a `ChallengeStore` trait so applications can use Redis, a database, or another bounded shared store.

```rust
use std::time::Duration;
use website_captcha::{ChallengeStore, MemoryCaptcha};

let captcha = MemoryCaptcha::new();
let (challenge, answer) = captcha.issue_math(Duration::from_secs(120));
captcha.verify(&challenge.token, &answer)?;
# Ok::<(), website_captcha::Error>(())
```

Do not expose the answer to a browser in production. Combine CAPTCHA with rate limiting, CSRF protection where applicable, telemetry, and server-side validation. It must not be the only control for login, payments, authorization, or account recovery.

## Recommended request pipeline

1. Parse at the HTTP boundary.
2. Enforce method, framing, and body-size limits.
3. Apply security headers and exact origin policy.
4. Apply rate limiting and, where appropriate, CAPTCHA.
5. Authenticate and authorize the request.
6. Resolve the route.
7. Cache only data safe for that cache scope.
8. Encrypt sensitive values with purpose-specific associated data.
9. Log metadata without credentials, tokens, keys, or raw sensitive bodies.

## Repository layout

```text
crates/                 Rust modules
packages/python/        Python integration package
packages/javascript/    JavaScript Web Crypto package
packages/typescript/    TypeScript typed package
examples/               runnable examples
docs/                   architecture, integration, and translations
```

## Documentation

- [Documentation index](docs/README.md)
- [Architecture guide](docs/ARCHITECTURE.md)
- [Integration guide](docs/INTEGRATION.md)
- [CAPTCHA security notes](docs/CAPTCHA.md)
- [Security policy](SECURITY.md)
- [Contributing guide](CONTRIBUTING.md)

### Languages

[English](README.md) · [Magyar](docs/README.hu.md) · [Español](docs/README.es.md) · [Français](docs/README.fr.md) · [Deutsch](docs/README.de.md) · [Português](docs/README.pt.md) · [Italiano](docs/README.it.md) · [Türkçe](docs/README.tr.md) · [Polski](docs/README.pl.md) · [Українська](docs/README.uk.md) · [Nederlands](docs/README.nl.md) · [日本語](docs/README.ja.md) · [한국어](docs/README.ko.md) · [العربية](docs/README.ar.md) · [Русский](docs/README.ru.md) · [简体中文](docs/README.zh.md)

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Language-package checks are documented in their package READMEs. Never commit real credentials, keys, tokens, or CAPTCHA answers.

## Roadmap

- [ ] Framework adapters for Axum, Actix, Flask, FastAPI, Express, and Fastify
- [ ] Shared cache-store adapters
- [ ] Signed-token and CSRF modules after threat-model review
- [ ] Cross-language interoperability fixtures
- [ ] Public API review before 1.0

## License

MIT. See [LICENSE](LICENSE).
