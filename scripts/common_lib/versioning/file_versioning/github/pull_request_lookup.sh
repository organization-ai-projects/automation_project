#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# shellcheck source=scripts/common_lib/versioning/file_versioning/github/commands.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/commands.sh"

github_find_pr_number_by_branch() {
  local branch_name="$1"
  local base_branch="${2:-}"
  local pr_number=""
  local cmd=(vcs_remote_pr_list --head "$branch_name")

  if [[ -n "$base_branch" ]]; then
    cmd+=(--base "$base_branch")
  fi
  cmd+=(--json number --jq '.[0].number')
  pr_number="$("${cmd[@]}" 2>/dev/null || true)"

  if [[ -z "$pr_number" || "$pr_number" == "null" ]]; then
    return 1
  fi

  printf '%s\n' "$pr_number"
}
