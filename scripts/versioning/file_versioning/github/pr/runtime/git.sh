#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Runtime git/reference helpers.

pr_preferred_base_ref_with_origin() {
  local ref_name="$1"
  if [[ -z "$ref_name" || "$ref_name" == "HEAD" ]]; then
    echo "$ref_name"
    return
  fi

  if git show-ref --verify --quiet "refs/remotes/origin/${ref_name}"; then
    echo "origin/${ref_name}"
    return
  fi

  echo "$ref_name"
}

pr_normalize_branch_display_ref() {
  local raw_ref="$1"
  local normalized

  normalized="${raw_ref#refs/remotes/}"
  normalized="${normalized#refs/heads/}"
  normalized="${normalized#remotes/}"
  normalized="${normalized#origin/}"
  echo "$normalized"
}
