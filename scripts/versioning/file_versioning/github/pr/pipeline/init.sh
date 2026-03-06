#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Pipeline initialization and dependency helpers.

pr_pipeline_init_artifacts_and_state() {
  if [[ "$keep_artifacts" == "true" ]]; then
    extracted_prs_file="extracted_prs.txt"
    resolved_issues_file="resolved_issues.txt"
    reopened_issues_file="reopened_issues.txt"
    conflict_issues_file="issue_directive_conflicts.txt"
  else
    extracted_prs_file="$(mktemp)"
    resolved_issues_file="$(mktemp)"
    reopened_issues_file="$(mktemp)"
    conflict_issues_file="$(mktemp)"
  fi

  features_tmp="$(mktemp)"
  bugs_tmp="$(mktemp)"
  refactors_tmp="$(mktemp)"
  sync_tmp="$(mktemp)"
  issues_tmp="$(mktemp)"
  reopen_tmp="$(mktemp)"
  conflict_tmp="$(mktemp)"
  directive_resolution_tmp="$(mktemp)"

  declare -gA pr_title_hint
  online_enrich="false"
  pr_enrich_failed=0
  breaking_detected=0
  ci_status="UNKNOWN"
  breaking_scope_crates=""
  breaking_scope_commits=""
  pr_created_successfully="false"
  dry_compare_commit_messages=""
  dry_compare_commit_headlines=""
}

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

pr_pipeline_resolve_refs_and_modes() {
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

pr_pipeline_init_issue_tracking() {
  declare -gA seen_issue
  declare -gA issue_category
  declare -gA issue_action
  declare -gA issue_neutralization_reason
  declare -gA issue_reopen_detected
  declare -gA seen_reopen_issue
  declare -gA reopen_issue_category
  declare -gA issue_directive_decision
  declare -gA issue_inferred_decision
  declare -gA issue_directive_conflict_reason
  declare -gA issue_directive_conflict_action
  declare -gA issue_directive_resolution
  declare -gA issue_directive_final_action
  declare -gA pr_ref_cache
  declare -gA duplicate_targets
  declare -gA issue_non_compliance_reason_cache
  declare -gA issue_non_compliance_skip
  declare -gA issue_non_compliance_action

  pr_count=0
  issue_count=0
  reopen_issue_count=0
  directive_conflict_count=0
  neutralized_issue_count=0
}
