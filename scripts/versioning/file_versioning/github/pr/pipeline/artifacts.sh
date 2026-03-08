#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Pipeline artifacts/state initialization helpers.

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

