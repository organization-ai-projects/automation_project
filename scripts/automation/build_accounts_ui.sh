#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
UI_DIR="$ROOT_DIR/projects/products/accounts/ui"
OUT_DIR="$UI_DIR/ui_dist"

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

cp "$UI_DIR/ui_manifest.ron" "$OUT_DIR/ui_manifest.ron"

echo "Accounts UI bundle generated in $OUT_DIR"
