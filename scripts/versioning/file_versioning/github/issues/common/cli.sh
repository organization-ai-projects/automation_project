#!/usr/bin/env bash

issue_cli_require_option_value() {
  local option="$1"
  local value="${2:-}"
  local error_handler="${3:-}"

  if [[ -n "$value" ]]; then
    return 0
  fi

  local message="${option} requires a value."
  if [[ -n "$error_handler" ]] && declare -F "$error_handler" >/dev/null 2>&1; then
    "$error_handler" "$message"
    return 1
  fi

  echo "Error: ${message}" >&2
  return 1
}
