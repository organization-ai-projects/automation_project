#!/usr/bin/env bash
# shellcheck shell=bash

auto_add_usage() {
  cat <<EOF
Usage:
  auto_add_closes_on_dev_pr/run.sh --pr PR_NUMBER [--repo owner/name]

Description:
  - Targets open PRs into dev.
  - Detects "Part of #N" refs from PR body + commits.
  - If issue #N has exactly one assignee and that assignee is the PR author,
    ensures the PR body contains "Closes #N" in a managed block.
EOF
}

auto_add_require_number() {
  local name="$1"
  local value="${2:-}"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Error: ${name} must be a positive integer." >&2
    exit 2
  fi
}

auto_add_parse_cli() {
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
      auto_add_usage
      exit 0
      ;;
    *)
      echo "Error: unknown option: $1" >&2
      auto_add_usage >&2
      exit 2
      ;;
    esac
  done

  if [[ -z "$out_pr_number" ]]; then
    echo "Error: --pr is required." >&2
    auto_add_usage >&2
    exit 2
  fi

  auto_add_require_number "--pr" "$out_pr_number"
}
