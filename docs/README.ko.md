# website-rs

안전한 웹 서비스와 API 백엔드를 위한 모듈형 Rust 워크스페이스입니다.

## 개요

`website-rs`는 보안 중심 웹 서비스에서 자주 반복되는 기본 요소를 정리해 둔 프로젝트입니다.

- 인증된 암호화
- 요청 검증 및 크기 제한
- 캐시 기반 라우팅
- 웹 보호 기능
- 타입 기반 설정
- 구조화된 관측성

## 핵심 모듈

- `website-encryption`: AES-256-GCM 기반 암호화
- `website-api`: 요청/응답 모델
- `website-cache`: TTL 메모리 캐시
- `website-router`: 경로 해석과 캐시 키
- `website-protection`: 보안 헤더, Origin 검사, 속도 제한
- `website-config`: 환경설정
- `website-observability`: 로깅과 트레이싱

## 빠른 시작

```bash
cargo test --workspace
cargo run --manifest-path examples/secure-service/Cargo.toml
```

## 보안 원칙

- 키는 Git에 저장하지 않는다.
- 운영 환경에서는 TLS를 사용한다.
- 키는 계획적으로 교체하고 이전 버전과 안전하게 마이그레이션한다.
- 클라이언트 검증은 보안 경계가 아니다.

## 링크

- [메인 README](../README.md)
- [CONTRIBUTING](../CONTRIBUTING.md)
- [SECURITY](../SECURITY.md)
