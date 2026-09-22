# Detection and response policy

## What may be scored

Use coarse, explainable behavior signals:

- request rate and concurrency
- failed challenges and authentication failures
- malformed protocol traffic
- synchronized bursts across independent pseudonymous sources
- resource cost and destination concentration

Do not infer identity from an IP, VPN exit, ASN, device fingerprint, or language model detector. These signals are uncertain and can harm real users.

## Response ladder

| Level | Response | Expiry |
| --- | --- | --- |
| low | observe | n/a |
| medium | challenge and reduce expensive quotas | minutes |
| high | throttle and require re-authentication | minutes |
| critical | narrow temporary quarantine and upstream mitigation | minutes, renewable only with review |

Every response should have a reason code, an expiry, an audit event without secrets, and an appeal or operator override. Permanent blocking requires a separate governance process and must not be inferred from a model score.

## Shared provider operation

For AT&T, VPN, ISP, CDN, and hosting deployments, expose an adapter interface rather than hard-coding provider-specific surveillance or bypass behavior. Exchange incident indicators through authenticated, lawful channels, with data minimization and retention limits.
