#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Issue directive and decision resolution helpers.

pr_issue_clear_tracking_for() {
  local issue_key="$1"
  [[ -n "$issue_key" ]] || return
  unset "seen_issue[$issue_key]"
  unset "issue_action[$issue_key]"
  unset "seen_reopen_issue[$issue_key]"
  unset "reopen_issue_category[$issue_key]"
}

pr_issue_clear_close_tracking_for() {
  local issue_key="$1"
  [[ -n "$issue_key" ]] || return
  unset "seen_issue[$issue_key]"
  unset "issue_action[$issue_key]"
  unset "issue_category[$issue_key]"
}

pr_issue_resolve_to_reopen() {
  local issue_key="$1"
  local category="$2"
  local force_category="${3:-false}"
  [[ -n "$issue_key" ]] || return

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

pr_issue_try_pre_decision_via_va() {
  local action="$1"
  local issue_key="$2"
  local default_category="$3"
  local inferred_decision="$4"
  local _out_result_var="$5"
  local seen_reopen="false"
  local reopen_category=""
  local explicit_decision=""
  local allow_inferred="false"
  local -n _out_result_ref="$_out_result_var"

  [[ -n "$issue_key" ]] || return 1

  if [[ -n "${seen_reopen_issue[$issue_key]:-}" ]]; then
    seen_reopen="true"
    reopen_category="${reopen_issue_category[$issue_key]:-}"
  fi
  explicit_decision="${issue_directive_decision[$issue_key]:-}"
  if [[ "${allow_inferred_directive_conflicts:-false}" == "true" ]]; then
    allow_inferred="true"
  fi

  _out_result_ref="$(
    va_exec pr issue-decision \
      --action "$action" \
      --issue "$issue_key" \
      --default-category "$default_category" \
      --seen-reopen "$seen_reopen" \
      --reopen-category "$reopen_category" \
      --inferred-decision "$inferred_decision" \
      --explicit-decision "$explicit_decision" \
      --allow-inferred "$allow_inferred" 2>/dev/null
  )"

  [[ "$_out_result_ref" =~ ^DECISION\| ]] || return 1
  return 0
}

pr_issue_apply_pre_decision_result() {
  local result_line="$1"
  local issue_key="$2"
  local action="$3"
  local _out_status_var="$4"
  local kind reason final_action category force_category
  local -n _out_status_ref="$_out_status_var"

  IFS='|' read -r _ kind reason final_action category force_category <<<"$result_line"
  case "$kind" in
  resolve_reopen)
    issue_directive_resolution["$issue_key"]="${reason:-Resolved via directive decision => reopen.}"
    issue_directive_final_action["$issue_key"]="${final_action:-reopen}"
    pr_issue_resolve_to_reopen "$issue_key" "${category:-Unknown}" "${force_category:-false}"
    _out_status_ref="handled"
    return 0
    ;;
  conflict)
    issue_directive_conflict_reason["$issue_key"]="${reason:-conflicting inferred directives}"
    issue_directive_conflict_action["$issue_key"]="$action"
    _out_status_ref="handled"
    return 0
    ;;
  ignore)
    _out_status_ref="handled"
    return 0
    ;;
  continue)
    _out_status_ref="continue"
    return 0
    ;;
  *)
    return 1
    ;;
  esac
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
  local issue_key="" issue_number="" non_compliance_reason="" effective_category="" effective_decision="" inferred_decision=""
  local inferred_conflict_reason=""
  local va_pre_decision_result=""
  local va_pre_decision_status=""
  local pre_decision_resolved_via_va="false"

  pr_issue_parse_key_and_number "$issue_key_raw" issue_key issue_number || return
  [[ -n "$issue_key" && -n "$issue_number" ]] || return

  inferred_decision="${issue_inferred_decision[$issue_key]:-}"
  if pr_issue_try_pre_decision_via_va "$action" "$issue_key" "$default_category" "$inferred_decision" va_pre_decision_result &&
    pr_issue_apply_pre_decision_result "$va_pre_decision_result" "$issue_key" "$action" va_pre_decision_status; then
    if [[ "$va_pre_decision_status" == "handled" ]]; then
      return
    fi
    pre_decision_resolved_via_va="true"
  fi

  if [[ "$pre_decision_resolved_via_va" != "true" ]]; then
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
