# Branch policy — portaki-modules

Repo: **PortakiApp/portaki-modules** · default branch: **`main`**

## Flow

```
feat/fix/* ──PR──► main ──► CI (quality) ──► publish GHCR (semver from Cargo.toml)
```

Bump `modules/<id>/Cargo.toml` version in the PR that ships the change. Merge to `main` = publish. No release-please.

## Merge settings

| Setting | Value |
|---------|--------|
| **Rebase and merge** | Only |
| Squash / merge commit | Off |
| Delete head branch on merge | Yes |

## Rulesets

| Ruleset | Target | Rules |
|---------|--------|--------|
| `portaki-modules: branch integrity` | `main` | no force-push / delete |
| `portaki-modules: main integration` | `main` | PR + `quality` (strict) |

**Bypass**: **OrganizationAdmin** (direct push). Optional CI App via `CI_APP_ID`.

```bash
GITHUB_TOKEN=… node .github/scripts/configure-repo-rulesets.mjs
# optional: CI_APP_ID=… node …
```
