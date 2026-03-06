#!/usr/bin/env bash
# shellcheck shell=bash

pr_directive_conflict_guard_usage() {
  cat <<USAGE
Usage:
  pr_directive_conflict_guard/run.sh --pr PR_NUMBER [--repo owner/name]

Notes:
  - Detects Closes/Fixes + Reopen directives targeting the same issue in PR body.
  - Requires explicit per-issue decision:
      Directive Decision: #<issue> => close|reopen
  - Writes a deterministic decision/conflict section into PR body.
  - Exits non-zero when unresolved conflicts remain.
USAGE
}

pr_directive_conflict_guard_require_number() {
  local name="$1"
  local value="${2:-}"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Error: ${name} must be a positive integer." >&2
    exit 2
  fi
}

pr_directive_conflict_guard_parse_cli() {
  # shellcheck disable=SC2034
  local -n out_pr_number="$1"
  # shellcheck disable=SC2034
  local -n out_repo_name="$2"
  shift 2

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --pr)
      out_pr_number="${2:-}"
      shift 2
      ;;
    --repo)
      # shellcheck disable=SC2034
      out_repo_name="${2:-}"
      shift 2
      ;;
    -h | --help)
      pr_directive_conflict_guard_usage
      exit 0
      ;;
    *)
      echo "Error: unknown option: $1" >&2
      pr_directive_conflict_guard_usage >&2
      exit 2
      ;;
    esac
  done

  if [[ -z "$out_pr_number" ]]; then
    echo "Error: --pr is required." >&2
    pr_directive_conflict_guard_usage >&2
    exit 2
  fi

  pr_directive_conflict_guard_require_number "--pr" "$out_pr_number"
}
