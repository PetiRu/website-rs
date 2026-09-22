# website-rs

Rust の安全性重視のモジュール型ワークスペースです。

## 概要

`website-rs` は、セキュアな Web サービスで必要になる基本的な構成要素をまとめたものです。

- 認証付き暗号化
- リクエスト検証とサイズ制限
- キャッシュ付きルーティング
- Web セキュリティ保護
- 型付き設定
- 構造化ログと検証

## 主なモジュール

- `website-encryption`: AES-256-GCM を使った暗号化
- `website-api`: リクエスト/レスポンスの基本型
- `website-cache`: TTL ベースのメモリキャッシュ
- `website-router`: ルート解決とキャッシュキー
- `website-protection`: ヘッダー、オリジン制御、レート制限
- `website-config`: 環境変数ベースの設定
- `website-observability`: ロギングとトレース

## クイックスタート

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## セキュリティ

- 鍵はバージョン管理しない
- 本番環境では TLS を使用する
- 鍵のローテーションと移行を計画的に行う
- クライアント側の検証は安全境界ではない

## リンク

- [メイン README](../README.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- [SECURITY](../SECURITY.md)
