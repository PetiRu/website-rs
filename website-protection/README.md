# Website protection module

The `website-protection` module provides the HTTP-side hardening layer for web services.

## Included protections

- security headers
- origin checking
- request rate limiting
- abuse prevention helpers
- safe defaults for modern sites and APIs

## Example usage

```rust
use website_protection::{origin_allowed, security_headers};

let headers = security_headers();
assert!(headers.iter().any(|(k, v)| *k == "X-Frame-Options" && *v == "DENY"));
assert!(origin_allowed("https://example.com", &["https://example.com"]));
```

## Notes

This module is intentionally simple and framework-agnostic. It can be plugged into an HTTP server or middleware layer with minimal integration work.
