# Egress gateway contract

The gateway is the only component allowed to make outbound requests for an AI worker.

## Required controls

- default-deny destination policy
- HTTPS-only by default
- DNS resolution performed by the gateway
- block private, loopback, link-local, cloud metadata, and reserved ranges
- re-check redirects against the same policy
- maximum response bytes, download time, redirects, and concurrent requests
- per-worker and per-tenant bandwidth quotas
- request and response content-type limits
- no arbitrary TCP, UDP, SMTP, or raw socket access unless explicitly reviewed
- immutable audit events without secrets or full payloads

## Capability tokens

Issue short-lived, narrowly scoped capabilities containing the worker, destination class, method, byte budget, and expiry. Bind them to the worker identity and reject replay. A policy service should revoke capabilities when a budget or abuse threshold is reached.

These controls reduce blast radius; they do not prove mathematical escape resistance. Keep the worker isolated at the VM or microVM layer.
