## Summary

Brief description of what this PR does and why.

Closes #<!-- issue number -->

---

## Type of Change

- [ ] Bug fix — non-breaking, fixes incorrect behavior
- [ ] New feature — non-breaking, adds functionality
- [ ] Breaking change — changes existing behavior or API
- [ ] Documentation — no behavior change
- [ ] Performance — no behavior change, improves speed or memory
- [ ] Shader / WGSL — changes to GPU shaders
- [ ] Refactor — no behavior change, improves code structure
- [ ] Test — adds or improves test coverage
- [ ] CI / build — changes to workflow or build configuration

---

## Checklist

- [ ] `cargo fmt` has been run
- [ ] `cargo clippy --workspace --all-features -- -D warnings` is clean
- [ ] `cargo test --workspace --all-features` passes
- [ ] `cargo test -p scenekit-math -p scenekit-core -p scenekit-input --no-default-features` passes
- [ ] New or changed public APIs have `///` doc comments with examples
- [ ] `cargo doc --workspace --all-features` builds without warnings
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
- [ ] Tests are included for new behavior or bug fixes
- [ ] If GPU/shader change: WGSL struct sizes match Rust struct sizes

---

## Notes for Reviewers

Anything the reviewer should pay particular attention to, tricky shader math, or uncertain design decisions.
