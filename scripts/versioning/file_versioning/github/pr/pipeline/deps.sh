#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Pipeline dependency-check helpers.

pr_pipeline_check_dependencies() {
  local need_jq="false"

  has_gh="false"
  if command -v gh >/dev/null 2>&1; then
    has_gh="true"
  fi

  if [[ "$has_gh" != "true" ]]; then
    echo "Error: command 'gh' not found." >&2
    exit "$E_DEPENDENCY"
  fi

  if [[ "$has_gh" == "true" ]]; then
    need_jq="true"
  fi
  if [[ "$dry_run" == "false" || "$create_pr" == "true" ]]; then
    need_jq="true"
  fi
  if [[ "$need_jq" == "true" ]] && ! command -v jq >/dev/null 2>&1; then
    echo "Error: command 'jq' not found." >&2
    exit "$E_DEPENDENCY"
  fi
}

