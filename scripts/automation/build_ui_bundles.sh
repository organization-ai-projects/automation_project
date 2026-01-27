#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
UI_MANIFEST="ui_manifest.ron"

if ! command -v dx >/dev/null 2>&1; then
  echo "dx (dioxus-cli) not found. Install with: cargo install dioxus-cli" >&2
  exit 1
fi

mapfile -t UI_CARGOS < <(find "$ROOT_DIR/projects/products" -type f -path "*/ui/Cargo.toml")

if [[ ${#UI_CARGOS[@]} -eq 0 ]]; then
  echo "No UI crates found under projects/products" >&2
  exit 1
fi

for cargo in "${UI_CARGOS[@]}"; do
  ui_dir="$(dirname "$cargo")"
  if rg -q "dioxus" "$cargo"; then
    echo "Building UI bundle in $ui_dir"
    (
      cd "$ui_dir"
      CARGO_PROFILE_RELEASE_DEBUG=0 RUSTFLAGS="${RUSTFLAGS:-} -C debuginfo=0" \
        dx bundle --release --debug-symbols false --out-dir ui_dist
      if [[ -f "$UI_MANIFEST" ]]; then
        cp "$UI_MANIFEST" "ui_dist/$UI_MANIFEST"
      fi
    )
  else
    echo "Skipping $ui_dir (no dioxus dependency)"
  fi
done

echo "UI bundle build complete"
