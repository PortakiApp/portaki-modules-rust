#!/usr/bin/env bash
# Build, lint, test, and publish one module to GHCR.
# Usage: publish-module.sh <module-name>
set -euo pipefail

MODULE="${1:?module name required}"
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
MODULE_DIR="${ROOT}/modules/${MODULE}"
REGISTRY="${REGISTRY:-ghcr.io/portakiapp}"

if [[ ! -d "$MODULE_DIR" ]]; then
  echo "::error::unknown module: ${MODULE}"
  exit 1
fi

version="$(grep -E '^version\s*=' "${MODULE_DIR}/Cargo.toml" | head -1 | sed -E 's/.*"([^"]+)".*/\1/')"

echo "=== publish ${MODULE}:${version} ==="
cd "$MODULE_DIR"

rm -rf "target/wasm32-unknown-unknown/release/build/${MODULE}-"* || true
portaki build --release

WASM="target/wasm32-unknown-unknown/release/${MODULE}.wasm"
if [[ ! -f "$WASM" ]]; then
  WASM="$(find target/wasm32-unknown-unknown/release -maxdepth 1 -name '*.wasm' | head -1)"
fi
if strings "$WASM" | grep -q '__wbindgen'; then
  echo "::error::${MODULE}: wasm contains wasm-bindgen imports"
  exit 1
fi

PUBLISH_MANIFEST=target/portaki/publish-manifest.json
MANIFEST_ID="$(jq -r '.id' "$PUBLISH_MANIFEST")"
MANIFEST_VERSION="$(jq -r '.version' "$PUBLISH_MANIFEST")"
if [[ "$MANIFEST_ID" != "$MODULE" ]]; then
  echo "::error::${MODULE}: publish-manifest id=${MANIFEST_ID}"
  exit 1
fi
if [[ "$MANIFEST_VERSION" != "$version" ]]; then
  echo "::error::${MODULE}: Cargo.toml version=${version} manifest=${MANIFEST_VERSION}"
  exit 1
fi

portaki lint
cargo test --lib
cargo test --test integration

PORTAKI_PUBLISH_VERSION="$version" portaki publish --registry "$REGISTRY"
portaki publish --registry "$REGISTRY" --dry-run

package_name="portaki-modules-${MODULE}"
package_visibility="$(gh api "orgs/PortakiApp/packages/container/${package_name}" --jq '.visibility' 2>/dev/null || echo unknown)"
echo "GHCR ${package_name} visibility: ${package_visibility}"
if [[ "${package_visibility}" != "public" ]]; then
  echo "Setting ${package_name} visibility to public..."
  if gh api --method PATCH "orgs/PortakiApp/packages/container/${package_name}" \
    --field visibility=public; then
    echo "Package ${package_name} is now public"
  else
    echo "::warning::Could not set ${package_name} visibility via API (404) — set public manually: https://github.com/orgs/PortakiApp/packages/container/package/${package_name}/settings"
  fi
fi
