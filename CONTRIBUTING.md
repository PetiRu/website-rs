# Local development

## Checks

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Example

```bash
cargo run --manifest-path examples/secure-service/Cargo.toml
```

Do not commit `.env` files or real encryption keys. Use test-only keys locally and rotate any credential that is accidentally exposed.
