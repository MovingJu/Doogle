# Doogle

범용 문서 검색 엔진. 문서 목록을 색인하고, 쿼리에 맞게 BM25로 순위를 매겨 반환하는 axum 기반 API.

## 실행

```bash
cargo run
```

기본적으로 `0.0.0.0:3000`에서 뜬다. 문서 색인/검색 예시는 `fixtures/sample_documents.json` 참고.

```bash
curl -X POST localhost:3000/documents \
  -H 'Content-Type: application/json' \
  -d @fixtures/sample_documents.json

curl 'localhost:3000/search?q=러스트'
```

## 개발 진행 상황

- 로드맵 / 버전 규칙: [이슈 #13](https://github.com/MovingJu/Doogle/issues/13) (pinned)
- 지금 단계(v1.0.0 — 핵심 검색 엔진)는 4단계로 쪼개져 있음: 토크나이저 → 역색인 → BM25 스코어링 → HTTP API 연결
- Milestone별 진행률: 리포 Insights → Milestones

## 구조

```
src/
  main.rs      — axum 라우터 (/documents, /search)
  tokenizer.rs — 텍스트 → 토큰
  index.rs     — 역색인 + BM25 스코어링
fixtures/
  sample_documents.json — 로컬 테스트/curl용 목 데이터
```

## 버전 규칙

semver(MAJOR.MINOR.PATCH). 핵심 검색 엔진 완성 시점을 1.0.0으로 잡고, 이후 기능(인증/영속성/검색품질)은 MINOR를 올린다. 자세한 내용은 이슈 #13 참고.
