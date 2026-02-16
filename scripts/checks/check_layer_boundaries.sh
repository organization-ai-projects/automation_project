#!/usr/bin/env bash
# Check workspace dependency boundaries between architecture layers.
# Current enforced rule:
# - Crates under projects/libraries/ must not depend on crates under projects/products/

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

if ! command -v cargo >/dev/null 2>&1; then
  echo "❌ cargo is required to run layer boundary checks."
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "❌ jq is required to run layer boundary checks."
  exit 1
fi

echo "Checking workspace layer dependency boundaries..."

metadata_json="$(cargo metadata --format-version 1 --all-features)"

violations="$(
  jq -r '
    def layer_of($manifest_path):
      if ($manifest_path | test("/projects/libraries/")) then "library"
      elif ($manifest_path | test("/projects/products/")) then "product"
      else "other"
      end;

    ( .workspace_members | map({(.): true}) | add ) as $workspace
    | ( .packages
        | map({
            (.id): {
              name: .name,
              layer: layer_of(.manifest_path)
            }
          })
        | add ) as $pkg_by_id
    | [
        .resolve.nodes[]?
        | select($workspace[.id] == true)
        | . as $node
        | $pkg_by_id[$node.id] as $from
        | $node.deps[]?
        | select($workspace[.pkg] == true)
        | $pkg_by_id[.pkg] as $to
        | select($from.layer == "library" and $to.layer == "product")
        | "\($from.name)\t\($to.name)"
      ]
    | unique
    | .[]
  ' <<< "$metadata_json"
)"

if [[ -z "$violations" ]]; then
  echo "✅ Layer boundaries OK: no forbidden dependencies found."
  exit 0
fi

echo "❌ Forbidden dependencies detected (library -> product):"
while IFS= read -r line; do
  [[ -z "$line" ]] && continue
  from_crate="${line%%$'\t'*}"
  to_crate="${line#*$'\t'}"
  echo "   - $from_crate -> $to_crate"
done <<< "$violations"

echo ""
echo "How to fix:"
echo "  1. Move shared code to projects/libraries/"
echo "  2. Invert the dependency direction (product depends on library)"
echo "  3. Remove product-level coupling from library crates"

exit 1
