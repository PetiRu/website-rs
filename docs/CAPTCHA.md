# Security note for CAPTCHA

The included arithmetic challenge is a composable example, not a universal CAPTCHA product. It should be combined with request limits, IP/user throttling, CSRF protection where applicable, telemetry, and server-side validation.

The default memory store is single-process and consumes memory per pending token. Expire and bound it in production, or implement `ChallengeStore` using a shared bounded store. Do not put answers in HTML, JavaScript, URLs, logs, or analytics. Do not use CAPTCHA as the only control for authentication, authorization, payments, or account recovery.
