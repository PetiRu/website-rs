# Internet access example

The policy below is only an authorization layer. It does not create isolation by itself.

```js
export class EgressDenied extends Error {}

export function authorizeEgress(rawUrl, { allowedHosts, method = "GET" }) {
  const url = new URL(rawUrl);
  if (url.protocol !== "https:" || url.username || url.password) throw new EgressDenied("HTTPS without credentials required");
  if (!allowedHosts.has(url.hostname.toLowerCase())) throw new EgressDenied("destination is not allowlisted");
  if (!["GET", "HEAD"].includes(method.toUpperCase())) throw new EgressDenied("method is not allowed");
  return url;
}
```

Resolve DNS and block private/link-local/metadata addresses in the egress gateway, not in an untrusted worker. Follow redirects only after re-authorizing every destination.
