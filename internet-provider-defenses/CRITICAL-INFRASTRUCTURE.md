# Critical-infrastructure playbook

This is a defensive operational checklist, not an attack tool.

## Prepare

- maintain upstream DDoS scrubbing and emergency contacts
- isolate hospital and emergency workloads from public origin traffic
- define critical endpoints and authenticated priority classes
- test rate limits, circuit breakers, rollback, and failover
- keep clocks, logs, and incident identifiers consistent

## During an event

- activate a temporary incident policy with an owner and expiry
- shed expensive non-critical work before essential traffic
- enforce per-tenant, per-destination, and concurrency budgets
- block metadata/private egress from AI workers
- preserve health checks, emergency access, and operator channels
- notify upstream providers and affected service owners

## After an event

- expire emergency controls
- review false positives and missed attacks
- rotate compromised credentials
- preserve only necessary evidence
- update thresholds using measured impact, not fear or model labels
