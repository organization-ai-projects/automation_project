#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Issue collection and categorization helpers.

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

  if pr_repo_api_call "$repo_name_with_owner" "pulls/${issue_number}" >/dev/null 2>&1; then
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
