import test from "node:test";
import assert from "node:assert/strict";
import { AiSandbox, SandboxError } from "./sandbox.mjs";

test("authorizes allowlisted tools and rejects shell", () => {
  const box = new AiSandbox(["search"], { maxSteps: 1 });
  assert.equal(box.authorize({ name: "search", arguments: { q: "rust" } }).name, "search");
  assert.throws(() => box.authorize({ name: "search", arguments: {} }), SandboxError);
  assert.throws(() => new AiSandbox(["search"]).authorize({ name: "shell", arguments: {} }), SandboxError);
});
