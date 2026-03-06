#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# Change-footprint helpers extracted from generate_pr_description.sh.

pr_footprint_print_crate_group() {
  local max_items_per_category="$1"
  shift
  local arr=("$@")
  local i crate crate_lines="" crate_count crate_name shown=0
  declare -A crate_counts=()

  for i in "${arr[@]}"; do
    crate="$(pr_infer_crate_from_path "$i")"
    if [[ -z "$crate" ]]; then
      crate="(unresolved crate)"
    fi
    crate_counts["$crate"]=$((${crate_counts["$crate"]:-0} + 1))
  done

  while IFS= read -r crate; do
    [[ -z "$crate" ]] && continue
    crate_lines+="${crate_counts[$crate]}"$'\t'"${crate}"$'\n'
  done < <(printf '%s\n' "${!crate_counts[@]}" | LC_ALL=C sort)

  while IFS=$'\t' read -r crate_count crate_name; do
    [[ -z "${crate_name:-}" ]] && continue
    echo "  - ${crate_name} (${crate_count} files)"
    shown=$((shown + 1))
    if [[ "$shown" -ge "$max_items_per_category" ]]; then
      break
    fi
  done < <(printf '%s' "$crate_lines" | LC_ALL=C sort -t$'\t' -k1,1nr -k2,2)
}

pr_footprint_print_group() {
  local label="$1"
  local max_items_per_category="$2"
  shift 2
  local arr=("$@")
  local i
  local total="${#arr[@]}"
  local count=0

  [[ "$total" -eq 0 ]] && return 1
  echo "- ${label} (${total})"

  if [[ "$label" == "Crates" && "$total" -gt "$max_items_per_category" ]]; then
    # For large Rust changesets, aggregate by crate for readability.
    pr_footprint_print_crate_group "$max_items_per_category" "${arr[@]}"
    return 0
  fi

  for i in "${arr[@]}"; do
    echo "  - ${i}"
    count=$((count + 1))
    if [[ "$count" -ge "$max_items_per_category" ]]; then
      break
    fi
  done
  return 0
}

pr_emit_change_footprint() {
  local range="$1"
  local files
  local file
  local any=0
  local max_items_per_category=12
  declare -a doc_files=()
  declare -a shell_files=()
  declare -a crate_files=()
  declare -a workspace_files=()
  declare -a other_files=()

  files="$(git diff --name-only "$range" 2>/dev/null || true)"
  if [[ -z "$files" ]]; then
    echo "- No changed files detected for this branch range."
    return
  fi

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    case "$file" in
    *.md | documentation/* | .github/documentation/*)
      doc_files+=("$file")
      ;;
    *.sh | scripts/*)
      shell_files+=("$file")
      ;;
    Cargo.toml | Cargo.lock | rust-toolchain* | .cargo/* | **/Cargo.toml | **/Cargo.lock)
      workspace_files+=("$file")
      ;;
    *.rs | */src/*)
      crate_files+=("$file")
      ;;
    *)
      other_files+=("$file")
      ;;
    esac
  done <<<"$files"

  pr_footprint_print_group "Documentation" "$max_items_per_category" "${doc_files[@]}" && any=1
  pr_footprint_print_group "Shell" "$max_items_per_category" "${shell_files[@]}" && any=1
  pr_footprint_print_group "Crates" "$max_items_per_category" "${crate_files[@]}" && any=1
  pr_footprint_print_group "Workspace" "$max_items_per_category" "${workspace_files[@]}" && any=1
  pr_footprint_print_group "Other" "$max_items_per_category" "${other_files[@]}" && any=1

  if [[ "$any" -eq 0 ]]; then
    echo "- No changed files detected for this branch range."
  fi
}
