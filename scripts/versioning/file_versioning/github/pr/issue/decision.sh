#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Issue directive and decision resolution helpers.

pr_mark_directive_decisions_from_text() {
  local text="$1"
  while IFS='|' read -r issue_key decision; do
    [[ -z "$issue_key" || -z "$decision" ]] && continue
    issue_directive_decision["$issue_key"]="$decision"
  done < <(parse_directive_decisions_from_text "$text")
}

pr_mark_inferred_decisions_from_text() {
  local text="$1"
  local issue_key action decision

  while IFS='|' read -r action issue_key; do
    [[ -z "$action" || -z "$issue_key" ]] && continue
    case "$action" in
    Closes) decision="close" ;;
    Reopen) decision="reopen" ;;
    *) continue ;;
    esac

    if [[ -z "${issue_directive_decision[$issue_key]:-}" ]]; then
      if [[ -n "${issue_inferred_decision[$issue_key]:-}" && "${issue_inferred_decision[$issue_key]}" != "$decision" ]]; then
        # Reopen has higher precedence than close when inferred directives conflict.
        if [[ "$decision" == "reopen" || "${issue_inferred_decision[$issue_key]}" == "reopen" ]]; then
          issue_inferred_decision["$issue_key"]="reopen"
        else
          issue_inferred_decision["$issue_key"]="conflict"
        fi
      elif [[ -z "${issue_inferred_decision[$issue_key]:-}" ]]; then
        issue_inferred_decision["$issue_key"]="$decision"
      fi
    fi
  done < <(parse_directive_events_from_text "$text")
}

pr_add_issue_entry() {
  local action="$1"
  local issue_key_raw="$2"
  local default_category="$3"
  local issue_key issue_number issue_labels_raw title_category effective_category effective_decision inferred_decision=""
  local inferred_conflict_reason=""
  local is_pr_ref
  local non_compliance_reason

  issue_key="$(normalize_issue_key "$issue_key_raw" || true)"
  [[ -z "$issue_key" ]] && return
  issue_number="${issue_key//#/}"

  if [[ "$action" == "Closes" && -n "${seen_reopen_issue[$issue_key]:-}" ]]; then
    issue_directive_resolution["$issue_key"]="Resolved via directive decision => reopen."
    issue_directive_final_action["$issue_key"]="reopen"
    issue_category["$issue_key"]="${reopen_issue_category[$issue_key]:-$default_category}"
    unset "seen_issue[$issue_key]"
    unset "issue_action[$issue_key]"
    unset "seen_reopen_issue[$issue_key]"
    unset "reopen_issue_category[$issue_key]"
    return
  fi

  if [[ "$allow_inferred_directive_conflicts" == "true" ]]; then
    inferred_decision="${issue_inferred_decision[$issue_key]:-}"
    if [[ -z "$inferred_decision" || "$inferred_decision" == "conflict" ]]; then
      inferred_conflict_reason="conflicting inferred directives"
    fi
  fi

  if [[ -z "$inferred_conflict_reason" ]]; then
    effective_decision="${issue_directive_decision[$issue_key]:-$inferred_decision}"
    if [[ "$effective_decision" == "close" ]]; then
      if [[ "$action" == "Reopen" ]]; then
        return
      fi
    elif [[ "$effective_decision" == "reopen" ]]; then
      issue_directive_resolution["$issue_key"]="Resolved via directive decision => reopen."
      issue_directive_final_action["$issue_key"]="reopen"
      if [[ -z "${issue_category[$issue_key]:-}" ]]; then
        issue_category["$issue_key"]="$default_category"
      fi
      unset "seen_issue[$issue_key]"
      unset "issue_action[$issue_key]"
      unset "seen_reopen_issue[$issue_key]"
      unset "reopen_issue_category[$issue_key]"
      return
    fi
  fi

  if [[ -n "$inferred_conflict_reason" ]]; then
    issue_directive_conflict_reason["$issue_key"]="$inferred_conflict_reason"
    issue_directive_conflict_action["$issue_key"]="$action"
    return
  fi

  issue_labels_raw="$(pr_issue_labels "$issue_number")"
  title_category="$(pr_issue_title_category "$issue_number")"
  effective_category="$(pr_resolve_effective_category "$default_category" "$issue_labels_raw" "$title_category")"

  if [[ "$action" == "Closes" ]]; then
    is_pr_ref="$(pr_is_pull_request_ref "$issue_number")"
    if [[ "$is_pr_ref" == "true" ]]; then
      return
    fi

    non_compliance_reason="$(pr_issue_non_compliance_reason_for "$issue_number" "$issue_labels_raw")"
    if [[ -n "$non_compliance_reason" ]]; then
      issue_non_compliance_skip["$issue_key"]="$non_compliance_reason"
      issue_non_compliance_action["$issue_key"]="$action"
      return
    fi
  fi

  if [[ "$action" == "Reopen" ]]; then
    pr_mark_reopen_issue "$issue_key" "$effective_category"
    if [[ -n "${seen_issue[$issue_key]:-}" ]]; then
      unset "seen_issue[$issue_key]"
      unset "issue_action[$issue_key]"
      unset "issue_category[$issue_key]"
    fi
    return
  fi

  seen_issue["$issue_key"]=1
  issue_action["$issue_key"]="$action"
  issue_category["$issue_key"]="$effective_category"
}
