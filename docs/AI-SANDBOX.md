# AI sandbox safety model

The `sandbox/` implementations are policy sandboxes, not secure arbitrary-code runners. An allowlist and a byte/step budget cannot contain malicious code by themselves.

For untrusted generated code, use a separate worker with:

- a non-root identity and read-only filesystem
- no host socket or secret mounts
- default-deny network egress
- CPU, memory, process, output, and wall-clock limits
- container or microVM isolation appropriate to the threat model
- a kill path and cleanup for timed-out jobs
- independent logging and abuse monitoring

Treat all model output as untrusted input. Validate tool arguments with schemas, authorize every action server-side, and never pass ambient credentials into an AI execution environment.
