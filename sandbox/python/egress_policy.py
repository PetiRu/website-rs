from dataclasses import dataclass
from ipaddress import ip_address, ip_network
from urllib.parse import urlparse


class EgressDenied(ValueError):
    pass


@dataclass(frozen=True)
class EgressPolicy:
    allowed_hosts: frozenset[str]
    max_response_bytes: int = 1_000_000
    max_redirects: int = 2

    def authorize(self, url: str, method: str = "GET") -> None:
        parsed = urlparse(url)
        if parsed.scheme != "https" or parsed.username or parsed.password or not parsed.hostname:
            raise EgressDenied("only credential-free HTTPS URLs are allowed")
        if method.upper() not in {"GET", "HEAD"}:
            raise EgressDenied("method is not allowed")
        if parsed.hostname.lower() not in self.allowed_hosts:
            raise EgressDenied("destination is not allowlisted")
        try:
            address = ip_address(parsed.hostname)
        except ValueError:
            return
        blocked = (ip_network("10.0.0.0/8"), ip_network("172.16.0.0/12"), ip_network("192.168.0.0/16"), ip_network("127.0.0.0/8"), ip_network("169.254.0.0/16"), ip_network("::1/128"), ip_network("fc00::/7"))
        if any(address in network for network in blocked):
            raise EgressDenied("private or link-local destination is blocked")
