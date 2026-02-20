#!/usr/bin/env bash

# Fallback Rust scope resolution when workspace member mapping is unavailable.

non_workspace_rust_resolve_nearest_scope_with_cargo() {
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

non_workspace_rust_resolve_scope_fallback() {
  local file="$1"
  local scope=""

  if [[ "$file" =~ ^projects/products/([^/]+)/([^/]+)/ ]]; then
    local product_root="projects/products/${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
    if scope="$(non_workspace_rust_resolve_nearest_scope_with_cargo "$file" "$product_root")"; then
      printf '%s\n' "$scope"
      return 0
    fi
    # Fallback for product-level non-crate files (README, docs, configs).
    printf '%s\n' "$product_root"
    return 0
  fi

  if [[ "$file" =~ ^projects/libraries/core/([^/]+)/ ]]; then
    local core_root="projects/libraries/core/${BASH_REMATCH[1]}"
    if scope="$(non_workspace_rust_resolve_nearest_scope_with_cargo "$file" "$core_root")"; then
      printf '%s\n' "$scope"
      return 0
    fi
    printf '%s\n' "$core_root"
    return 0
  fi

  if [[ "$file" =~ ^projects/libraries/([^/]+)/ ]]; then
    local library_root="projects/libraries/${BASH_REMATCH[1]}"
    if scope="$(non_workspace_rust_resolve_nearest_scope_with_cargo "$file" "$library_root")"; then
      printf '%s\n' "$scope"
      return 0
    fi
    printf '%s\n' "$library_root"
    return 0
  fi

  return 1
}
