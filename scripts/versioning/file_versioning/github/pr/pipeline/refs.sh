#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Pipeline ref/mode resolution and extraction helpers.

pr_pipeline_resolve_refs_and_modes() {
  local current_branch

  if [[ "$dry_run" == "true" ]]; then
    if ! command -v git >/dev/null 2>&1; then
      echo "Error: command 'git' not found." >&2
      exit "$E_GIT"
    fi
    if [[ -z "$head_ref" ]]; then
      current_branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || true)"
      head_ref="$current_branch"
    fi
    if [[ -z "$base_ref" ]]; then
      base_ref="dev"
    fi
    base_ref="$(pr_preferred_base_ref_with_origin "$base_ref")"
    if [[ -z "$head_ref" ]]; then
      echo "Error: unable to determine head branch in --dry-run mode." >&2
      exit "$E_GIT"
    fi
  else
    base_ref="$(pr_gh_optional "read base branch for PR #${main_pr_number}" pr view "$main_pr_number" --json baseRefName -q '.baseRefName')"
    head_ref="$(pr_gh_optional "read head branch for PR #${main_pr_number}" pr view "$main_pr_number" --json headRefName -q '.headRefName')"
    if [[ -z "$base_ref" ]]; then
      pr_warn_optional "PR #${main_pr_number} base branch unavailable; defaulting to dev (expected dev base)."
      base_ref="dev"
    fi
    if [[ -z "$head_ref" ]]; then
      pr_warn_optional "PR #${main_pr_number} head branch unavailable; defaulting to dev."
      head_ref="dev"
    fi
  fi

  base_ref_display="$(pr_normalize_branch_display_ref "$base_ref")"
  head_ref_display="$(pr_normalize_branch_display_ref "$head_ref")"
  base_ref_git="$base_ref"
  head_ref_git="$head_ref"

  if ! git rev-parse --verify --quiet "${base_ref_git}^{commit}" >/dev/null 2>&1; then
    base_ref_git="$base_ref_display"
  fi
  if ! git rev-parse --verify --quiet "${head_ref_git}^{commit}" >/dev/null 2>&1; then
    head_ref_git="$head_ref_display"
  fi

  if [[ "$dry_run" == "true" && "$create_pr" == "true" ]]; then
    online_enrich="true"
  fi
}

pr_pipeline_extract_pr_refs() {
  : >"$extracted_prs_file"
  : >"$resolved_issues_file"
  : >"$reopened_issues_file"
  : >"$conflict_issues_file"

  if [[ "$dry_run" == "true" ]]; then
    pr_load_dry_compare_commits_into_globals
    if ! pr_extract_child_prs_from_compare; then
      echo "Warning: unable to extract PRs from compare ${base_ref_git}...${head_ref_git}." >&2
    fi
  else
    if ! pr_extract_child_prs; then
      echo "Warning: unable to fetch commits for PR #${main_pr_number} (API unavailable or PR not found)." >&2
    fi
  fi
}

