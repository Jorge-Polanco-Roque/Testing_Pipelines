# Contributing to statkit

Thanks for helping! A few conventions keep the project healthy:

## Before you code
- **Discuss design/API changes in the issue first.** For anything beyond a well-scoped bug
  fix or a small addition, get maintainer agreement on the approach *before* opening a PR.
  Vague or design-level issues are for discussion, not immediate PRs.

## Pull requests
- Keep PRs **small and focused** — one concern per PR.
- **Sign off your commits** (DCO): `git commit -s`. Every commit must carry a `Signed-off-by:` trailer.
- Add a **test that fails without your change**.
- Follow the PR template (problem · root cause · approach · testing · `Fixes #NNN`).
- CI must be green (`cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`).

## Style
- Match the surrounding code. Minimal diffs. No `unwrap`/`panic!` in library code without reason.
