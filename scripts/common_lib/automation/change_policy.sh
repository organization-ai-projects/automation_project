#!/usr/bin/env bash

# Shared change policy helpers for hooks and automation scripts.

# shellcheck source=scripts/common_lib/automation/file_types.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/automation/file_types.sh"
# shellcheck source=scripts/common_lib/automation/scope_resolver.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/automation/scope_resolver.sh"

is_docs_or_scripts_file() {
  local file="$1"
  is_docs_file "$file" \
    || is_script_path_file "$file" \
    || is_workflow_file "$file"
}

is_docs_or_scripts_only_change() {
  local files="$1"
  is_only_change_matching "$files" is_docs_or_scripts_file
}

is_docs_only_change() {
  local files="$1"
  is_only_change_matching "$files" is_docs_file
}

is_tests_only_change() {
  local files="$1"
  is_only_change_matching "$files" is_test_file
}

is_only_change_matching() {
  local files="$1"
  local predicate="$2"
  [[ -n "$files" ]] || return 1
  [[ "$(count_files_matching "$files" "$predicate")" -eq "$(count_non_empty_lines "$files")" ]]
}

is_mixed_docs_and_non_docs_change() {
  local files="$1"
  local total docs
  [[ -n "$files" ]] || return 1
  total="$(count_non_empty_lines "$files")"
  docs="$(count_files_matching "$files" is_docs_file)"
  [[ "$docs" -gt 0 && "$docs" -lt "$total" ]]
}

has_multiple_scopes_in_files() {
  local files="$1"
  local scopes
  scopes="$(collect_scopes_from_files "$files")"
  [[ "$(count_non_empty_lines "$scopes")" -gt 1 ]]
}

count_non_empty_lines() {
  local lines="$1"
  count_matching_lines "$lines" policy_line_is_non_empty
}

count_files_matching() {
  local files="$1"
  local predicate="$2"
  count_matching_lines "$files" "$predicate"
}

count_matching_lines() {
  local lines="$1"
  local predicate="$2"
  local count=0
  local line
  while IFS= read -r line; do
    "$predicate" "$line" && count=$((count + 1))
  done <<< "$lines"
  printf '%s\n' "$count"
}

policy_line_is_non_empty() {
  local line="$1"
  [[ -n "$line" ]]
}
