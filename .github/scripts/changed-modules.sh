#!/usr/bin/env bash
# Detect which modules/* crates changed vs a git base ref.
# Usage: changed-modules.sh <base-sha>
# Writes GitHub Actions outputs: modules (JSON array), any (true|false), reason.
set -euo pipefail

BASE_REF="${1:-}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

list_all_modules() {
  find modules -mindepth 1 -maxdepth 1 -type d -print | sed 's|^modules/||' | sort
}

emit() {
  local modules_json="$1"
  local any="$2"
  local reason="$3"
  if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
    {
      echo "modules=${modules_json}"
      echo "any=${any}"
      echo "reason=${reason}"
    } >>"$GITHUB_OUTPUT"
  fi
  echo "changed-modules: any=${any} reason=${reason} modules=${modules_json}"
}

ALL_JSON="$(list_all_modules | jq -R . | jq -s -c .)"

# Missing / zero base → full rebuild (force-push, first commit, shallow miss).
if [[ -z "$BASE_REF" || "$BASE_REF" == "0000000000000000000000000000000000000000" ]]; then
  emit "$ALL_JSON" "true" "missing-base"
  exit 0
fi

if ! git cat-file -e "${BASE_REF}^{commit}" 2>/dev/null; then
  emit "$ALL_JSON" "true" "base-not-found"
  exit 0
fi

CHANGED_FILE="$(mktemp)"
trap 'rm -f "$CHANGED_FILE"' EXIT
git diff --name-only "${BASE_REF}"...HEAD >"$CHANGED_FILE" || true

if [[ ! -s "$CHANGED_FILE" ]]; then
  emit "[]" "false" "no-diff"
  exit 0
fi

# Workspace / toolchain / lock → rebuild every module.
# CI workflow/script-only edits must NOT fan out the wasm/publish matrix (minutes).
SHARED_REGEX='^(Cargo\.toml|Cargo\.lock|\.cargo/|rust-toolchain|rust-toolchain\.toml)'
while IFS= read -r path; do
  [[ -z "$path" ]] && continue
  if [[ "$path" =~ $SHARED_REGEX ]]; then
    emit "$ALL_JSON" "true" "shared:${path}"
    exit 0
  fi
done <"$CHANGED_FILE"

MODULES_FILE="$(mktemp)"
trap 'rm -f "$CHANGED_FILE" "$MODULES_FILE"' EXIT
: >"$MODULES_FILE"

while IFS= read -r path; do
  [[ -z "$path" ]] && continue
  if [[ "$path" =~ ^modules/([^/]+)/ ]]; then
    echo "${BASH_REMATCH[1]}" >>"$MODULES_FILE"
  fi
done <"$CHANGED_FILE"

if [[ ! -s "$MODULES_FILE" ]]; then
  emit "[]" "false" "non-module-paths"
  exit 0
fi

# Keep only known module dirs, unique + sorted.
MODULES_JSON="$(
  sort -u "$MODULES_FILE" | while IFS= read -r name; do
    if [[ -d "modules/${name}" ]]; then
      echo "$name"
    fi
  done | jq -R . | jq -s -c .
)"

if [[ "$MODULES_JSON" == "[]" ]]; then
  emit "[]" "false" "non-module-paths"
  exit 0
fi

emit "$MODULES_JSON" "true" "module-paths"
