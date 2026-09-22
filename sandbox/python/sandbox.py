from dataclasses import dataclass
from typing import Any, Callable, Mapping


@dataclass(frozen=True)
class ToolRequest:
    name: str
    arguments: Mapping[str, Any]


class SandboxError(ValueError):
    pass


class AiSandbox:
    """Policy layer, not OS isolation. Keep callbacks outside the trust boundary."""

    def __init__(self, allowed_tools: set[str], *, max_steps: int = 8, max_argument_bytes: int = 16_384):
        if max_steps < 1 or max_argument_bytes < 1:
            raise ValueError("limits must be positive")
        self.allowed_tools = frozenset(allowed_tools)
        self.max_steps = max_steps
        self.max_argument_bytes = max_argument_bytes
        self.steps = 0

    def run(self, request: ToolRequest, executor: Callable[[ToolRequest], Any]) -> Any:
        if request.name not in self.allowed_tools:
            raise SandboxError("tool is not allowed")
        if self.steps >= self.max_steps:
            raise SandboxError("step budget exceeded")
        if len(repr(dict(request.arguments)).encode()) > self.max_argument_bytes:
            raise SandboxError("argument budget exceeded")
        self.steps += 1
        return executor(request)
