#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Issue directive and decision resolution helpers.

pr_issue_clear_tracking_for() {
  local issue_key="$1"
  unset "seen_issue[$issue_key]"
  unset "issue_action[$issue_key]"
  unset "seen_reopen_issue[$issue_key]"
  unset "reopen_issue_category[$issue_key]"
}

pr_issue_clear_close_tracking_for() {
  local issue_key="$1"
  unset "seen_issue[$issue_key]"
  unset "issue_action[$issue_key]"
  unset "issue_category[$issue_key]"
}

pr_issue_resolve_to_reopen() {
  local issue_key="$1"
  local category="$2"
  local force_category="${3:-false}"

  issue_directive_resolution["$issue_key"]="Resolved via directive decision => reopen."
  issue_directive_final_action["$issue_key"]="reopen"
  if [[ "$force_category" == "true" ]]; then
    issue_category["$issue_key"]="$category"
  elif [[ -z "${issue_category[$issue_key]:-}" ]]; then
    issue_category["$issue_key"]="$category"
  fi
  pr_issue_clear_tracking_for "$issue_key"
}

pr_issue_inferred_conflict_reason_for() {
  local issue_key="$1"
  local inferred_decision="$2"

  [[ "$allow_inferred_directive_conflicts" != "true" ]] && return
  if [[ -z "$inferred_decision" || "$inferred_decision" == "conflict" ]]; then
    echo "conflicting inferred directives"
  fi
}

pr_issue_should_skip_close_action() {
  local issue_key="$1"
  local issue_number="$2"
  local non_compliance_reason="$3"
  local action="$4"
  local is_pr_ref

  [[ "$action" != "Closes" ]] && return 1

  is_pr_ref="$(pr_is_pull_request_ref "$issue_number")"
  if [[ "$is_pr_ref" == "true" ]]; then
    return 0
  fi

  if [[ -n "$non_compliance_reason" ]]; then
    issue_non_compliance_skip["$issue_key"]="$non_compliance_reason"
    issue_non_compliance_action["$issue_key"]="$action"
    return 0
  fi

  return 1
}

pr_issue_load_effective_context() {
  local issue_number="$1"
  local default_category="$2"
  local _out_effective_category_var="$3"
  local _out_non_compliance_reason_var="$4"
  local -n _out_effective_category_ref="$_out_effective_category_var"
  local -n _out_non_compliance_reason_ref="$_out_non_compliance_reason_var"
  local issue_context_payload issue_labels_raw title_category

  issue_context_payload="$(pr_issue_context_payload_for "$issue_number")"
  issue_labels_raw="$(pr_issue_payload_field "$issue_context_payload" "labels")"
  title_category="$(pr_issue_payload_field "$issue_context_payload" "title_category")"
  _out_non_compliance_reason_ref="$(pr_issue_payload_field "$issue_context_payload" "non_compliance_reason")"
  _out_effective_category_ref="$(pr_resolve_effective_category "$default_category" "$issue_labels_raw" "$title_category")"
}

pr_add_issue_entry() {
  local action="$1"
  local issue_key_raw="$2"
  local default_category="$3"
  local issue_key issue_number non_compliance_reason effective_category effective_decision inferred_decision=""
  local inferred_conflict_reason=""

  pr_issue_parse_key_and_number "$issue_key_raw" issue_key issue_number || return

  if [[ "$action" == "Closes" && -n "${seen_reopen_issue[$issue_key]:-}" ]]; then
    pr_issue_resolve_to_reopen "$issue_key" "${reopen_issue_category[$issue_key]:-$default_category}" "true"
    return
  fi

  inferred_decision="${issue_inferred_decision[$issue_key]:-}"
  inferred_conflict_reason="$(pr_issue_inferred_conflict_reason_for "$issue_key" "$inferred_decision")"

  if [[ -z "$inferred_conflict_reason" ]]; then
    effective_decision="${issue_directive_decision[$issue_key]:-$inferred_decision}"
    if [[ "$effective_decision" == "close" ]]; then
      if [[ "$action" == "Reopen" ]]; then
        return
      fi
    elif [[ "$effective_decision" == "reopen" ]]; then
      pr_issue_resolve_to_reopen "$issue_key" "$default_category"
      return
    fi
  fi

  if [[ -n "$inferred_conflict_reason" ]]; then
    issue_directive_conflict_reason["$issue_key"]="$inferred_conflict_reason"
    issue_directive_conflict_action["$issue_key"]="$action"
    return
  fi

  pr_issue_load_effective_context "$issue_number" "$default_category" effective_category non_compliance_reason

  if pr_issue_should_skip_close_action "$issue_key" "$issue_number" "$non_compliance_reason" "$action"; then
    return
  fi

  if [[ "$action" == "Reopen" ]]; then
    pr_mark_reopen_issue "$issue_key" "$effective_category"
    if [[ -n "${seen_issue[$issue_key]:-}" ]]; then
      pr_issue_clear_close_tracking_for "$issue_key"
    fi
    return
  fi

  seen_issue["$issue_key"]=1
  issue_action["$issue_key"]="$action"
  issue_category["$issue_key"]="$effective_category"
}
