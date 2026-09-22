# AI sandbox internet contract

Internet-enabled AI workers must not receive unrestricted sockets. The host should run untrusted work in a separate VM or microVM and permit outbound requests only through a gateway.

Required gateway controls:

- default-deny destinations and HTTPS-only by default
- gateway-side DNS resolution and revalidation after redirects
- blocking loopback, private, link-local, reserved, and cloud-metadata ranges
- short-lived capability tokens bound to worker, method, destination, and budgets
- limits for time, bytes, redirects, connections, processes, and output
- no host filesystem, credentials, Docker socket, or cloud identity mounts
- default-deny network egress except the gateway
- kill and cleanup on timeout or policy violation

Cooperating agents cannot be made mathematically escape-proof by an application crate. Defense in depth and infrastructure isolation are mandatory.
