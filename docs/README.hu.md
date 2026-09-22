# website-rs

A modern, security-oriented Rust workspace for building safe web services and API backends.

## Áttekintés

A `website-rs` célja, hogy a webes alkalmazásokhoz szükséges biztonsági és infrastruktúra-alapú építőelemeket jól strukturált, moduláris Rust csomagok formájában kínálja. Ide tartozik:

- hitelesített titkosítás
- HTTP kérés és válasz kezelés
- cache és routing alapok
- webes biztonsági védelmi elemek
- konfiguráció és megfigyelhetőség

## Fő modulok

- `website-encryption`: AES-256-GCM alapú titkosítás
- `website-api`: kérés/válasz absztrakciók és méretkorlátok
- `website-cache`: TTL alapú memória cache
- `website-router`: útvonal feloldás és cache kulcsok
- `website-protection`: fejlécek, origin ellenőrzés, rate limiting
- `website-config`: környezeti konfiguráció
- `website-observability`: logging és tracing

## Gyors kezdés

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## Biztonsági irányelvek

- A kulcsokat soha ne tedd verziókezelés alá.
- Production környezetben használj TLS-t és titkos kulcstárolót.
- A titkosítási kulcsokat rendszeresen cseréld és migráld biztonságosan.
- A kliens oldali validáció nem biztonsági határ.

## Források

- [README.md](../README.md)
- [CONTRIBUTING.md](../CONTRIBUTING.md)
- [SECURITY.md](../SECURITY.md)
