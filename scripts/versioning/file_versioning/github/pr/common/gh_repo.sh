#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# GitHub repo-scoped API helper shared across PR modules.

pr_repo_api_call() {
  local repo_name_with_owner="$1"
  local route="$2"
  shift 2
  if [[ "${1:-}" == "-X" ]]; then
    local method="${2:-GET}"
    shift 2
    gh api -X "$method" "repos/${repo_name_with_owner}/${route}" "$@"
    return
  fi
  gh api "repos/${repo_name_with_owner}/${route}" "$@"
}
