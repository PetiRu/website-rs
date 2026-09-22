export class SandboxError extends Error {}

export class AiSandbox {
  constructor(allowedTools, { maxSteps = 8, maxArgumentBytes = 16_384 } = {}) {
    if (maxSteps < 1 || maxArgumentBytes < 1) throw new RangeError("limits must be positive");
    this.allowedTools = new Set(allowedTools);
    this.maxSteps = maxSteps;
    this.maxArgumentBytes = maxArgumentBytes;
    this.steps = 0;
  }

  authorize(request) {
    if (!this.allowedTools.has(request.name)) throw new SandboxError("tool is not allowed");
    if (this.steps >= this.maxSteps) throw new SandboxError("step budget exceeded");
    const encoded = JSON.stringify(request.arguments ?? {});
    if (Buffer.byteLength(encoded, "utf8") > this.maxArgumentBytes) throw new SandboxError("argument budget exceeded");
    this.steps += 1;
    return Object.freeze({ name: request.name, arguments: request.arguments ?? {} });
  }
}
