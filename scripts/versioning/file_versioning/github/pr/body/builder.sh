#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# PR body content generation helpers.

pr_body_compute_validation_state() {
  pr_compute_ci_status
  pr_compute_breaking_scope

  ci_status_with_symbol="$ci_status"
  case "$ci_status" in
  PASS) ci_status_with_symbol="PASS ✅" ;;
  FAIL) ci_status_with_symbol="FAIL ❌" ;;
  RUNNING) ci_status_with_symbol="RUNNING ⏳" ;;
  *) ci_status_with_symbol="UNKNOWN ⚪" ;;
  esac
}

pr_body_build_description_section() {
  echo "### Description"
  echo ""
  echo "This pull request merges the \`${head_ref_display}\` branch into \`${base_ref_display}\` and summarizes merged pull requests and resolved issues."
  echo ""
}

pr_body_build_validation_section() {
  pr_build_validation_gate_section \
    "$ci_status_with_symbol" \
    "$breaking_detected" \
    "$breaking_scope_crates" \
    "$breaking_scope_commits"
  echo ""
}

pr_body_build_issue_outcomes_section() {
  echo "### Issue Outcomes"
  echo ""

  if [[ ! -s "$resolved_issues_file" && ! -s "$reopened_issues_file" && ! -s "$directive_resolution_tmp" && ! -s "$conflict_issues_file" ]]; then
    echo "- No issues processed in this PR."
    echo ""
    return
  fi

  echo "#### Category 1: Issues Without Conflicts"
  echo ""
  echo "##### Closes/Fixes"
  echo ""
  if [[ -s "$resolved_issues_file" ]]; then
    cat "$resolved_issues_file"
  else
    echo "- No resolved issues detected via GitHub references or PR body keywords."
  fi
  echo ""

  echo "##### Reopened"
  echo ""
  if [[ -s "$reopened_issues_file" ]]; then
    cat "$reopened_issues_file"
  else
    echo "- No reopened issues detected."
  fi
  echo ""

  echo "#### Category 2: Issues With Conflicts"
  echo ""
  echo "##### Auto-resolved"
  echo ""
  if [[ -s "$directive_resolution_tmp" ]]; then
    cat "$directive_resolution_tmp"
  else
    echo "- No auto-resolved directive conflicts."
  fi
  echo ""

  echo "##### Not resolved"
  echo ""
  if [[ -s "$conflict_issues_file" ]]; then
    cat "$conflict_issues_file"
  else
    echo "- No unresolved directive conflicts."
  fi
  echo ""
}

pr_body_emit_key_changes_group() {
  local title="$1"
  local source_file="$2"

  [[ -s "$source_file" ]] || return 1

  echo "#### ${title}"
  echo ""
  write_section_from_file "$source_file"
  echo ""
  return 0
}

pr_body_build_key_changes_section() {
  local key_changes_found=0

  echo "### Key Changes"
  echo ""

  pr_body_emit_key_changes_group "Synchronization" "$sync_tmp" && key_changes_found=1
  pr_body_emit_key_changes_group "Features" "$features_tmp" && key_changes_found=1
  pr_body_emit_key_changes_group "Bug Fixes" "$bugs_tmp" && key_changes_found=1
  pr_body_emit_key_changes_group "Refactoring" "$refactors_tmp" && key_changes_found=1

  if [[ "$key_changes_found" -eq 0 ]]; then
    echo "- No significant items detected."
    echo ""
  fi

  echo "#### Change Footprint"
  echo ""
  pr_emit_change_footprint "${base_ref_git}..${head_ref_git}"
  echo ""
}

pr_body_build_content() {
  pr_body_compute_validation_state

  body_content="$(
    {
    pr_body_build_description_section
    pr_body_build_validation_section
    pr_body_build_issue_outcomes_section
    pr_body_build_key_changes_section
    }
  )"
}

pr_body_apply_validation_only_if_needed() {
  local current_pr_body
  local validation_gate_section

  [[ "$validation_only" != "true" || -z "$auto_edit_pr_number" ]] && return

  # Safety: allow validation-only refresh to run even if called before full body build.
  if [[ -z "${ci_status_with_symbol:-}" ]]; then
    pr_body_compute_validation_state
  fi

  current_pr_body="$(pr_get_pr_body "$auto_edit_pr_number" "read PR #${auto_edit_pr_number} body for validation-only update")"
  if [[ -z "$current_pr_body" ]]; then
    echo "Error: unable to read current PR body for validation-only update." >&2
    exit "$E_PARTIAL"
  fi

  validation_gate_section="$(pr_build_validation_gate_section "$ci_status_with_symbol" "$breaking_detected" "$breaking_scope_crates" "$breaking_scope_commits")"
  body_content="$(pr_replace_validation_gate_in_body "$current_pr_body" "$validation_gate_section")"
}
