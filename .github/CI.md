# GitHub Actions CI

## Flow

1. **`changes`** — `changed-modules.sh` → JSON module list (or empty).
2. **`rust`** — fmt + clippy + tests on one runner (setup cost shared).
3. **`wasm` matrix** (only if modules changed) → upload `wasm-{module}` artifact.
4. **`publish` matrix** on `main` push — download artifact, `portaki publish --skip-build`.
5. **`quality`** gate aggregates results.

## Changed-modules rules

- Touch `modules/<name>/…` → only that module’s wasm/publish.
- Touch `Cargo.toml` / `Cargo.lock` / toolchain → **all** modules.
- Touch **only** `.github/workflows/` or `.github/scripts/` → **no** wasm matrix (rust still runs). CI-only edits must not burn N× publish minutes.

## Minutes

GitHub bills **job-minutes**. Parallel matrix jobs multiply cost.

- Quality on **`pull_request`**; `push` only on **`main`** (publish) — no `feature/**` double bill with an open PR.
- `max-parallel: 2` on wasm/publish — softens spikes without serializing forever.
- Prefer one rust job over fmt/clippy/test split when setup dominates.
- `concurrency` cancels superseded PR runs.
- Artifacts: `retention-days: 1`.

Local Cursor mirror (gitignored): `.cursor/rules/github-actions-ci.mdc`.
