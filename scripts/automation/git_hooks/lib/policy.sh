#!/usr/bin/env bash

# Shared policy helpers for hooks and staging guards.

# shellcheck source=scripts/automation/git_hooks/lib/scope_resolver.sh
source "$(git rev-parse --show-toplevel)/scripts/automation/git_hooks/lib/scope_resolver.sh"

is_docs_or_scripts_file() {
  local file="$1"
  is_docs_file "$file" \
    || [[ "$file" == scripts/* ]] \
    || [[ "$file" == .github/workflows/* ]]
}

is_docs_or_scripts_only_change() {
  local files="$1"
  local file

  [[ -z "$files" ]] && return 1

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    if is_docs_or_scripts_file "$file"; then
      continue
    fi
    return 1
  done <<< "$files"

  return 0
}

is_mixed_docs_and_non_docs_change() {
  local files="$1"
  local file
  local has_docs=0
  local has_non_docs=0

  [[ -z "$files" ]] && return 1

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    if is_docs_file "$file"; then
      has_docs=1
    else
      has_non_docs=1
    fi
  done <<< "$files"

  [[ $has_docs -eq 1 && $has_non_docs -eq 1 ]]
}

has_multiple_scopes_in_files() {
  local files="$1"
  local scopes
  local count=0

  scopes="$(collect_scopes_from_files "$files")"
  [[ -z "$scopes" ]] && return 1

  while IFS= read -r _; do
    [[ -n "$_" ]] && count=$((count + 1))
  done <<< "$scopes"

  [[ $count -gt 1 ]]
}
