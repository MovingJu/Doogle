<!-- PR 제목은 Conventional Commits 형식이어야 합니다: <type>(<scope>)?: <description> -->
<!-- 예: feat(tokenizer): add bigram support / fix(index): handle empty query -->
<!-- 병합은 Squash and merge로 처리되며, 이 PR 제목이 그대로 커밋 메시지가 됩니다. -->

## 요약

<!-- 뭘 왜 바꿨는지 한두 줄 -->

## 관련 이슈

Closes #

## 테스트

<!-- 어떻게 확인했는지 (유닛 테스트, 수동 확인 등) -->

## 체크리스트

- [ ] `cargo fmt --all -- --check` 통과
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` 통과
- [ ] `cargo test` 통과
