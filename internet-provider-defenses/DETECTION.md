# Automated-abuse detection policy

## Threat model

This policy addresses coordinated automated traffic that exhausts compute, connection slots, bandwidth, or paid resources. It is intended for defensive operation by providers and service owners.

## Signal categories

Store only what is needed for the decision and prefer coarse or keyed values:

- **traffic:** requests/minute, concurrent connections, bytes, error ratio
- **identity:** account age, verified state, token reuse, device/session continuity
- **protocol:** malformed requests, unusual method/path ratios, TLS or HTTP anomalies
- **challenge:** rate of failed or abandoned challenges
- **coordination:** synchronized bursts across accounts, prefixes, or destinations

Never treat an IP address as a person. VPN, carrier-grade NAT, campuses, and proxies can represent many legitimate users.

## Graduated response

1. Observe and aggregate.
2. Add friction with a bounded challenge or proof-of-work.
3. Reduce quotas and require re-authentication.
4. Quarantine the account or automation token with an expiry.
5. Escalate verified incidents to upstream providers.
6. Preserve an appeal and false-positive review process.

## Privacy and governance

- Hash or tokenize identifiers with rotating keys.
- Keep raw network data for the shortest operational period.
- Restrict access and audit defensive decisions.
- Publish retention, appeal, and emergency-blocking policies.
- Require human review for durable account or network blocks.
