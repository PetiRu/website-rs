# Internet provider defenses

This module is provider-neutral and can be activated by ISPs, VPN providers, hosting providers, CDNs, and enterprise networks. It is designed for coordinated automated resource exhaustion, including incidents where many agents target one another or shared infrastructure.

It does **not** identify a person as an AI, treat an IP as a person, or provide a permanent blocklist. VPNs, carrier-grade NAT, mobile networks, and shared proxies contain legitimate users.

## Activation model

1. Observe traffic and aggregate privacy-preserving signals.
2. Apply short-lived challenges and quotas.
3. Throttle expensive operations and protect critical origins.
4. Temporarily quarantine narrowly scoped traffic when multiple signals agree.
5. Escalate verified incidents to upstream DDoS providers and affected operators.
6. Review false positives and automatically expire emergency actions.

## Provider modules

- `provider-defense` — behavior scoring and graduated responses
- `internet-provider-defenses/` — egress, detection, and operational guidance
- provider adapters should translate local telemetry into coarse `Signals`; they should not export raw subscriber data unnecessarily

## Emergency mode

For hospital, emergency, payment, and other critical systems, use an independently operated circuit breaker and upstream scrubbing service. Prefer preserving essential traffic by endpoint and authenticated role rather than blocking an entire ASN, country, VPN, or carrier prefix.

No software library can guarantee that cooperating agents cannot escape a sandbox. Use microVM/container isolation, default-deny egress, no host secrets, resource limits, and a kill path for untrusted execution.
