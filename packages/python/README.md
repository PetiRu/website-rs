# website-rs-security for Python

Framework-neutral helpers for Flask, Django, FastAPI, and custom Python services.

```python
from website_rs_security import MemoryCaptcha, RateLimiter, security_headers

captcha = MemoryCaptcha()
challenge, answer_for_demo = captcha.issue_math()
assert captcha.verify(challenge.token, answer_for_demo)
assert RateLimiter(10, 60).allow("client-1")
```

The memory implementations are suitable for examples and single-process services. Use a shared, bounded store for multiple workers. These helpers do not replace TLS, CSRF protection, authentication, authorization, or a security review.
