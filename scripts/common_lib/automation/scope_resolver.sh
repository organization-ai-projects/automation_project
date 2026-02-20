#!/usr/bin/env bash

# Shared scope/crate resolver helpers used by hooks and automation scripts.

resolve_product_scope() {
  local stability="$1"
  local product="$2"
  local component="${3:-}"
  local product_root="projects/products/${stability}/${product}"

  if [[ -n "$component" && -f "${product_root}/${component}/Cargo.toml" ]]; then
    printf '%s/%s\n' "$product_root" "$component"
    return 0
  fi

  printf '%s\n' "$product_root"
  return 0
}

resolve_scope_from_path() {
  local file="$1"

  # Products: infer ui/backend scope only when corresponding crate exists.
  if [[ "$file" =~ ^projects/products/([^/]+)/([^/]+)/(ui|backend)/ ]]; then
    resolve_product_scope "${BASH_REMATCH[1]}" "${BASH_REMATCH[2]}" "${BASH_REMATCH[3]}"
    return 0
  fi

  if [[ "$file" =~ ^projects/products/([^/]+)/([^/]+)/ ]]; then
    resolve_product_scope "${BASH_REMATCH[1]}" "${BASH_REMATCH[2]}"
    return 0
  fi

  if [[ "$file" =~ ^projects/libraries/core/([^/]+)/ ]]; then
    printf 'projects/libraries/core/%s\n' "${BASH_REMATCH[1]}"
    return 0
  fi

  if [[ "$file" =~ ^projects/libraries/([^/]+)/ ]]; then
    printf 'projects/libraries/%s\n' "${BASH_REMATCH[1]}"
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
