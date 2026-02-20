#!/usr/bin/env bash

# Shared scope/crate resolver helpers used by hooks and automation scripts.

scope_resolver_workspace_member_dirs() {
  local repo_root
  repo_root="$(git rev-parse --show-toplevel 2>/dev/null || true)"
  [[ -n "$repo_root" ]] || return 1

  # cargo metadata is the workspace source of truth for member crates.
  # We only extract manifest paths and map them to repo-relative crate dirs.
  cargo metadata --no-deps --format-version 1 2>/dev/null \
    | tr ',' '\n' \
    | sed -n 's/.*"manifest_path":"\([^"]*\)".*/\1/p' \
    | sed 's#\\/#/#g' \
    | while IFS= read -r manifest_path; do
        [[ -z "$manifest_path" ]] && continue
        local crate_dir="${manifest_path%/Cargo.toml}"
        local rel_dir="$crate_dir"
        case "$crate_dir" in
          "$repo_root"/*) rel_dir="${crate_dir#"$repo_root"/}" ;;
        esac
        [[ "$rel_dir" == projects/* ]] && printf '%s\n' "$rel_dir"
      done \
    | sort -u
}

scope_resolver_load_workspace_dirs() {
  if [[ -n "${SCOPE_RESOLVER_WORKSPACE_DIRS_LOADED:-}" ]]; then
    return 0
  fi

  SCOPE_RESOLVER_WORKSPACE_DIRS="$(scope_resolver_workspace_member_dirs || true)"
  SCOPE_RESOLVER_WORKSPACE_DIRS_LOADED=1
}

resolve_scope_from_workspace_members() {
  local file="$1"
  local dir="$file"

  scope_resolver_load_workspace_dirs
  [[ -n "${SCOPE_RESOLVER_WORKSPACE_DIRS:-}" ]] || return 1

  if [[ ! -d "$dir" ]]; then
    dir="$(dirname "$file")"
  fi

  while :; do
    if grep -Fxq "$dir" <<< "$SCOPE_RESOLVER_WORKSPACE_DIRS"; then
      printf '%s\n' "$dir"
      return 0
    fi
    [[ "$dir" == "." || "$dir" == "/" ]] && break
    dir="$(dirname "$dir")"
  done

  return 1
}

resolve_nearest_scope_with_cargo() {
  local file="$1"
  local boundary="$2"
  local dir="$file"

  if [[ ! -d "$dir" ]]; then
    dir="$(dirname "$file")"
  fi

  while :; do
    if [[ -f "${dir}/Cargo.toml" ]]; then
      printf '%s\n' "$dir"
      return 0
    fi

    if [[ "$dir" == "$boundary" || "$dir" == "." || "$dir" == "/" ]]; then
      break
    fi

    dir="$(dirname "$dir")"
  done

  return 1
}

resolve_scope_from_path() {
  local file="$1"
  local scope=""

  # Workspace members are the primary source of truth for crate boundaries.
  if [[ "$file" =~ ^projects/(libraries|products)/ ]]; then
    if scope="$(resolve_scope_from_workspace_members "$file")"; then
      printf '%s\n' "$scope"
      return 0
    fi
  fi

  if [[ "$file" =~ ^projects/products/([^/]+)/([^/]+)/ ]]; then
    local product_root="projects/products/${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
    if scope="$(resolve_nearest_scope_with_cargo "$file" "$product_root")"; then
      printf '%s\n' "$scope"
      return 0
    fi
    # Fallback for product-level non-crate files (README, docs, configs).
    printf '%s\n' "$product_root"
    return 0
  fi

  if [[ "$file" =~ ^projects/libraries/core/([^/]+)/ ]]; then
    local core_root="projects/libraries/core/${BASH_REMATCH[1]}"
    if scope="$(resolve_nearest_scope_with_cargo "$file" "$core_root")"; then
      printf '%s\n' "$scope"
      return 0
    fi
    printf '%s\n' "$core_root"
    return 0
  fi

  if [[ "$file" =~ ^projects/libraries/([^/]+)/ ]]; then
    local library_root="projects/libraries/${BASH_REMATCH[1]}"
    if scope="$(resolve_nearest_scope_with_cargo "$file" "$library_root")"; then
      printf '%s\n' "$scope"
      return 0
    fi
    printf '%s\n' "$library_root"
    return 0
  fi

  return 1
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
