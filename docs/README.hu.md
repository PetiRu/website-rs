# website-rs

Moduláris, biztonságközpontú Rust munkaterület weboldalak és API-k építéséhez.

## Fő funkciók

- hitelesített AES-256-GCM titkosítás
- HTTP kérés- és válaszkezelési alapok
- TTL cache és útválasztás
- biztonsági fejlécek, origin-ellenőrzés és rate limiting
- opcionális, egyszer használható matematikai CAPTCHA-primitív
- könnyű integráció Axum, Actix, Warp vagy saját szerverréteggel

## Gyors kezdés

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## Integráció

A szükséges crate-eket külön is hozzáadhatod a `Cargo.toml` fájlhoz. A `website-captcha` fejlesztéshez és alacsony kockázatú űrlapokhoz használható; éles rendszernél közös tároló, rate limiting és további botvédelem szükséges.

Részletek: [integrációs útmutató](INTEGRATION.md), [architektúra](ARCHITECTURE.md), [fő README](../README.md).

## Biztonság

A kulcsokat ne tárold Gitben, használj TLS-t, és ne kezeld a kliensoldali validációt biztonsági határként.
