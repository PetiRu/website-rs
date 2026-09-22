# Router module

The router layer provides a lightweight dispatch mechanism for service-level routing.

## Responsibilities

- match HTTP methods and paths
- resolve handlers deterministically
- create stable cache keys for safe request/response caching
- keep path matching explicit and predictable

## Important rule

The router is intentionally simple. It does not decide authorization or perform cryptography. Those responsibilities remain in dedicated layers.
