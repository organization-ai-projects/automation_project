#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Pipeline dependency-check helpers.

pr_pipeline_require_cmd() {
  local cmd_name="$1"
  local error_code="$2"
  if ! command -v "$cmd_name" >/dev/null 2>&1; then
    echo "Error: command '${cmd_name}' not found." >&2
    exit "$error_code"
  fi
}

pr_pipeline_check_dependencies() {
  has_gh="true"
  pr_pipeline_require_cmd gh "$E_DEPENDENCY"
  pr_pipeline_require_cmd jq "$E_DEPENDENCY"
}
