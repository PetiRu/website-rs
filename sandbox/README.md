# AI sandboxing

The repository now includes policy sandboxes in Python, Rust, and JavaScript under `sandbox/`, plus the Rust crate `website-ai-sandbox`.

These modules authorize named AI tool calls and enforce simple budgets. They do not provide operating-system isolation and must not execute untrusted code directly. See [AI sandbox safety](docs/AI-SANDBOX.md).
