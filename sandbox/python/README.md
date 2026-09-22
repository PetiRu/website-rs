# Python AI policy sandbox

A dependency-free example for validating AI tool requests. It does not execute arbitrary Python and should be paired with an isolated worker for untrusted code.

```python
from sandbox import AiSandbox, ToolRequest

sandbox = AiSandbox({"search", "summarize"}, max_steps=4)
result = sandbox.run(ToolRequest("search", {"query": "Rust security"}), lambda request: {"ok": True})
print(result)
```
