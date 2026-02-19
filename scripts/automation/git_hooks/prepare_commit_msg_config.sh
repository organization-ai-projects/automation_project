#!/usr/bin/env bash
# Configuration for prepare-commit-msg scope detection.

# Resolve commit scope from a staged file path.
# Prints scope and returns 0 when matched, otherwise returns 1.
resolve_scope_from_path() {
  local file="$1"

  if [[ "$file" =~ ^projects/libraries/([^/]+)/ ]]; then
    printf 'projects/libraries/%s\n' "${BASH_REMATCH[1]}"
    return 0
  fi

  if [[ "$file" =~ ^projects/products/[^/]+/([^/]+)/ ]]; then
    local product_name="${BASH_REMATCH[1]}"
    local component=""
    if [[ "$file" =~ ^projects/products/[^/]+/[^/]+/(ui|backend)/ ]]; then
      component="${BASH_REMATCH[1]}"
    fi

    if [[ -n "$component" ]]; then
      printf 'projects/products/%s/%s\n' "$product_name" "$component"
    else
      printf 'projects/products/%s\n' "$product_name"
    fi
    return 0
  fi

  return 1
}
