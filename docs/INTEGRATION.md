# 🧩 Easy integration

`website-rs` is framework-agnostic by design. Add only the crates you need to an existing or new Rust site.

## Add selected modules

```toml
[dependencies]
website-api = { git = "https://github.com/PetiRu/website-rs" }
website-protection = { git = "https://github.com/PetiRu/website-rs" }
website-encryption = { git = "https://github.com/PetiRu/website-rs" }
```

For a local workspace, use path dependencies instead.

## Adapter pattern

Translate your framework request into `website_api::Request`, apply body limits and protection, resolve your framework handler, then translate the response back. The crates do not require Axum, Actix, Warp, Rocket, or a specific runtime.

## Captcha integration

The optional `website-captcha` crate provides a one-time challenge store and a simple arithmetic challenge suitable for low-risk forms and development. It is not a replacement for bot detection, rate limiting, email verification, or a production CAPTCHA provider.

```rust
use std::time::Duration;
use website_captcha::{ChallengeStore, MemoryCaptcha};

let captcha = MemoryCaptcha::new();
let (challenge, answer_for_demo) = captcha.issue_math(Duration::from_secs(120));
// Render `challenge.prompt` and keep `challenge.token` with the form.
captcha.verify(&challenge.token, &answer_for_demo)?;
# Ok::<(), website_captcha::Error>(())
```

For multiple application instances, implement `ChallengeStore` over Redis or another shared store. Never expose the expected answer to a browser in a real application.
