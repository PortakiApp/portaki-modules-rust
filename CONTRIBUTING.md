# Contributing to portaki-modules-rust

This monorepo hosts **official** Portaki modules (Pattern A in `PORTAKI_PLATFORM.md` §8.1). Third-party modules use standalone repos (Pattern B).

## Adding a module

1. Create `modules/<module-id>/` with `Cargo.toml`, `src/`, `i18n/`, `tests/`.
2. Use workspace dependencies for SDK crates (`portaki-sdk`, `portaki-connectors`, etc.).
3. Add `#[portaki_module(id = "...")]` in `lib.rs`.
4. Run quality gates from repo root:

   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   cd modules/<module-id> && portaki build --release && portaki lint
   ```

5. Open a PR to `main` — never push directly to `main`.

## Capability IDs

Use **string literals** in `#[capability]` attributes until SDK macro path resolution is fixed (see portaki-sdk-rust PR #2).

## i18n

All user-facing strings via `i18n:` keys in bundle files under `i18n/`. No inline French/English in Rust source.

## Releases

- Tag format: `<module-id>-v<semver>` (e.g. `weather-v0.2.0`).
- Requires `SCW_PROJECT_ID` secret and Scaleway OIDC on the repo.
- Operator approval required before tagging production releases.

## SDK dependency

Workspace pins `portaki-sdk-rust` via git branch `fix/macro-expand-emissions` until [PR #2](https://github.com/PortakiApp/portaki-sdk-rust/pull/2) merges; then update root `Cargo.toml` to `branch = "main"`.

## Monorepo + portaki CLI

Each module sets `target-dir = "target"` in `.cargo/config.toml` so `portaki build` / `portaki lint` find macro emissions (workspace builds otherwise use repo-root `target/`).
