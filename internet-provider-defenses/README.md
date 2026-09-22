# Internet-enabled AI sandboxing and provider defenses

This directory documents a safe design for AI agents that need limited internet access and for providers that need to absorb coordinated automated abuse.

> **Important limitation:** an in-process allowlist is not an escape-proof sandbox. No software package can honestly guarantee that cooperating agents cannot escape. Internet access must be mediated by an isolated worker, a default-deny egress proxy, and infrastructure-level controls.

## Safe internet model

```text
AI request
   │ authenticated, schema-validated
   ▼
Policy service ── tool allowlist, budget, destination policy
   │ signed short-lived capability
   ▼
Isolated worker ── non-root, read-only, no host secrets
   │
   ▼
Egress gateway ── default deny, DNS pinning, URL/IP policy, quotas
   │
   ▼
Internet
```

Use a separate VM or microVM for untrusted code, not just a process sandbox. Disable host networking and metadata endpoints, mount no credentials, apply CPU/memory/process/output/time limits, and terminate the worker on policy violations.

## Provider-neutral defenses

The `internet-provider-defenses/` folder contains guidance applicable to ISPs, hosting providers, VPN providers, CDNs, and enterprise networks. It does not contain AT&T-specific bypasses or surveillance tooling.

Recommended controls:

- per-account, per-IP, per-prefix, per-ASN, and per-destination rate limits
- connection, request, bandwidth, and concurrency budgets
- SYN/UDP amplification protection and upstream DDoS scrubbing
- authenticated abuse reports and rapid quarantine workflows
- bot signals combined with account and network context
- privacy-preserving aggregation, short retention, and appeal paths
- emergency circuit breakers that protect critical services

## AI-automation detection

Do not label a person, profile, or IP as AI-operated from one signal. Automation scores are probabilistic and can cause false positives, especially for NAT, VPN, Tor, mobile networks, accessibility tools, and shared infrastructure.

Use multiple signals—request velocity, session consistency, token reuse, challenge outcomes, protocol fingerprints, and behavior over time—to trigger graduated actions such as proof-of-work, CAPTCHA, throttling, review, or temporary quarantine. Do not use the score alone for identity, access, or punishment.
