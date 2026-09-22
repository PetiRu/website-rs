# website-rs

Um workspace Rust modular e orientado à segurança para serviços web modernos.

## Visão geral

O `website-rs` reúne os blocos básicos geralmente montados manualmente em aplicações web seguras:

- criptografia autenticada
- validação de requisições e limites de tamanho
- roteamento com cache
- proteção web e headers HTTP
- configuração tipada
- observabilidade estruturada

## Módulos principais

- `website-encryption`: criptografia AES-256-GCM
- `website-api`: primitivas de requisição e resposta
- `website-cache`: cache em memória TTL
- `website-router`: resolução de rotas e chaves de cache
- `website-protection`: headers, origens e rate limiting
- `website-config`: configuração por ambiente
- `website-observability`: logs e tracing

## Início rápido

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## Segurança

- Armazene chaves em um gerenciador de segredos.
- Use HTTPS/TLS em produção.
- Rode e migre chaves de forma controlada.
- Validação no cliente não é barreira de segurança.

## Links

- [README principal](../README.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- [SECURITY](../SECURITY.md)
