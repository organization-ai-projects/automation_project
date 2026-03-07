#!/usr/bin/env bash
# shellcheck shell=bash

closure_hygiene_bootstrap() {
  gh_cli_require_gh_jq

  # shellcheck disable=SC2034
  REPO_NAME="${GH_REPO:-}"
  if [[ -z "$REPO_NAME" ]]; then
    REPO_NAME="$(gh_cli_resolve_repo_name)"
  fi
  if [[ -z "$REPO_NAME" ]]; then
    echo "Error: unable to resolve repository name." >&2
    exit 3
  fi
  # shellcheck disable=SC2034
  REPO_OWNER="${REPO_NAME%%/*}"
  # shellcheck disable=SC2034
  REPO_SHORT_NAME="${REPO_NAME#*/}"
}
