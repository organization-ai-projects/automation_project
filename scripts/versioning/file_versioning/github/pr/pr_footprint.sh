#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# Change-footprint helpers extracted from generate_pr_description.sh.

pr_infer_crate_from_path() {
  local rel_path="$1"
  local dir candidate product_root

  if [[ "$rel_path" == *"/src/"* ]]; then
    candidate="${rel_path%%/src/*}"
    if [[ -f "${candidate}/Cargo.toml" ]]; then
      echo "$candidate"
      return
    fi
  fi

  # Temporary legacy fallback (generic for any product):
  # if Rust paths still come from product root (src/tests) while the product
  # is already split into backend/ui crates, attribute them to backend crate.
  # Remove this fallback once root-level src/tests paths are fully migrated.
  if [[ "$rel_path" =~ ^(projects/products/(stable|unstable)/[^/]+)/(src|tests)/ ]]; then
    product_root="${BASH_REMATCH[1]}"
    if [[ -f "${product_root}/backend/Cargo.toml" ]]; then
      echo "${product_root}/backend"
      return
    fi
  fi

  if [[ "$rel_path" == */Cargo.toml ]]; then
    dir="${rel_path%/Cargo.toml}"
    if [[ -n "$dir" ]]; then
      echo "$dir"
      return
    fi
  fi

  dir="$(dirname "$rel_path")"
  while [[ "$dir" != "." && "$dir" != "/" ]]; do
    if [[ -f "${dir}/Cargo.toml" ]]; then
      echo "$dir"
      return
    fi
    dir="$(dirname "$dir")"
  done
}

pr_emit_change_footprint() {
  local range="$1"
  local files
  local file
  local any=0
  local max_items_per_category=12
  local count=0
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

  print_group() {
    local label="$1"
    shift
    local arr=("$@")
    local i
    local total="${#arr[@]}"
    [[ "$total" -eq 0 ]] && return
    any=1
    echo "- ${label} (${total})"

    if [[ "$label" == "Crates" && "$total" -gt "$max_items_per_category" ]]; then
      # For large Rust changesets, aggregate by crate for readability.
      declare -A crate_counts=()
      local crate=""
      local crate_lines=""
      local shown=0
      local crates_total=0
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
        crates_total=$((crates_total + 1))
      done < <(printf '%s\n' "${!crate_counts[@]}" | LC_ALL=C sort)

      while IFS=$'\t' read -r crate_count crate_name; do
        [[ -z "${crate_name:-}" ]] && continue
        echo "  - ${crate_name} (${crate_count} files)"
        shown=$((shown + 1))
        if [[ "$shown" -ge "$max_items_per_category" ]]; then
          break
        fi
      done < <(printf '%s' "$crate_lines" | LC_ALL=C sort -t$'\t' -k1,1nr -k2,2)
      return
    fi

    count=0
    for i in "${arr[@]}"; do
      echo "  - ${i}"
      count=$((count + 1))
      if [[ "$count" -ge "$max_items_per_category" ]]; then
        break
      fi
    done
  }

  print_group "Documentation" "${doc_files[@]}"
  print_group "Shell" "${shell_files[@]}"
  print_group "Crates" "${crate_files[@]}"
  print_group "Workspace" "${workspace_files[@]}"
  print_group "Other" "${other_files[@]}"

  if [[ "$any" -eq 0 ]]; then
    echo "- No changed files detected for this branch range."
  fi
}
