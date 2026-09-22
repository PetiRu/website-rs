# Rust AI policy sandbox

`website-ai-sandbox` validates AI tool requests and returns authorized requests to a host-owned executor. It does not run arbitrary Rust, shell, filesystem, or network code.

```rust
use website_ai_sandbox::{Sandbox, ToolRequest};

let mut policy = Sandbox::new(["search"], 4, 16_384);
let request = policy.authorize(ToolRequest { name: "search".into(), arguments: "{\"q\":\"rust\"}".into() })?;
// Execute `request` only in a separately controlled host adapter.
let _ = request;
# Ok::<(), website_ai_sandbox::Error>(())
```
