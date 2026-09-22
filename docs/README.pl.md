# website-rs

Modularny zestaw komponentów Rust do bezpiecznych stron internetowych i API.

Zapewnia szyfrowanie uwierzytelnione, limity żądań, routing, cache, nagłówki bezpieczeństwa, rate limiting oraz opcjonalne jednorazowe wyzwania CAPTCHA. Moduły można dodawać osobno do istniejącego projektu.

```bash
cargo test --workspace
```

Zobacz [integrację](INTEGRATION.md), [architekturę](ARCHITECTURE.md) i [główny README](../README.md). CAPTCHA nie zastępuje TLS, autoryzacji ani profesjonalnej ochrony antybotowej.
