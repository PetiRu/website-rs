# website-rs

Un espace de travail Rust modulaire et orienté sécurité pour les services web modernes.

## Vue d'ensemble

`website-rs` regroupe les briques de base souvent réécrites pour les applications web sécurisées :

- chiffrement authentifié
- validation des requêtes et limites de taille
- routage avec cache
- protection HTTP
- configuration typée
- observabilité structurée

## Modules clés

- `website-encryption` : chiffrement AES-256-GCM
- `website-api` : requêtes et réponses
- `website-cache` : cache mémoire TTL
- `website-router` : résolution d'URL et clés de cache
- `website-protection` : en-têtes HTTP, origines, rate limiting
- `website-config` : configuration environnementale
- `website-observability` : logs et tracing

## Démarrage rapide

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## Sécurité

- Stockez les clés dans un gestionnaire de secrets.
- Utilisez HTTPS/TLS en production.
- Faites tourner les clés via une migration contrôlée.
- Ne considérez pas la validation côté client comme une barrière de sécurité.

## Liens

- [README principal](../README.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- [SECURITY](../SECURITY.md)
