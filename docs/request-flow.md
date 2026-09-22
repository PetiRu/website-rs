# API and routing flow

A framework adapter should apply the layers in this order:

1. Parse the incoming request and reject malformed framing at the HTTP edge.
2. Enforce `website-api` body limits before buffering or deserializing data.
3. Apply origin policy, security headers, and rate limits from `website-protection`.
4. Resolve the method/path through the application router.
5. Build a cache key only for responses that are safe to cache. Never cache private responses by default.
6. Encrypt sensitive values with `website-encryption` using purpose-specific associated data.
7. Emit structured logs without request bodies, credentials, keys, or tokens.

The workspace intentionally avoids selecting one web framework. Adapters can be added without changing the security primitives.
