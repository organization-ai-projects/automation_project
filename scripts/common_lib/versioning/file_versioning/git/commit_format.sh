#!/usr/bin/env bash

# Shared commit message format constants and helpers.
# Default accepted format:
#   <type>(<scope>): <message>
#   <type>: <message>

COMMIT_ALLOWED_TYPES_REGEX='^(feature|feat|fix|fixture|doc|docs|refactor|test|tests|chore|ci|perf|build)'
COMMIT_SCOPE_REGEX='(\([a-zA-Z0-9_./,-]+\))?'
COMMIT_MESSAGE_BODY_REGEX='.+'

# validate_commit_message_format <message> [separator_regex] [anchored]
# separator_regex default is POSIX whitespace class.
# anchored=true enforces full-line match.
validate_commit_message_format() {
  local message="$1"
  local separator_regex="${2:-[[:space:]]}"
  local anchored="${3:-false}"
  local regex

  regex="${COMMIT_ALLOWED_TYPES_REGEX}${COMMIT_SCOPE_REGEX}:${separator_regex}${COMMIT_MESSAGE_BODY_REGEX}"
  if [[ "$anchored" == "true" ]]; then
    regex="${regex}$"
  fi

  [[ "$message" =~ $regex ]]
}
