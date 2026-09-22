# Easy integration

The project is designed for incremental adoption. Add only the package that matches your stack.

## Rust

```toml
[dependencies]
website-api = { git = "https://github.com/PetiRu/website-rs" }
website-protection = { git = "https://github.com/PetiRu/website-rs" }
website-captcha = { git = "https://github.com/PetiRu/website-rs" }
website-csrf = { git = "https://github.com/PetiRu/website-rs" }
```

Translate your framework request into `website_api::Request`, enforce limits, apply protection, verify CSRF for cookie-authenticated state-changing requests, then dispatch your handler.

## Python, JavaScript, and TypeScript

Install the local package while developing:

```bash
pip install -e packages/python
cd packages/javascript && npm install
cd ../typescript && npm install
```

The companion packages expose small helpers for security headers, rate limiting, CAPTCHA examples, and Web Crypto where supported. They are intentionally framework-neutral, so they can be wrapped by Flask/FastAPI, Express/Fastify, or another server.

## Existing-site checklist

1. Put TLS and a trusted proxy configuration at the edge.
2. Add body limits before parsing large payloads.
3. Add security headers without overwriting deliberate application policy.
4. Apply rate limits by a carefully chosen identity.
5. Use CSRF protection for cookie-authenticated state changes.
6. Never cache private responses or authorization-bearing data.
7. Keep keys and CAPTCHA answers server-side.
8. Add integration tests for rejection paths.
