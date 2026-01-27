#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
UI_DIR="$ROOT_DIR/projects/products/accounts/ui"
OUT_DIR="$UI_DIR/ui_dist"
UI_MANIFEST="ui_manifest.ron"

if ! command -v dx >/dev/null 2>&1; then
  echo "dx (dioxus-cli) not found. Install with: cargo install dioxus-cli" >&2
  exit 1
fi

mkdir -p "$OUT_DIR"

(
  cd "$UI_DIR"
  CARGO_PROFILE_RELEASE_DEBUG=0 RUSTFLAGS="${RUSTFLAGS:-} -C debuginfo=0" \
    dx bundle --release --debug-symbols false --out-dir ui_dist
)

if [[ -f "$UI_DIR/$UI_MANIFEST" ]]; then
  cp "$UI_DIR/$UI_MANIFEST" "$OUT_DIR/$UI_MANIFEST"
fi

echo "Accounts UI bundle generated in $OUT_DIR"
