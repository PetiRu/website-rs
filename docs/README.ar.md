# website-rs

مساحة عمل Rust معيارية وموجهة للأمان لبناء خدمات ويب وخدمات API.

## نظرة عامة

`website-rs` يجمع اللبنات الأساسية اللازمة لخدمات الويب الآمنة:

- تشفير موثّق
- التحقق من الطلبات وحدود الحجم
- توجيه مع التخزين المؤقت
- حماية الويب
- إعدادات محسوبة
- مراقبة منظمة

## الوحدات الأساسية

- `website-encryption`: تشفير AES-256-GCM
- `website-api`: طلبات واستجابات
- `website-cache`: ذاكرة تخزين مؤقت TTL
- `website-router`: توجيه ومفاتيح التخزين المؤقت
- `website-protection`: رؤوس HTTP والتحقق من المصدر والحد من المعدل
- `website-config`: إعدادات البيئة
- `website-observability`: السجلات والتتبع

## التشغيل السريع

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## الأمان

- لا تضع المفاتيح في المستودع.
- استخدم HTTPS/TLS في الإنتاج.
- قم بتدوير المفاتيح ونقلها بشكل مراقب.
- لا تعتمد على التحقق من العميل كخط حماية.

## الروابط

- [README الرئيسي](../README.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- [SECURITY](../SECURITY.md)
