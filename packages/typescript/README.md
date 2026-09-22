# @websafers/security-typescript

Typed companion helpers for TypeScript services. The implementation uses platform Web Crypto and explicit public types.

```ts
import { MemoryCaptcha, securityHeaders } from "@websafers/security-typescript";

const challenge = new MemoryCaptcha().issueMath();
console.log(challenge.prompt, securityHeaders());
```

Use a shared store for production multi-instance CAPTCHA deployments. Do not use this package as a replacement for authentication, authorization, TLS, CSRF protection, or a threat model.
