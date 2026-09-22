# JavaScript AI policy sandbox

A dependency-free Node.js policy layer. It authorizes named tools and applies step and argument budgets; it does not execute arbitrary JavaScript.

```js
import { AiSandbox } from "./sandbox.mjs";
const policy = new AiSandbox(["search"], { maxSteps: 4 });
const request = policy.authorize({ name: "search", arguments: { query: "Rust" } });
```
