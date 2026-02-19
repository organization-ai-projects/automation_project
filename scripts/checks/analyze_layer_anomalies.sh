#!/usr/bin/env bash
# Semi-automated analysis for strict adjacent-only library layering.
# Produces a human-readable report and optional JSON export.

set -euo pipefail

usage() {
  cat <<'USAGE'
Usage:
  ./scripts/checks/analyze_layer_anomalies.sh [options]

Options:
  --protocol-layer L1|L2|UNDECIDED   Placement assumption for crate "protocol" (default: L1)
  --map-file PATH                    Optional override map file (format: crate=L0|L1|L2|L3|UNMAPPED)
  --json-out PATH                    Optional JSON report output path
  --fail-on-anomaly true|false       Exit non-zero if anomalies are found (default: false)
  -h, --help                         Show this help

Notes:
  - This script is an analysis aid (semi-automated), not the final blocking checker.
  - Workspace edges are extracted from `cargo metadata`.
  - External crate edges are ignored for layering rules.
USAGE
}

protocol_layer="L1"
map_file=""
json_out=""
fail_on_anomaly="false"
canonical_map_file="scripts/checks/layer_map.txt"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --protocol-layer)
      protocol_layer="${2:-}"
      shift 2
      ;;
    --map-file)
      map_file="${2:-}"
      shift 2
      ;;
    --json-out)
      json_out="${2:-}"
      shift 2
      ;;
    --fail-on-anomaly)
      fail_on_anomaly="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ "$protocol_layer" != "L1" && "$protocol_layer" != "L2" && "$protocol_layer" != "UNDECIDED" ]]; then
  echo "Invalid --protocol-layer: $protocol_layer (expected L1|L2|UNDECIDED)" >&2
  exit 2
fi
if [[ "$fail_on_anomaly" != "true" && "$fail_on_anomaly" != "false" ]]; then
  echo "Invalid --fail-on-anomaly: $fail_on_anomaly (expected true|false)" >&2
  exit 2
fi
if [[ -n "$map_file" && ! -f "$map_file" ]]; then
  echo "Map file not found: $map_file" >&2
  exit 2
fi

if [[ -z "$map_file" && -f "$canonical_map_file" ]]; then
  map_file="$canonical_map_file"
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "❌ cargo is required." >&2
  exit 3
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "❌ jq is required." >&2
  exit 3
fi

tmpdir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmpdir"
}
trap cleanup EXIT

metadata_json="$tmpdir/metadata.json"
cargo metadata --format-version 1 --all-features > "$metadata_json"

pkg_tsv="$tmpdir/packages.tsv"
edges_tsv="$tmpdir/edges.tsv"
lib_tsv="$tmpdir/libraries.tsv"
layer_tsv="$tmpdir/layers.tsv"
override_tsv="$tmpdir/overrides.tsv"

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
      $node.id,
      ($pkg[$node.id].name // ""),
      ($pkg[$node.id].manifest // ""),
      .pkg,
      ($pkg[.pkg].name // ""),
      ($pkg[.pkg].manifest // "")
    ]
  | @tsv
' "$metadata_json" | sort -u > "$edges_tsv"

awk -F'\t' '$4=="library"{print $2 "\t" $3}' "$pkg_tsv" | sort -u > "$lib_tsv"

if [[ -n "$map_file" ]]; then
  awk '
    BEGIN { FS="=" }
    /^[[:space:]]*#/ { next }
    /^[[:space:]]*$/ { next }
    {
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", $1)
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2)
      if ($2 ~ /^(L0|L1|L2|L3|UNMAPPED)$/) {
        print $1 "\t" $2
      }
    }
  ' "$map_file" > "$override_tsv"
else
  : > "$override_tsv"
fi

if [[ -z "$map_file" ]]; then
  echo "⚠️ Using built-in provisional mapping. Prefer scripts/checks/layer_map.txt or --map-file for canonical runs." >&2
else
  echo "ℹ️ Using map file: $map_file" >&2
fi

default_layer_for() {
  local crate="$1"
  case "$crate" in
    # L0 Foundation candidates
    ast_core|ast_macros|common_binary|common_calendar|common_parsing|common_time|common_tokenize|hybrid_arena|pjson_proc_macros|protocol_macros)
      echo "L0"
      ;;
    # L1 Technical specialization candidates
    command_runner|common|common_json|common_ron)
      echo "L1"
      ;;
    # L2 Domain candidates
    identity|neural|security|symbolic|ui|ui-lib|versioning)
      echo "L2"
      ;;
    # L3 Orchestration candidate
    ai)
      echo "L3"
      ;;
    protocol)
      if [[ "$protocol_layer" == "UNDECIDED" ]]; then
        echo "L1"
      else
        echo "$protocol_layer"
      fi
      ;;
    *)
      echo "UNMAPPED"
      ;;
  esac
}

while IFS=$'\t' read -r crate _manifest; do
  layer="$(default_layer_for "$crate")"
  # Override when provided
  override="$(awk -F'\t' -v c="$crate" '$1==c{print $2; exit}' "$override_tsv" || true)"
  if [[ -n "$override" ]]; then
    layer="$override"
  fi
  printf "%s\t%s\n" "$crate" "$layer"
done < "$lib_tsv" | sort -u > "$layer_tsv"

layer_of() {
  local crate="$1"
  awk -F'\t' -v c="$crate" '$1==c{print $2; found=1; exit} END{if(!found) print "UNMAPPED"}' "$layer_tsv"
}

layer_rank() {
  local layer="$1"
  case "$layer" in
    L0) echo 0 ;;
    L1) echo 1 ;;
    L2) echo 2 ;;
    L3) echo 3 ;;
    *) echo -1 ;;
  esac
}

library_to_product="$tmpdir/library_to_product.tsv"
lateral_edges="$tmpdir/lateral.tsv"
upward_edges="$tmpdir/upward.tsv"
non_adjacent_edges="$tmpdir/non_adjacent.tsv"
foundation_internal_edges="$tmpdir/foundation_internal.tsv"
unknown_edges="$tmpdir/unknown.tsv"
cycle_signals="$tmpdir/cycle_signals.txt"
unmapped_crates="$tmpdir/unmapped.tsv"
mixed_profile="$tmpdir/mixed_profile.tsv"

: > "$library_to_product"
: > "$lateral_edges"
: > "$upward_edges"
: > "$non_adjacent_edges"
: > "$foundation_internal_edges"
: > "$unknown_edges"
: > "$cycle_signals"
: > "$mixed_profile"

awk -F'\t' '$2=="UNMAPPED"{print $1}' "$layer_tsv" | sort -u > "$unmapped_crates"

while IFS=$'\t' read -r _fid from_name from_manifest _tid to_name to_manifest; do
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

  if [[ "$from_kind" != "library" || "$to_kind" != "library" ]]; then
    continue
  fi

  from_layer="$(layer_of "$from_name")"
  to_layer="$(layer_of "$to_name")"
  from_rank="$(layer_rank "$from_layer")"
  to_rank="$(layer_rank "$to_layer")"

  if [[ "$from_rank" -lt 0 || "$to_rank" -lt 0 ]]; then
    printf "%s\t%s\t%s\t%s\n" "$from_name" "$from_layer" "$to_name" "$to_layer" >> "$unknown_edges"
    continue
  fi

  if [[ "$from_layer" == "L0" ]]; then
    printf "%s\t%s\t%s\t%s\n" "$from_name" "$from_layer" "$to_name" "$to_layer" >> "$foundation_internal_edges"
    continue
  fi

  if [[ "$to_rank" -gt "$from_rank" ]]; then
    printf "%s\t%s\t%s\t%s\n" "$from_name" "$from_layer" "$to_name" "$to_layer" >> "$upward_edges"
    continue
  fi

  if [[ "$to_rank" -eq "$from_rank" ]]; then
    printf "%s\t%s\t%s\t%s\n" "$from_name" "$from_layer" "$to_name" "$to_layer" >> "$lateral_edges"
    continue
  fi

  if [[ "$to_rank" -ne $((from_rank - 1)) ]]; then
    printf "%s\t%s\t%s\t%s\n" "$from_name" "$from_layer" "$to_name" "$to_layer" >> "$non_adjacent_edges"
    continue
  fi
done < "$edges_tsv"

sort -u "$library_to_product" -o "$library_to_product"
sort -u "$lateral_edges" -o "$lateral_edges"
sort -u "$upward_edges" -o "$upward_edges"
sort -u "$non_adjacent_edges" -o "$non_adjacent_edges"
sort -u "$foundation_internal_edges" -o "$foundation_internal_edges"
sort -u "$unknown_edges" -o "$unknown_edges"

# Potential cycle signals from library-only graph (tsort is a quick signal, not full SCC report).
lib_edges_for_tsort="$tmpdir/lib_edges_for_tsort.tsv"
awk -F'\t' '
  $3 ~ /\/projects\/libraries\// && $6 ~ /\/projects\/libraries\// {
    print $2 "\t" $5
  }
' "$edges_tsv" | sort -u > "$lib_edges_for_tsort"

if [[ -s "$lib_edges_for_tsort" ]]; then
  if ! tr '\t' ' ' < "$lib_edges_for_tsort" | tsort >/dev/null 2>"$cycle_signals"; then
    true
  fi
fi

# Mixed profile signal: crates with both product consumers and library consumers.
stats_tsv="$tmpdir/mixed_stats.tsv"
awk -F'\t' '
  # library -> library
  $3 ~ /\/projects\/libraries\// && $6 ~ /\/projects\/libraries\// {
    out_pair = $2 SUBSEP $5
    if (!(out_pair in seen_out)) {
      seen_out[out_pair] = 1
      lib_out[$2]++
    }
    in_pair = $5 SUBSEP $2
    if (!(in_pair in seen_in_lib)) {
      seen_in_lib[in_pair] = 1
      lib_in[$5]++
    }
  }
  # product -> library
  $3 ~ /\/projects\/products\// && $6 ~ /\/projects\/libraries\// {
    p_pair = $5 SUBSEP $2
    if (!(p_pair in seen_in_prod)) {
      seen_in_prod[p_pair] = 1
      prod_in[$5]++
    }
  }
  END {
    for (c in prod_in) {
      printf "%s\t%d\t%d\t%d\n", c, prod_in[c], lib_in[c] + 0, lib_out[c] + 0
    }
    for (c in lib_in) {
      if (!(c in prod_in)) {
        printf "%s\t%d\t%d\t%d\n", c, 0, lib_in[c], lib_out[c] + 0
      }
    }
    for (c in lib_out) {
      if (!(c in prod_in) && !(c in lib_in)) {
        printf "%s\t%d\t%d\t%d\n", c, 0, 0, lib_out[c]
      }
    }
  }
' "$edges_tsv" | sort -u > "$stats_tsv"

while IFS=$'\t' read -r crate prod_in lib_in lib_out; do
  if [[ "$prod_in" -gt 0 && "$lib_in" -gt 0 && "$lib_out" -gt 0 ]]; then
    printf "%s\tproduct_in=%s\tlibrary_in=%s\tlibrary_out=%s\n" "$crate" "$prod_in" "$lib_in" "$lib_out" >> "$mixed_profile"
  fi
done < "$stats_tsv"
sort -u "$mixed_profile" -o "$mixed_profile"

count_file_lines() {
  local f="$1"
  if [[ -s "$f" ]]; then
    wc -l < "$f" | tr -d ' '
  else
    echo 0
  fi
}

c_library_to_product="$(count_file_lines "$library_to_product")"
c_lateral="$(count_file_lines "$lateral_edges")"
c_upward="$(count_file_lines "$upward_edges")"
c_non_adjacent="$(count_file_lines "$non_adjacent_edges")"
c_foundation_internal="$(count_file_lines "$foundation_internal_edges")"
c_unknown="$(count_file_lines "$unknown_edges")"
c_unmapped="$(count_file_lines "$unmapped_crates")"
c_mixed="$(count_file_lines "$mixed_profile")"

echo "=== Layer Anomaly Analysis (semi-automated) ==="
echo "Protocol layer assumption: $protocol_layer"
echo "Map overrides: ${map_file:-none}"
echo ""
echo "Library crates by provisional layer:"
awk -F'\t' '{print "- " $1 ": " $2}' "$layer_tsv"
echo ""
echo "Summary:"
echo "- library->product edges: $c_library_to_product"
echo "- foundation internal deps (L0 -> workspace): $c_foundation_internal"
echo "- lateral edges: $c_lateral"
echo "- upward edges: $c_upward"
echo "- non-adjacent edges: $c_non_adjacent"
echo "- edges with UNMAPPED layers: $c_unknown"
echo "- unmapped crates: $c_unmapped"
echo "- mixed consumer profile crates: $c_mixed"
echo ""

print_section() {
  local title="$1"
  local file="$2"
  local format="${3:-pairs}"
  local max="${4:-30}"
  local n
  n="$(count_file_lines "$file")"
  echo "## $title ($n)"
  if [[ "$n" -eq 0 ]]; then
    echo "  none"
    echo ""
    return
  fi
  if [[ "$format" == "edge4" ]]; then
    awk -F'\t' -v max="$max" 'NR<=max{print "  - " $1 " (" $2 ") -> " $3 " (" $4 ")"}' "$file"
  elif [[ "$format" == "pairs" ]]; then
    awk -F'\t' -v max="$max" 'NR<=max{print "  - " $1 " -> " $2}' "$file"
  else
    awk -v max="$max" 'NR<=max{print "  - " $0}' "$file"
  fi
  if [[ "$n" -gt "$max" ]]; then
    echo "  ... +$((n - max)) more"
  fi
  echo ""
}

print_section "library->product (forbidden)" "$library_to_product" "pairs"
print_section "foundation internal dependencies (L0 must have none)" "$foundation_internal_edges" "edge4"
print_section "lateral edges" "$lateral_edges" "edge4"
print_section "upward edges" "$upward_edges" "edge4"
print_section "non-adjacent edges" "$non_adjacent_edges" "edge4"
print_section "edges with UNMAPPED layers" "$unknown_edges" "edge4"
print_section "UNMAPPED crates" "$unmapped_crates" "raw"
print_section "mixed consumer profile (decision hotspots)" "$mixed_profile" "raw"

echo "## potential cycle signals"
if [[ -s "$cycle_signals" ]]; then
  sed 's/^/  /' "$cycle_signals"
else
  echo "  none detected by tsort signal"
fi
echo ""

if [[ -n "$json_out" ]]; then
  jq -n \
    --arg protocol_layer "$protocol_layer" \
    --arg map_file "${map_file:-}" \
    --argjson library_to_product "$(
      jq -R -s '
        split("\n") | map(select(length>0)) |
        map(split("\t")) | map({from: .[0], to: .[1]})
      ' "$library_to_product"
    )" \
    --argjson foundation_internal "$(
      jq -R -s '
        split("\n") | map(select(length>0)) |
        map(split("\t")) | map({from: .[0], from_layer: .[1], to: .[2], to_layer: .[3]})
      ' "$foundation_internal_edges"
    )" \
    --argjson lateral "$(
      jq -R -s '
        split("\n") | map(select(length>0)) |
        map(split("\t")) | map({from: .[0], from_layer: .[1], to: .[2], to_layer: .[3]})
      ' "$lateral_edges"
    )" \
    --argjson upward "$(
      jq -R -s '
        split("\n") | map(select(length>0)) |
        map(split("\t")) | map({from: .[0], from_layer: .[1], to: .[2], to_layer: .[3]})
      ' "$upward_edges"
    )" \
    --argjson non_adjacent "$(
      jq -R -s '
        split("\n") | map(select(length>0)) |
        map(split("\t")) | map({from: .[0], from_layer: .[1], to: .[2], to_layer: .[3]})
      ' "$non_adjacent_edges"
    )" \
    --argjson unknown_edges "$(
      jq -R -s '
        split("\n") | map(select(length>0)) |
        map(split("\t")) | map({from: .[0], from_layer: .[1], to: .[2], to_layer: .[3]})
      ' "$unknown_edges"
    )" \
    --argjson unmapped_crates "$(
      jq -R -s 'split("\n") | map(select(length>0))' "$unmapped_crates"
    )" \
    --argjson mixed_profile "$(
      jq -R -s 'split("\n") | map(select(length>0))' "$mixed_profile"
    )" \
    --arg cycle_signals "$(
      cat "$cycle_signals" 2>/dev/null || true
    )" \
    --argjson provisional_layers "$(
      jq -R -s '
        split("\n") | map(select(length>0)) |
        map(split("\t")) | map({crate: .[0], layer: .[1]})
      ' "$layer_tsv"
    )" \
    '{
      protocol_layer_assumption: $protocol_layer,
      map_file: (if $map_file == "" then null else $map_file end),
      provisional_layers: $provisional_layers,
      anomalies: {
        library_to_product: $library_to_product,
        foundation_internal: $foundation_internal,
        lateral: $lateral,
        upward: $upward,
        non_adjacent: $non_adjacent,
        unknown_edges: $unknown_edges,
        unmapped_crates: $unmapped_crates,
        mixed_profile: $mixed_profile,
        potential_cycle_signals: $cycle_signals
      }
    }' > "$json_out"
  echo "JSON report written to: $json_out"
fi

total_anomalies=$((c_library_to_product + c_foundation_internal + c_lateral + c_upward + c_non_adjacent + c_unknown + c_unmapped))
if [[ "$fail_on_anomaly" == "true" && "$total_anomalies" -gt 0 ]]; then
  exit 1
fi

exit 0
