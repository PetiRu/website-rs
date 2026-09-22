# website-rs

Modüler Rust araç seti ile güvenli web siteleri ve API'ler oluşturun.

Proje; kimlik doğrulamalı şifreleme, istek sınırları, yönlendirme, önbellek, güvenlik başlıkları, rate limiting ve isteğe bağlı tek kullanımlık CAPTCHA meydan okumaları sağlar. Modüller mevcut Rust projelerine ayrı ayrı eklenebilir.

```bash
cargo test --workspace
```

[Entegrasyon](INTEGRATION.md), [mimari](ARCHITECTURE.md) ve [ana README](../README.md) belgelerine bakın. CAPTCHA, TLS veya güçlü bot korumasının yerine geçmez.
