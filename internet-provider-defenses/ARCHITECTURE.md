# Provider defense architecture

This folder is a provider-neutral blueprint for ISPs, VPN providers, hosting companies, CDNs, and SaaS operators.

## Control plane

- tenant and account quotas
- abuse-event aggregation
- risk scoring with explainable signals
- circuit breakers and temporary quarantine
- human review and appeals

## Data plane

- connection and request limits
- per-destination concurrency budgets
- upstream DDoS mitigation
- malformed protocol rejection
- cache and origin protection
- emergency traffic shedding

## Avoid dangerous shortcuts

Do not block entire countries, ASNs, VPN providers, or carrier prefixes solely because automated traffic was observed. Do not infer that a profile is AI-generated as a fact. Use narrowly scoped, time-limited controls and measure false positives.
