# website-rs

Moduláris Rust biztonsági eszköztár webhelyekhez és API-khoz. Tartalmaz titkosítást, kéréskorlátokat, routingot, cache-t, biztonsági fejléceket, rate limitinget, CAPTCHA- és CSRF-primitíveket.

A Rust mellett könnyű Python, JavaScript és TypeScript csomagok is találhatók a `packages/` könyvtárban, így a meglévő webhelyek fokozatosan bővíthetők.

```bash
cargo test --workspace
```

Dokumentáció: [integráció](INTEGRATION.md), [architektúra](ARCHITECTURE.md), [fő README](../README.md).
