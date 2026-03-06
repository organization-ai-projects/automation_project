#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Crate-inference helpers for footprint and breaking-scope attribution.

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

