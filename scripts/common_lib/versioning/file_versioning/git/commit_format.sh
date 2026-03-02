#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# shellcheck source=scripts/common_lib/versioning/file_versioning/conventions.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/../conventions.sh"

# Shared commit message format constants and helpers.
# Default accepted format:
#   <type>(<scope>): <message>
#   <type>: <message>

# validate_commit_subject_format <message> [separator_regex] [anchored]
# separator_regex default is POSIX whitespace class.
# anchored=true enforces full-line match.
validate_commit_subject_format() {
  local message="$1"
  local separator_regex="${2:-[[:space:]]}"
  local anchored="${3:-false}"

  validate_file_versioning_commit_subject_format "$message" "$separator_regex" "$anchored"
}

# Backward-compatible alias.
validate_commit_message_format() {
  validate_commit_subject_format "$@"
}
