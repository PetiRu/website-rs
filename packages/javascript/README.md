# @websafers/security for JavaScript

Small Web Crypto and website-protection helpers for Node.js 18+ and browser-compatible environments.

```js
import { MemoryCaptcha, securityHeaders } from "@websafers/security";

const captcha = new MemoryCaptcha();
const challenge = captcha.issueMath();
console.log(challenge.prompt);
console.log(captcha.verify(challenge.token, "demo-answer"));
console.log(securityHeaders());
```

The CAPTCHA example is intentionally basic and must be combined with rate limiting and server-side controls. `encryptAesGcm` uses the platform Web Crypto API and returns `iv || ciphertext-with-tag`.
