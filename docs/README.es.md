# website-rs

Un espacio de trabajo modular y seguro en Rust para servicios web y APIs modernas.

## Visión general

`website-rs` está pensado para reunir los bloques básicos que normalmente se implementan a mano en servicios web seguros:

- cifrado autenticado
- validación de peticiones y límites de tamaño
- enrutado con caché
- protección web y seguridad HTTP
- configuración tipada
- observabilidad estructurada

## Módulos principales

- `website-encryption`: cifrado autenticado con AES-256-GCM
- `website-api`: tipos de petición y respuesta
- `website-cache`: caché en memoria por TTL
- `website-router`: resolución de rutas y claves de caché
- `website-protection`: headers, orígenes y rate limiting
- `website-config`: configuración basada en variables de entorno
- `website-observability`: logging y tracing

## Inicio rápido

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## Seguridad

- Guarda llaves en un gestor de secretos.
- Usa HTTPS/TLS en producción.
- Rota y migra claves con control.
- No relies en validación de cliente para proteger datos sensibles.

## Enlaces

- [README principal](../README.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- [SECURITY](../SECURITY.md)
