#!/usr/bin/env bash
# Enforce strict workspace library dependency boundaries.
# Rules:
# - library -> product is forbidden
# - L0 -> no workspace dependencies
# - L1 -> L0 only
# - L2 -> L1 only
# - L3 -> L2 only
# - no upward or lateral edges by default

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$REPO_ROOT"

MAP_FILE="scripts/checks/layer_map.txt"
WHITELIST_FILE="scripts/checks/layer_whitelist.txt"
strict_mode="false"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --strict)
      strict_mode="true"
      shift
      ;;
    *)
      echo "Unknown option: $1" >&2
      echo "Usage: ./scripts/checks/check_layer_boundaries.sh [--strict]" >&2
      exit 2
      ;;
  esac
done

if ! command -v cargo >/dev/null 2>&1; then
  echo "❌ cargo is required to run layer boundary checks."
  exit 1
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "❌ jq is required to run layer boundary checks."
  exit 1
fi
if [[ "$strict_mode" == "true" ]] && [[ ! -f "$MAP_FILE" ]]; then
  echo "❌ missing map file: $MAP_FILE"
  exit 1
fi

tmpdir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmpdir"
}
trap cleanup EXIT

edges_tsv="$tmpdir/edges.tsv"
pkg_tsv="$tmpdir/packages.tsv"
layer_tsv="$tmpdir/layers.tsv"
whitelist_tsv="$tmpdir/whitelist.tsv"

if [[ "$strict_mode" == "true" ]]; then
  awk '
    BEGIN { FS="=" }
    /^[[:space:]]*#/ { next }
    /^[[:space:]]*$/ { next }
    {
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", $1)
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2)
      if ($2 !~ /^(L0|L1|L2|L3|UNMAPPED)$/) {
        printf "invalid layer in map: %s=%s\n", $1, $2 > "/dev/stderr"
        exit 2
      }
      print $1 "\t" $2
    }
  ' "$MAP_FILE" | sort -u > "$layer_tsv"

  if [[ -f "$WHITELIST_FILE" ]]; then
    awk '
      BEGIN { FS="\\|" }
      /^[[:space:]]*#/ { next }
      /^[[:space:]]*$/ { next }
      {
        if (NF != 4) {
          printf "invalid whitelist entry (expected 4 fields): %s\n", $0 > "/dev/stderr"
          exit 2
        }
        edge=$1
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", edge)
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2)
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", $3)
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", $4)
        if (edge !~ /^[a-zA-Z0-9_-]+->[a-zA-Z0-9_-]+$/ || $2 == "" || $3 == "" || $4 == "") {
          printf "invalid whitelist entry content: %s\n", $0 > "/dev/stderr"
          exit 2
        }
        split(edge, pair, "->")
        print pair[1] "\t" pair[2]
      }
    ' "$WHITELIST_FILE" | sort -u > "$whitelist_tsv"
  else
    : > "$whitelist_tsv"
  fi
else
  : > "$layer_tsv"
  : > "$whitelist_tsv"
fi

metadata_json="$tmpdir/metadata.json"
cargo metadata --format-version 1 --all-features > "$metadata_json"

jq -r '
  .packages[]
  | [
      .id,
      .name,
      .manifest_path,
      (if (.manifest_path|test("/projects/libraries/")) then "library"
       elif (.manifest_path|test("/projects/products/")) then "product"
       else "other" end)
    ]
  | @tsv
' "$metadata_json" > "$pkg_tsv"

jq -r '
  (.workspace_members | map({(.): true}) | add) as $ws
  | (.packages | map({(.id): {name: .name, manifest: .manifest_path}}) | add) as $pkg
  | .resolve.nodes[]
  | select($ws[.id] == true)
  | . as $node
  | $node.deps[]?
  | select($ws[.pkg] == true)
  | (
      [(.dep_kinds[]?.kind // "normal")]
      | if length == 0 then ["normal"] else . end
    ) as $kinds
  | select(any($kinds[]; . != "dev"))
  | [
      ($pkg[$node.id].name // ""),
      ($pkg[$node.id].manifest // ""),
      ($pkg[.pkg].name // ""),
      ($pkg[.pkg].manifest // "")
    ]
  | @tsv
' "$metadata_json" | sort -u > "$edges_tsv"

layer_of() {
  local crate="$1"
  awk -F'\t' -v c="$crate" '$1==c{print $2; found=1; exit} END{if(!found) print "UNMAPPED"}' "$layer_tsv"
}

rank_of() {
  case "$1" in
    L0) echo 0 ;;
    L1) echo 1 ;;
    L2) echo 2 ;;
    L3) echo 3 ;;
    *) echo -1 ;;
  esac
}

is_whitelisted() {
  local from="$1"
  local to="$2"
  awk -F'\t' -v f="$from" -v t="$to" '$1==f && $2==t{found=1; exit} END{exit(found?0:1)}' "$whitelist_tsv"
}

library_to_product="$tmpdir/library_to_product.tsv"
foundation_internal="$tmpdir/foundation_internal.tsv"
lateral="$tmpdir/lateral.tsv"
upward="$tmpdir/upward.tsv"
non_adjacent="$tmpdir/non_adjacent.tsv"
unmapped="$tmpdir/unmapped.tsv"
whitelist_used="$tmpdir/whitelist_used.tsv"

: > "$library_to_product"
: > "$foundation_internal"
: > "$lateral"
: > "$upward"
: > "$non_adjacent"
: > "$unmapped"
: > "$whitelist_used"

while IFS=$'\t' read -r from_name from_manifest to_name to_manifest; do
  from_kind="other"
  to_kind="other"
  [[ "$from_manifest" == *"/projects/libraries/"* ]] && from_kind="library"
  [[ "$from_manifest" == *"/projects/products/"* ]] && from_kind="product"
  [[ "$to_manifest" == *"/projects/libraries/"* ]] && to_kind="library"
  [[ "$to_manifest" == *"/projects/products/"* ]] && to_kind="product"

  if [[ "$from_kind" == "library" && "$to_kind" == "product" ]]; then
    printf "%s\t%s\n" "$from_name" "$to_name" >> "$library_to_product"
    continue
  fi

  if [[ "$strict_mode" != "true" || "$from_kind" != "library" || "$to_kind" != "library" ]]; then
    continue
  fi

  if is_whitelisted "$from_name" "$to_name"; then
    printf "%s\t%s\n" "$from_name" "$to_name" >> "$whitelist_used"
    continue
  fi

  from_layer="$(layer_of "$from_name")"
  to_layer="$(layer_of "$to_name")"
  from_rank="$(rank_of "$from_layer")"
  to_rank="$(rank_of "$to_layer")"

  if [[ "$from_rank" -lt 0 || "$to_rank" -lt 0 ]]; then
    printf "%s\t%s\t%s\t%s\n" "$from_name" "$from_layer" "$to_name" "$to_layer" >> "$unmapped"
    continue
  fi
  if [[ "$from_layer" == "L0" ]]; then
    printf "%s\t%s\t%s\t%s\n" "$from_name" "$from_layer" "$to_name" "$to_layer" >> "$foundation_internal"
    continue
  fi
  if [[ "$to_rank" -gt "$from_rank" ]]; then
    printf "%s\t%s\t%s\t%s\n" "$from_name" "$from_layer" "$to_name" "$to_layer" >> "$upward"
    continue
  fi
  if [[ "$to_rank" -eq "$from_rank" ]]; then
    printf "%s\t%s\t%s\t%s\n" "$from_name" "$from_layer" "$to_name" "$to_layer" >> "$lateral"
    continue
  fi
  if [[ "$to_rank" -ne $((from_rank - 1)) ]]; then
    printf "%s\t%s\t%s\t%s\n" "$from_name" "$from_layer" "$to_name" "$to_layer" >> "$non_adjacent"
    continue
  fi
done < "$edges_tsv"

sort -u "$library_to_product" -o "$library_to_product"
sort -u "$foundation_internal" -o "$foundation_internal"
sort -u "$lateral" -o "$lateral"
sort -u "$upward" -o "$upward"
sort -u "$non_adjacent" -o "$non_adjacent"
sort -u "$unmapped" -o "$unmapped"
sort -u "$whitelist_used" -o "$whitelist_used"

count_lines() {
  local file="$1"
  [[ -s "$file" ]] && wc -l < "$file" | tr -d ' ' || echo 0
}

c_library_to_product="$(count_lines "$library_to_product")"
c_foundation_internal=0
c_lateral=0
c_upward=0
c_non_adjacent=0
c_unmapped=0
c_whitelist_used=0
if [[ "$strict_mode" == "true" ]]; then
  c_foundation_internal="$(count_lines "$foundation_internal")"
  c_lateral="$(count_lines "$lateral")"
  c_upward="$(count_lines "$upward")"
  c_non_adjacent="$(count_lines "$non_adjacent")"
  c_unmapped="$(count_lines "$unmapped")"
  c_whitelist_used="$(count_lines "$whitelist_used")"
fi

if [[ "$strict_mode" == "true" ]]; then
  echo "Checking strict workspace layer dependency boundaries..."
  echo "- map file: $MAP_FILE"
  echo "- whitelist file: $WHITELIST_FILE"
  echo "- whitelist edges applied: $c_whitelist_used"
else
  echo "Checking workspace layer dependency boundaries (legacy mode)..."
fi

print_violations() {
  local class_name="$1"
  local title="$2"
  local file="$3"
  local suggestion="$4"
  local format="${5:-pair}"
  if [[ ! -s "$file" ]]; then
    return 0
  fi
  echo ""
  echo "❌ $title [class=$class_name]"
  if [[ "$format" == "edge4" ]]; then
    awk -F'\t' -v class_name="$class_name" -v suggestion="$suggestion" '
      {
        print "   - VIOLATION class=" class_name " edge=" $1 "(" $2 ")->" $3 "(" $4 ") suggestion=\"" suggestion "\""
      }
    ' "$file"
  else
    awk -F'\t' -v class_name="$class_name" -v suggestion="$suggestion" '
      {
        print "   - VIOLATION class=" class_name " edge=" $1 "->" $2 " suggestion=\"" suggestion "\""
      }
    ' "$file"
  fi
}

print_violations \
  "library-to-product" \
  "Forbidden dependencies (library -> product)" \
  "$library_to_product" \
  "Move shared code to projects/libraries and invert dependency direction (product -> library)."
if [[ "$strict_mode" == "true" ]]; then
  print_violations \
    "foundation-internal" \
    "Foundation internal dependencies (L0 must have none)" \
    "$foundation_internal" \
    "Move shared logic downward (or split crate responsibilities) so L0 has no workspace dependency." \
    "edge4"
  print_violations \
    "lateral" \
    "Lateral dependencies (forbidden by default)" \
    "$lateral" \
    "Extract shared contract/adapter to lower layer, then depend on that lower layer only." \
    "edge4"
  print_violations \
    "upward" \
    "Upward dependencies (forbidden)" \
    "$upward" \
    "Invert dependency: move needed abstraction downward and keep higher layer free of upward coupling." \
    "edge4"
  print_violations \
    "non-adjacent" \
    "Non-adjacent dependencies (forbidden)" \
    "$non_adjacent" \
    "Route dependency through immediate lower layer (adjacent-only), introducing adapter if needed." \
    "edge4"
  print_violations \
    "unmapped" \
    "Edges with unmapped crate layer(s)" \
    "$unmapped" \
    "Map all involved crates in scripts/checks/layer_map.txt before enforcing strict boundaries." \
    "edge4"
fi

total=$((c_library_to_product + c_foundation_internal + c_lateral + c_upward + c_non_adjacent + c_unmapped))

if [[ "$total" -eq 0 ]]; then
  echo "✅ Layer boundaries OK: no forbidden dependencies found."
  exit 0
fi

echo ""
echo "Summary:"
echo "- library->product: $c_library_to_product"
if [[ "$strict_mode" == "true" ]]; then
  echo "- foundation-internal: $c_foundation_internal"
  echo "- lateral: $c_lateral"
  echo "- upward: $c_upward"
  echo "- non-adjacent: $c_non_adjacent"
  echo "- unmapped: $c_unmapped"
fi
echo ""
echo "How to fix:"
if [[ "$strict_mode" == "true" ]]; then
  echo "  1. Move shared logic to the immediate lower layer."
  echo "  2. Remove lateral/upward couplings by introducing proper adapters."
  echo "  3. Add governed temporary whitelist entries only when strictly necessary."
else
  echo "  1. Move shared code to projects/libraries/."
  echo "  2. Invert dependency direction (product -> library)."
  echo "  3. Remove product-level coupling from library crates."
fi

exit 1
