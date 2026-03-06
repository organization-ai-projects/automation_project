#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Pipeline issue-tracking initialization helpers.

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

