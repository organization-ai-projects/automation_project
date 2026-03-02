#!/usr/bin/env bash

github_find_pr_number_by_branch() {
  local branch_name="$1"
  local base_branch="${2:-}"
  local pr_number=""

  if [[ -n "$base_branch" ]]; then
    pr_number="$(gh pr list --head "$branch_name" --base "$base_branch" --json number --jq '.[0].number' 2>/dev/null || true)"
  else
    pr_number="$(gh pr list --head "$branch_name" --json number --jq '.[0].number' 2>/dev/null || true)"
  fi

  if [[ -z "$pr_number" || "$pr_number" == "null" ]]; then
    return 1
  fi

  printf '%s\n' "$pr_number"
}
