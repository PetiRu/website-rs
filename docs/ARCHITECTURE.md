# Architecture

This project is intentionally built as a collection of clear, bounded modules rather than a single heavyweight HTTP framework abstraction.

## Layered model

### 1. Transport boundary

The HTTP layer should parse the raw request and perform basic validation.

Responsibilities:

- request validation
- method enforcement
- maximum body-size checks
- protocol-level sanity checks

### 2. Security enforcement

`website-protection` handles the most common safety controls:

- security headers
- origin allowlisting
- rate limits
- abuse detection hooks

### 3. Routing and dispatch

`website-router` resolves a request to a logical handler using exact path matching. It also provides a stable cache key derived from the HTTP method and path.

### 4. Cache layer

`website-cache` enables TTL-based caching with a bounded memory policy. This is intentionally simple and easy to reason about.

### 5. Encryption layer

`website-encryption` should be used only for sensitive payloads or secrets that must be protected at rest or in transit across trust boundaries.

The crate uses AES-256-GCM and binds encryption to associated data to reduce accidental reuse across contexts.

### 6. Observability and operations

`website-observability` initializes tracing in a consistent way so logging remains structured and production friendly.

## Why this shape works

This modular division makes each component easier to test, reason about, and swap out as requirements evolve.

It also reduces the risk of accidentally mixing concerns such as request parsing, route matching, cache policy, and cryptographic handling into one large codepath.

## Recommended integration pattern

Use a server adapter to translate framework-specific request types into the project’s lightweight request/response models, then apply the security pipeline before business logic.

## Security assumptions

This project assumes:

- TLS is terminated upstream or at the edge
- sensitive keys live outside source control
- secure secret rotation is part of deployment policy
- application authorization is enforced separately from transport security
