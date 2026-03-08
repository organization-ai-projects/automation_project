#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# CI status detection helpers.

pr_compute_ci_status() {
  local target_pr_number=""
  local rollup_json conclusions unresolved

  if [[ -n "$auto_edit_pr_number" ]]; then
    target_pr_number="$auto_edit_pr_number"
  elif [[ "$dry_run" == "false" && -n "$main_pr_number" ]]; then
    target_pr_number="$main_pr_number"
  fi

  ci_status="UNKNOWN"
  [[ -z "$target_pr_number" ]] && return

  rollup_json="$(pr_gh_optional "read checks for PR #${target_pr_number}" pr view "$target_pr_number" --json statusCheckRollup)"
  if [[ -z "$rollup_json" ]]; then
    return
  fi

  conclusions="$(echo "$rollup_json" | jq -r '.statusCheckRollup // [] | map((.conclusion // .state // .status // "UNKNOWN") | tostring | ascii_upcase) | map(select(length > 0)) | .[]' 2>/dev/null || true)"
  if [[ -z "$conclusions" ]]; then
    return
  fi

  if echo "$conclusions" | grep -Eq '^(FAILURE|FAILED|CANCELLED|TIMED_OUT|ACTION_REQUIRED|STARTUP_FAILURE)$'; then
    ci_status="FAIL"
    return
  fi

  if echo "$conclusions" | grep -Eq '^(IN_PROGRESS|QUEUED|PENDING|WAITING|REQUESTED)$'; then
    ci_status="RUNNING"
    return
  fi

  unresolved="$(echo "$conclusions" | grep -Ev '^(SUCCESS|PASSED|NEUTRAL|SKIPPED|COMPLETED)$' || true)"
  if [[ -n "$unresolved" ]]; then
    ci_status="UNKNOWN"
    return
  fi

  if echo "$conclusions" | grep -Eq '^(SUCCESS|PASSED)$'; then
    ci_status="PASS"
  fi
}
