from __future__ import annotations

from dataclasses import dataclass
import secrets
import threading
import time
from typing import Dict, Tuple


@dataclass(frozen=True)
class Challenge:
    token: str
    prompt: str
    expires_in: int


class MemoryCaptcha:
    """Single-process one-time arithmetic challenges.

    Use a bounded shared store such as Redis for multiple workers. The answer
    must remain server-side and should never be sent to the browser.
    """

    def __init__(self) -> None:
        self._pending: Dict[str, Tuple[str, float]] = {}
        self._lock = threading.Lock()

    def issue_math(self, ttl: int = 120) -> tuple[Challenge, str]:
        left = secrets.randbelow(18) + 2
        right = secrets.randbelow(18) + 2
        answer = str(left + right)
        token = secrets.token_urlsafe(24)
        with self._lock:
            self._pending[token] = (answer, time.monotonic() + ttl)
        return Challenge(token, f"What is {left} + {right}?", ttl), answer

    def verify(self, token: str, answer: str) -> bool:
        with self._lock:
            record = self._pending.pop(token, None)
        if record is None or record[1] <= time.monotonic():
            return False
        return secrets.compare_digest(record[0], answer.strip())


class RateLimiter:
    def __init__(self, maximum: int, window_seconds: float) -> None:
        if maximum < 1 or window_seconds <= 0:
            raise ValueError("maximum and window_seconds must be positive")
        self.maximum = maximum
        self.window_seconds = window_seconds
        self._events: Dict[str, list[float]] = {}
        self._lock = threading.Lock()

    def allow(self, identity: str) -> bool:
        now = time.monotonic()
        with self._lock:
            events = [stamp for stamp in self._events.get(identity, []) if now - stamp < self.window_seconds]
            allowed = len(events) < self.maximum
            if allowed:
                events.append(now)
            self._events[identity] = events
            return allowed


def security_headers() -> dict[str, str]:
    return {
        "X-Content-Type-Options": "nosniff",
        "X-Frame-Options": "DENY",
        "Referrer-Policy": "strict-origin-when-cross-origin",
        "Content-Security-Policy": "default-src 'self'",
        "Strict-Transport-Security": "max-age=31536000; includeSubDomains",
    }
