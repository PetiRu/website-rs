# Integration guide

## Rust

```toml
[dependencies]
website-provider-defense = { git = "https://github.com/PetiRu/website-rs" }
website-captcha = { git = "https://github.com/PetiRu/website-rs" }
website-csrf = { git = "https://github.com/PetiRu/website-rs" }
```

Map provider telemetry into coarse signals, call `evaluate`, and apply the returned response at your edge. Keep identity correlation, storage, and legal policy in the provider adapter.

## Existing sites

Add controls incrementally: body limits, security headers, rate limiting, CAPTCHA for selected flows, CSRF for cookie-authenticated state changes, then provider-edge quotas and upstream scrubbing.

Do not deploy a global AI/IP blocklist. Use narrow, temporary, explainable responses and protect essential endpoints separately.
