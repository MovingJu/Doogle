<!-- PR title must follow Conventional Commits: <type>(<scope>)?: <description> -->
<!-- e.g. feat(tokenizer): add bigram support / fix(index): handle empty query -->
<!-- Merges use Squash and merge, so this title becomes the commit message. -->

## Summary

<!-- What changed and why, in a line or two -->

## Related issue

Closes #

## Testing

<!-- How you verified this (unit tests, manual checks, etc.) -->

## Checklist

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test` passes
