# Cache module

The `cache` concept in this repository is intentionally small and predictable.

## What it provides

- TTL-aware values
- bounded in-memory capacity
- simple `Cache` trait abstraction
- safe default behavior for response caching and temporary state

## Typical usage

Use the cache for:

- short-lived API responses
- repeated expensive reads
- precomputed metadata
- request deduplication where safe

Avoid caching:

- secret-bearing responses
- user-specific sensitive content
- headers with authorization metadata
