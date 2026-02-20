#!/usr/bin/env bash

# Shared scope/crate resolver helpers used by hooks and automation scripts.

# shellcheck source=scripts/common_lib/automation/workspace_rust.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/automation/workspace_rust.sh"
# shellcheck source=scripts/common_lib/automation/non_workspace_rust.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/automation/non_workspace_rust.sh"

resolve_scope_from_path() {
  local file="$1"
  local scope=""

  # Workspace members are the primary source of truth for crate boundaries.
  if [[ "$file" =~ ^projects/(libraries|products)/ ]]; then
    if scope="$(workspace_rust_resolve_scope_from_members "$file")"; then
      printf '%s\n' "$scope"
      return 0
    fi
  fi

  # Fallback when workspace mapping cannot resolve the file.
  non_workspace_rust_resolve_scope_fallback "$file"
}

collect_scopes_from_files() {
  local files="$1"
  local -a scopes=()
  local file

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    local scope=""
    if scope="$(resolve_scope_from_path "$file")"; then
      if [[ ! " ${scopes[*]} " =~ " ${scope} " ]]; then
        scopes+=("$scope")
      fi
    fi
  done <<< "$files"

  printf '%s\n' "${scopes[@]+"${scopes[@]}"}"
}

detect_required_scopes_from_staged_files() {
  local files
  files="$(git diff --cached --name-only --diff-filter=ACMRUD)"
  collect_scopes_from_files "$files"
}

resolve_crate_name_from_file() {
  local file="$1"
  local dir

  if [[ -d "$file" ]]; then
    dir="$file"
  else
    dir="$(dirname "$file")"
  fi

  while [[ "$dir" != "." && "$dir" != "/" ]]; do
    local cargo_toml="${dir}/Cargo.toml"
    if [[ -f "$cargo_toml" ]]; then
      sed -n 's/^name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$cargo_toml" | head -n1
      return 0
    fi
    dir="$(dirname "$dir")"
  done

  return 1
}

collect_crates_from_files() {
  local files="$1"
  local -a crates=()
  local file

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    local crate=""
    if crate="$(resolve_crate_name_from_file "$file")" && [[ -n "$crate" ]]; then
      if [[ ! " ${crates[*]} " =~ " ${crate} " ]]; then
        crates+=("$crate")
      fi
    fi
  done <<< "$files"

  printf '%s\n' "${crates[@]+"${crates[@]}"}"
}
