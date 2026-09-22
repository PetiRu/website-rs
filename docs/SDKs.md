# Cross-language packages

The Rust crates are the reference implementation. The companion packages make the same adoption path available to existing Python, JavaScript, and TypeScript sites.

| Stack | Location | Typical integration |
| --- | --- | --- |
| Rust | `crates/` | native crate dependencies |
| Python | `packages/python/` | Flask, FastAPI, Django, ASGI/WSGI |
| JavaScript | `packages/javascript/` | Node.js, Express, Fastify |
| TypeScript | `packages/typescript/` | typed Node.js and full-stack applications |

These packages are not automatically wire-compatible for encryption. Do not exchange ciphertext across languages until an explicit format, key derivation, nonce policy, and interoperability test suite are defined.
