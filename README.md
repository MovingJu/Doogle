# Doogle

A general-purpose document search engine. An axum-based API that indexes a list of documents and ranks them by BM25 relevance for a given query.

## Run

```bash
cargo run
```

Listens on `0.0.0.0:3000` by default. See `fixtures/sample_documents.json` for indexing/search examples.

```bash
curl -X POST localhost:3000/documents \
  -H 'Content-Type: application/json' \
  -d @fixtures/sample_documents.json

curl 'localhost:3000/search?q=러스트'
```

## Development status

- Roadmap / versioning rules: [issue #13](https://github.com/MovingJu/Doogle/issues/13) (pinned)
- The current phase (v1.0.0 — core search engine) is split into 4 steps: tokenizer → inverted index → BM25 scoring → HTTP API wiring
- Per-milestone progress: repo Insights → Milestones

## Structure

```
src/
  main.rs      — axum router (/documents, /search)
  tokenizer.rs — text → tokens
  index.rs     — inverted index + BM25 scoring
fixtures/
  sample_documents.json — mock data for local testing/curl
```

## Versioning

semver (MAJOR.MINOR.PATCH). v1.0.0 marks the point where the core search engine is complete; later features (auth/persistence/search quality) bump MINOR. See issue #13 for details.
