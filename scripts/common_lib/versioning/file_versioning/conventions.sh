#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# Shared naming/message conventions across file_versioning tooling.

FILE_VERSIONING_ALLOWED_TYPES_REGEX='^(feature|feat|fix|doc|docs|refactor|test|tests|chore|perf)'
FILE_VERSIONING_SCOPE_REGEX='(\([a-zA-Z0-9_./,-]+\))?'
FILE_VERSIONING_MESSAGE_BODY_REGEX='.+'

# Backward-compatible aliases for existing commit helpers.
COMMIT_ALLOWED_TYPES_REGEX="$FILE_VERSIONING_ALLOWED_TYPES_REGEX"
COMMIT_SCOPE_REGEX="$FILE_VERSIONING_SCOPE_REGEX"
COMMIT_MESSAGE_BODY_REGEX="$FILE_VERSIONING_MESSAGE_BODY_REGEX"

# PR titles follow the same convention as commit subjects in this automation.
FILE_VERSIONING_PR_ALLOWED_TYPES_REGEX="$FILE_VERSIONING_ALLOWED_TYPES_REGEX"
FILE_VERSIONING_PR_SCOPE_REGEX="$FILE_VERSIONING_SCOPE_REGEX"
FILE_VERSIONING_PR_TITLE_BODY_REGEX="$FILE_VERSIONING_MESSAGE_BODY_REGEX"

build_file_versioning_subject_regex() {
  local types_regex="$1"
  local scope_regex="$2"
  local body_regex="$3"
  local separator_regex="${4:-[[:space:]]}"
  local anchored="${5:-false}"
  local regex

  regex="${types_regex}${scope_regex}:${separator_regex}${body_regex}"
  if [[ "$anchored" == "true" ]]; then
    regex="${regex}$"
  fi

  printf '%s\n' "$regex"
}

validate_file_versioning_commit_subject_format() {
  local message="$1"
  local separator_regex="${2:-[[:space:]]}"
  local anchored="${3:-false}"
  local regex

  regex="$(build_file_versioning_subject_regex \
    "$FILE_VERSIONING_ALLOWED_TYPES_REGEX" \
    "$FILE_VERSIONING_SCOPE_REGEX" \
    "$FILE_VERSIONING_MESSAGE_BODY_REGEX" \
    "$separator_regex" \
    "$anchored")"
  [[ "$message" =~ $regex ]]
}

validate_file_versioning_pr_title_format() {
  local title="$1"
  local separator_regex="${2:-[[:space:]]}"
  local anchored="${3:-true}"
  local regex

  regex="$(build_file_versioning_subject_regex \
    "$FILE_VERSIONING_PR_ALLOWED_TYPES_REGEX" \
    "$FILE_VERSIONING_PR_SCOPE_REGEX" \
    "$FILE_VERSIONING_PR_TITLE_BODY_REGEX" \
    "$separator_regex" \
    "$anchored")"
  [[ "$title" =~ $regex ]]
}
