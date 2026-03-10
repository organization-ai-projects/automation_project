#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Issue collection and categorization helpers.

pr_issue_parse_key_and_number() {
  local issue_key_raw="$1"
  local _out_key_var="$2"
  local _out_number_var="$3"
  local normalized_issue_key=""
  local normalized_issue_number=""
  local -n _out_key_ref="$_out_key_var"
  local -n _out_number_ref="$_out_number_var"

  normalized_issue_key="$(normalize_issue_key "$issue_key_raw" || true)"
  if [[ -z "$normalized_issue_key" ]]; then
    _out_key_ref=""
    _out_number_ref=""
    return 1
  fi

  normalized_issue_number="${normalized_issue_key//#/}"
  _out_key_ref="$normalized_issue_key"
  _out_number_ref="$normalized_issue_number"
  return 0
}

pr_issue_payload_field() {
  local payload="$1"
  local field_name="$2"
  local first second third

  first="${payload%%$'\x1f'*}"
  second="${payload#*$'\x1f'}"
  if [[ "$second" == "$payload" ]]; then
    second=""
    third=""
  else
    third="${second#*$'\x1f'}"
    if [[ "$third" == "$second" ]]; then
      third=""
    else
      second="${second%%$'\x1f'*}"
    fi
  fi

  case "$field_name" in
  labels) printf '%s' "$first" ;;
  title_category) printf '%s' "$second" ;;
  non_compliance_reason) printf '%s' "$third" ;;
  *) return 1 ;;
  esac
}

pr_issue_context_payload_for() {
  local issue_number="$1"
  local issue_key="#${issue_number}"
  local issue_json
  local title
  local body
  local labels_raw=""
  local title_category="Unknown"
  local title_category_from_va=""
  local reason=""

  if [[ "${issue_context_cached[$issue_key]:-0}" == "1" ]]; then
    printf "%s\x1f%s\x1f%s" \
      "${issue_labels_cache[$issue_key]:-}" \
      "${issue_title_category_cache[$issue_key]:-Unknown}" \
      "${issue_non_compliance_reason_cache[$issue_key]:-}"
    return
  fi

  if [[ "$has_gh" == "true" ]]; then
    issue_json="$(pr_issue_view_full_json "$issue_number")"
    if [[ -n "$issue_json" ]]; then
      labels_raw="$(echo "$issue_json" | jq -r '.labels // [] | map(.name) | join("||")')"
      title="$(echo "$issue_json" | jq -r '.title // ""')"
      body="$(echo "$issue_json" | jq -r '.body // ""')"

      if [[ -n "$title" ]]; then
        if command -v va_exec >/dev/null 2>&1; then
          title_category_from_va="$(
            va_exec pr issue-category-from-title \
              --title "$title" 2>/dev/null || true
          )"
        fi
        if [[ -n "$title_category_from_va" ]]; then
          title_category="$title_category_from_va"
        else
          title_category="$(issue_category_from_title "$title")"
        fi
      fi

      reason="$(issue_non_compliance_reason_from_content "$title" "$body" "$labels_raw")"
    fi
  fi

  issue_labels_cache["$issue_key"]="$labels_raw"
  issue_title_category_cache["$issue_key"]="$title_category"
  issue_non_compliance_reason_cache["$issue_key"]="$reason"
  issue_context_cached["$issue_key"]="1"

  printf "%s\x1f%s\x1f%s" "$labels_raw" "$title_category" "$reason"
}

pr_resolve_effective_category() {
  local default_category="$1"
  local issue_labels_raw="$2"
  local title_category="$3"
  local label_category
  local effective_category

  if command -v va_exec >/dev/null 2>&1; then
    effective_category="$(
      va_exec pr effective-category \
        --labels-raw "$issue_labels_raw" \
        --title-category "$title_category" \
        --default-category "$default_category" 2>/dev/null || true
    )"
    if [[ -n "$effective_category" ]]; then
      echo "$effective_category"
      return
    fi

    label_category="$(
      va_exec pr issue-category-from-labels \
        --labels-raw "$issue_labels_raw" 2>/dev/null || true
    )"
  fi
  if [[ -z "${label_category:-}" ]]; then
    label_category="$(issue_category_from_labels "$issue_labels_raw")"
  fi
  if command -v va_exec >/dev/null 2>&1; then
    effective_category="$(
      va_exec pr resolve-category \
        --label-category "$label_category" \
        --title-category "$title_category" \
        --default-category "$default_category" 2>/dev/null || true
    )"
    if [[ -n "$effective_category" ]]; then
      echo "$effective_category"
      return
    fi
  fi

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
  local payload
  payload="$(pr_issue_context_payload_for "$issue_number")"
  pr_issue_payload_field "$payload" "labels"
}

pr_issue_title_category() {
  local issue_number="$1"
  local payload
  payload="$(pr_issue_context_payload_for "$issue_number")"
  pr_issue_payload_field "$payload" "title_category"
}

pr_issue_non_compliance_reason_for() {
  local issue_number="$1"
  local payload
  local _labels_raw="${2:-}"
  payload="$(pr_issue_context_payload_for "$issue_number")"
  pr_issue_payload_field "$payload" "non_compliance_reason"
}

pr_is_pull_request_ref() {
  local issue_number="$1"
  local cache_key="#${issue_number}"
  local repo_name_with_owner
  local va_result=""

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

  if command -v va_exec >/dev/null 2>&1; then
    va_result="$(
      va_exec pr issue-ref-kind \
        --issue "$issue_number" \
        --repo "$repo_name_with_owner" 2>/dev/null || true
    )"
    if [[ "$va_result" == "true" ]]; then
      pr_ref_cache["$cache_key"]="1"
      echo "true"
      return
    fi
    if [[ "$va_result" == "false" ]]; then
      pr_ref_cache["$cache_key"]="0"
      echo "false"
      return
    fi
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
  local issue_key="" issue_number="" issue_context_payload="" issue_labels_raw="" title_category="" effective_category=""

  pr_issue_parse_key_and_number "$issue_key_raw" issue_key issue_number || return
  [[ -n "$issue_key" && -n "$issue_number" ]] || return
  issue_context_payload="$(pr_issue_context_payload_for "$issue_number")"
  issue_labels_raw="${issue_context_payload%%$'\x1f'*}"
  title_category="${issue_context_payload#*$'\x1f'}"
  title_category="${title_category%%$'\x1f'*}"
  effective_category="$(pr_resolve_effective_category "$default_category" "$issue_labels_raw" "$title_category")"
  seen_reopen_issue["$issue_key"]=1
  reopen_issue_category["$issue_key"]="$effective_category"
}
