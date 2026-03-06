#!/usr/bin/env bash

# Issue-flow helpers extracted from generate_pr_description.sh.

pr_resolve_effective_category() {
  local default_category="$1"
  local issue_labels_raw="$2"
  local title_category="$3"
  local label_category
  local effective_category

  label_category="$(issue_category_from_labels "$issue_labels_raw")"
  effective_category="$label_category"
  if [[ "$effective_category" == "Unknown" || "$effective_category" == "Mixed" ]]; then
    if [[ "$title_category" != "Unknown" && "$title_category" != "Mixed" ]]; then
      effective_category="$title_category"
    fi
  fi
  if [[ "$effective_category" == "Unknown" && "$default_category" != "Unknown" ]]; then
    effective_category="$default_category"
  fi

  echo "$effective_category"
}

pr_issue_labels() {
  local issue_number="$1"
  local repo_name_with_owner

  if [[ "$has_gh" != "true" ]]; then
    pr_debug_log "issue_labels(#${issue_number}): gh unavailable, fallback empty labels."
    echo ""
    return
  fi

  repo_name_with_owner="$(pr_get_repo_name_with_owner)"

  if [[ -n "$repo_name_with_owner" ]]; then
    gh issue view "$issue_number" -R "$repo_name_with_owner" --json labels \
      -q '.labels | map(.name) | join("||")' 2>/dev/null || true
    return
  fi

  gh issue view "$issue_number" --json labels \
    -q '.labels | map(.name) | join("||")' 2>/dev/null || true
}

pr_issue_title_category() {
  local issue_number="$1"
  local issue_title

  if [[ "$has_gh" != "true" ]]; then
    echo "Unknown"
    return
  fi

  issue_title="$(gh issue view "$issue_number" --json title -q '.title // ""' 2>/dev/null || true)"
  if [[ -z "$issue_title" ]]; then
    echo "Unknown"
    return
  fi

  issue_category_from_title "$issue_title"
}

pr_issue_non_compliance_reason_for() {
  local issue_number="$1"
  local labels_raw="${2:-}"
  local issue_key="#${issue_number}"
  local issue_json
  local title
  local body
  local reason

  if [[ -n "${issue_non_compliance_reason_cache[$issue_key]:-}" ]]; then
    echo "${issue_non_compliance_reason_cache[$issue_key]}"
    return
  fi

  if [[ "$has_gh" != "true" ]]; then
    issue_non_compliance_reason_cache["$issue_key"]=""
    echo ""
    return
  fi

  issue_json="$(gh issue view "$issue_number" --json title,body 2>/dev/null || true)"
  if [[ -z "$issue_json" ]]; then
    issue_non_compliance_reason_cache["$issue_key"]=""
    echo ""
    return
  fi

  title="$(echo "$issue_json" | jq -r '.title // ""')"
  body="$(echo "$issue_json" | jq -r '.body // ""')"
  reason="$(issue_non_compliance_reason_from_content "$title" "$body" "$labels_raw")"
  issue_non_compliance_reason_cache["$issue_key"]="${reason}"
  echo "${reason}"
}

pr_is_pull_request_ref() {
  local issue_number="$1"
  local cache_key="#${issue_number}"
  local repo_name_with_owner

  if [[ -n "${pr_ref_cache[$cache_key]:-}" ]]; then
    if [[ "${pr_ref_cache[$cache_key]}" == "1" ]]; then
      echo "true"
    else
      echo "false"
    fi
    return
  fi

  if [[ "$has_gh" != "true" ]]; then
    pr_ref_cache["$cache_key"]="0"
    echo "false"
    return
  fi

  repo_name_with_owner="$(pr_get_repo_name_with_owner)"
  if [[ -z "$repo_name_with_owner" ]]; then
    pr_ref_cache["$cache_key"]="0"
    echo "false"
    return
  fi

  if gh api "repos/${repo_name_with_owner}/pulls/${issue_number}" >/dev/null 2>&1; then
    pr_ref_cache["$cache_key"]="1"
    echo "true"
  else
    pr_ref_cache["$cache_key"]="0"
    echo "false"
  fi
}

pr_mark_reopen_issue() {
  local issue_key_raw="$1"
  local default_category="$2"
  local issue_key issue_number issue_labels_raw title_category effective_category

  issue_key="$(normalize_issue_key "$issue_key_raw" || true)"
  [[ -z "$issue_key" ]] && return
  issue_number="${issue_key//#/}"
  issue_labels_raw="$(pr_issue_labels "$issue_number")"
  title_category="$(pr_issue_title_category "$issue_number")"
  effective_category="$(pr_resolve_effective_category "$default_category" "$issue_labels_raw" "$title_category")"
  seen_reopen_issue["$issue_key"]=1
  reopen_issue_category["$issue_key"]="$effective_category"
}

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

pr_add_duplicate_entry() {
  local duplicate_issue_raw="$1"
  local canonical_issue_raw="$2"
  local duplicate_issue_key canonical_issue_key

  duplicate_issue_key="$(normalize_issue_key "$duplicate_issue_raw" || true)"
  canonical_issue_key="$(normalize_issue_key "$canonical_issue_raw" || true)"
  [[ -z "$duplicate_issue_key" || -z "$canonical_issue_key" ]] && return

  duplicate_targets["$duplicate_issue_key"]="$canonical_issue_key"
}

pr_process_duplicate_mode() {
  local duplicate_issue_key canonical_issue_key duplicate_issue_number comment_body
  local repo_name_with_owner
  local auto_close_allowed="true"

  [[ -z "$duplicate_mode" ]] && return 0

  if [[ -z "${duplicate_targets+x}" || "${#duplicate_targets[@]}" -eq 0 ]]; then
    echo "Duplicate mode (${duplicate_mode}): no duplicate declarations detected."
    return 0
  fi

  if [[ "$dry_run" == "true" ]]; then
    echo "Duplicate mode (${duplicate_mode}): dry-run simulation; no GitHub mutation applied."
    return 0
  fi

  if [[ "$duplicate_mode" == "auto-close" && "$assume_yes" != "true" ]]; then
    auto_close_allowed="false"
    echo "Warning: duplicate auto-close requested without --yes; close action will be skipped." >&2
  fi

  repo_name_with_owner="$(pr_get_repo_name_with_owner)"
  if [[ -z "$repo_name_with_owner" ]]; then
    echo "Warning: unable to resolve repository; duplicate mode skipped." >&2
    return 0
  fi

  for duplicate_issue_key in "${!duplicate_targets[@]}"; do
    canonical_issue_key="${duplicate_targets[$duplicate_issue_key]}"
    duplicate_issue_number="${duplicate_issue_key//#/}"

    if [[ "$duplicate_mode" == "safe" ]]; then
      comment_body="Potential duplicate detected by PR generation workflow: ${duplicate_issue_key} may duplicate ${canonical_issue_key}. Please review manually."
    else
      comment_body="Duplicate of ${canonical_issue_key}"
    fi

    gh api "repos/${repo_name_with_owner}/issues/${duplicate_issue_number}/comments" \
      --raw-field body="${comment_body}" >/dev/null
    echo "Duplicate mode (${duplicate_mode}): commented on ${duplicate_issue_key} (target ${canonical_issue_key})."

    if [[ "$duplicate_mode" == "auto-close" && "$auto_close_allowed" == "true" ]]; then
      gh api -X PATCH "repos/${repo_name_with_owner}/issues/${duplicate_issue_number}" \
        -f state="closed" -f state_reason="not_planned" >/dev/null
      echo "Duplicate mode (${duplicate_mode}): closed ${duplicate_issue_key}."
    elif [[ "$duplicate_mode" == "auto-close" ]]; then
      echo "Duplicate mode (${duplicate_mode}): close skipped for ${duplicate_issue_key} (missing --yes)."
    fi
  done
}
