# website-rs

Ein modulares, sicherheitsorientiertes Rust-Workspace für moderne Webdienste.

## Überblick

`website-rs` bündelt die Grundbausteine, die typischerweise für sichere Webanwendungen von Hand zusammengesetzt werden:

- authentifizierte Verschlüsselung
- Anfragevalidierung und Größenbegrenzungen
- Routing mit Cache
- Web-Schutzmechanismen
- konfigurierte Umgebungswerte
- strukturierte Observability

## Kernmodule

- `website-encryption`: AES-256-GCM Verschlüsselung
- `website-api`: Anfrage/Antwort-Primitiven
- `website-cache`: TTL-basierter Speicher-Cache
- `website-router`: Routenauflösung und Cache-Schlüssel
- `website-protection`: Header, Origin-Checks, Rate Limiting
- `website-config`: Umgebungsmodellierung
- `website-observability`: Logging und Tracing

## Schnellstart

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## Sicherheit

- Schlüssel nie in Git-Versionierung speichern.
- In Produktion HTTPS/TLS und Secret-Management verwenden.
- Schlüssel rotieren und sauber migrieren.
- Client-side Validierung ist keine Sicherheitsgrenze.

## Verknüpfungen

- [Haupt-README](../README.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- [SECURITY](../SECURITY.md)
