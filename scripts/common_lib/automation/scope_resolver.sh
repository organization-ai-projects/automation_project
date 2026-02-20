#!/usr/bin/env bash

# Shared scope/crate resolver helpers used by hooks and automation scripts.

# shellcheck source=scripts/common_lib/automation/file_types.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/automation/file_types.sh"
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

collect_format_categories_from_files() {
  local files="$1"
  local -a categories=()
  local file

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue

    if resolve_scope_from_path "$file" >/dev/null 2>&1; then
      [[ ! " ${categories[*]} " =~ " rust " ]] && categories+=("rust")
      continue
    fi

    if is_shell_path_file "$file"; then
      [[ ! " ${categories[*]} " =~ " shell " ]] && categories+=("shell")
      continue
    fi

    if is_markdown_path_file "$file"; then
      [[ ! " ${categories[*]} " =~ " markdown " ]] && categories+=("markdown")
      continue
    fi

    [[ ! " ${categories[*]} " =~ " other " ]] && categories+=("other")
  done <<< "$files"

  printf '%s\n' "${categories[@]+"${categories[@]}"}"
}

resolve_common_path_scope_from_files() {
  local files="$1"
  local file
  local dir
  local common=""

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue

    if [[ "$file" == */* ]]; then
      dir="${file%/*}"
    else
      dir="."
    fi

    if [[ -z "$common" ]]; then
      common="$dir"
      continue
    fi

    while [[ "$dir" != "$common" && "$dir" != "$common/"* ]]; do
      if [[ "$common" == "." ]]; then
        break
      fi
      common="$(dirname "$common")"
    done
  done <<< "$files"

  [[ -z "$common" ]] && return 1
  if [[ "$common" == "." ]]; then
    printf 'workspace\n'
    return 0
  fi

  printf '%s\n' "$common"
}

detect_required_scopes_from_staged_files() {
  local files
  local scopes
  local only_shell=1
  local only_markdown=1
  local file
  files="$(git diff --cached --name-only --diff-filter=ACMRUD)"
  scopes="$(collect_scopes_from_files "$files")"
  if [[ -n "$scopes" ]]; then
    printf '%s\n' "$scopes"
    return 0
  fi

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    if ! is_shell_path_file "$file"; then
      only_shell=0
    fi
    if ! is_markdown_path_file "$file"; then
      only_markdown=0
    fi
  done <<< "$files"

  if [[ $only_shell -eq 1 && -n "$files" ]]; then
    printf 'shell\n'
    return 0
  fi

  if [[ $only_markdown -eq 1 && -n "$files" ]]; then
    printf 'markdown\n'
    return 0
  fi

  if [[ -n "$files" ]]; then
    resolve_common_path_scope_from_files "$files"
  fi

  return 0
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
