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

issue_cli_require_option_value_or_usage() {
  local option="$1"
  local value="${2:-}"
  local usage_fn="${3:-}"

  if issue_cli_require_option_value "$option" "$value"; then
    return 0
  fi
  if [[ -n "$usage_fn" ]] && declare -F "$usage_fn" >/dev/null 2>&1; then
    "$usage_fn" >&2
  fi
  return 1
}

issue_cli_unknown_option_with_usage() {
  local option="$1"
  local usage_fn="${2:-}"

  echo "Error: unknown option: ${option}" >&2
  if [[ -n "$usage_fn" ]] && declare -F "$usage_fn" >/dev/null 2>&1; then
    "$usage_fn" >&2
  fi
}
