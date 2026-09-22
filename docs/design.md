# Design notes

## Boundaries

`website-rs` provides reusable primitives rather than a full HTTP server. Framework adapters should translate their request types into `website-api` types and apply protection before business logic.

## Key management

Keys are supplied by the application. Production deployments should load them from a secret manager, use key identifiers for rotation, and retain old keys only for controlled decryption migrations. Never derive a key from a password without a memory-hard KDF.
