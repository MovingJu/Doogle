# 기여 가이드

## 브랜치 전략

- 기본 개발 브랜치는 `develop`. 새 작업은 `develop`에서 브랜치를 따서 `develop`으로 PR을 연다.
- `main`은 릴리즈 브랜치. 보호되어 있어 PR을 통해서만, 리뷰 승인과 CI 통과 후에만 반영된다. `develop → main` PR은 유지보수자가 릴리즈 시점에 연다.

## 커밋 / PR 제목 규칙

모든 PR은 병합 시 **Squash and merge**로 처리되고, PR 제목이 그대로 `main`/`develop`의 커밋 메시지가 된다. 그래서 PR 제목은 [Conventional Commits](https://www.conventionalcommits.org/) 형식을 따라야 한다:

```
<type>(<scope>)?: <description>
```

- `type`: `feat` `fix` `docs` `style` `refactor` `perf` `test` `chore` `build` `ci` 중 하나
- 예: `feat(tokenizer): add bigram support`, `fix(index): handle empty query`
- PR 안의 개별 커밋 메시지 형식은 자유. 어차피 머지되면서 PR 제목 하나로 뭉쳐진다.

## CI

- PR을 열면 제목 형식 검사가 바로 돈다.
- `develop`에 커밋이 반영되면 전체 검사(`cargo build`, `cargo test`, `cargo fmt --check`, `cargo clippy`)가 돈다.
- `main`으로 가는 PR(릴리즈 PR)은 전체 검사를 통과해야 병합 가능하다.

## PR 체크리스트

- [ ] 로컬에서 `cargo fmt`, `cargo clippy`, `cargo test` 통과 확인
- [ ] 관련 이슈 연결 (`Closes #`)
- [ ] 리뷰어 1명 이상 승인 (main 대상 PR)
