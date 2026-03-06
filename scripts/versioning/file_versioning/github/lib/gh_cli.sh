#!/usr/bin/env bash
# shellcheck shell=bash

# Shared GitHub CLI helpers for shell automation scripts.

gh_cli_require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: ${cmd} is required." >&2
    exit 3
  fi
}

gh_cli_require_gh_jq() {
  gh_cli_require_cmd gh
  gh_cli_require_cmd jq
}

gh_cli_require_gh_jq_perl() {
  gh_cli_require_gh_jq
  gh_cli_require_cmd perl
}

gh_cli_resolve_repo_name() {
  if [[ -n "${GH_REPO:-}" ]]; then
    echo "$GH_REPO"
    return 0
  fi
  gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true
}
