# Contributing Guide

## Branch strategy

- The default development branch is `develop`. Branch off `develop` for new work and open PRs against `develop`.
- `main` is the release branch. It's protected: changes land only through PRs, after review approval and a passing CI run. `develop → main` PRs are opened by a maintainer at release time.

## Commit / PR title convention

Every PR is merged with **Squash and merge**, so the PR title becomes the commit message on `main`/`develop`. PR titles must follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>)?: <description>
```

- `type`: one of `feat` `fix` `docs` `style` `refactor` `perf` `test` `chore` `build` `ci`
- Examples: `feat(tokenizer): add bigram support`, `fix(index): handle empty query`
- Individual commit messages inside a PR can be whatever — they get squashed into the PR title anyway.

## CI

- Opening a PR immediately runs the title-format check.
- Every commit landing on `develop` runs the full suite (`cargo build`, `cargo test`, `cargo fmt --check`, `cargo clippy`).
- PRs into `main` (release PRs) must pass the full suite before they can be merged.

## PR checklist

- [ ] `cargo fmt`, `cargo clippy`, and `cargo test` pass locally
- [ ] Linked to the relevant issue (`Closes #`)
- [ ] At least one reviewer approval (for PRs into `main`)
