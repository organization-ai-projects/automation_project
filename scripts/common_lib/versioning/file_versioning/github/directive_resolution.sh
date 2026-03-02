#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# Shared directive arbitration helpers (Closes/Fixes/Reopen).
# Depends on parse/normalize helpers from issue_refs.sh.

normalize_directive_issue_key() {
  local issue_key_raw="$1"
  local issue_key=""
  issue_key="$(normalize_issue_key "$issue_key_raw" 2>/dev/null || true)"
  if [[ "$issue_key" =~ ^#[0-9]+$ ]]; then
    printf '%s\n' "$issue_key"
  fi
}

is_supported_directive_decision() {
  local decision="${1:-}"
  [[ "$decision" == "close" || "$decision" == "reopen" ]]
}

directive_resolution_allow_inferred() {
  local commit_messages="$1"
  local branch_count
  branch_count="$(count_distinct_source_branches_from_commits "$commit_messages")"
  if [[ "${branch_count:-0}" -gt 1 ]]; then
    printf '%s\n' "false"
    return 0
  fi
  printf '%s\n' "true"
}

directive_unresolved_reason() {
  local allow_inferred="$1"
  if [[ "$allow_inferred" == "true" ]]; then
    printf '%s\n' "closes-and-reopen-without-explicit-decision"
  else
    printf '%s\n' "closes-and-reopen-across-multiple-source-branches"
  fi
}

count_distinct_source_branches_from_commits() {
  local commit_messages="$1"
  printf '%s\n' "$commit_messages" \
    | sed -nE 's@.*Merge pull request #[0-9]+ from [^/]+/(.+)@\1@p' \
    | sort -u \
    | sed '/^$/d' \
    | wc -l \
    | tr -d ' '
}

resolve_issue_directives() {
  local payload_text="$1"
  local decision_text="$2"
  local commit_messages="$3"
  local allow_inferred

  allow_inferred="$(directive_resolution_allow_inferred "$commit_messages")"

  declare -A has_close
  declare -A has_reopen
  declare -A explicit_decision
  declare -A inferred_decision
  declare -A issues

  while IFS='|' read -r action issue_key; do
    issue_key="$(normalize_directive_issue_key "$issue_key")"
    [[ -n "$issue_key" ]] || continue
    issues["$issue_key"]=1
    case "$action" in
      Closes)
        has_close["$issue_key"]=1
        inferred_decision["$issue_key"]="close"
        ;;
      Reopen)
        has_reopen["$issue_key"]=1
        inferred_decision["$issue_key"]="reopen"
        ;;
    esac
  done < <(parse_directive_events_from_text "$payload_text")

  while IFS='|' read -r issue_key decision; do
    issue_key="$(normalize_directive_issue_key "$issue_key")"
    decision="$(printf '%s' "$decision" | tr '[:upper:]' '[:lower:]')"
    [[ -n "$issue_key" ]] || continue
    is_supported_directive_decision "$decision" || continue
    explicit_decision["$issue_key"]="$decision"
    issues["$issue_key"]=1
  done < <(parse_directive_decisions_from_text "$decision_text")

  for issue_key in "${!issues[@]}"; do
    local close_flag reopen_flag decision source reason
    close_flag="${has_close[$issue_key]:-0}"
    reopen_flag="${has_reopen[$issue_key]:-0}"
    decision=""
    source=""
    reason=""

    if [[ -n "${explicit_decision[$issue_key]:-}" ]]; then
      decision="${explicit_decision[$issue_key]}"
      source="explicit"
    elif [[ "$close_flag" == "1" && "$reopen_flag" == "1" ]]; then
      if [[ "$allow_inferred" == "true" && -n "${inferred_decision[$issue_key]:-}" ]]; then
        decision="${inferred_decision[$issue_key]}"
        source="inferred"
      else
        source="unresolved"
        reason="$(directive_unresolved_reason "$allow_inferred")"
      fi
    elif [[ "$reopen_flag" == "1" ]]; then
      decision="reopen"
      source="direct"
    elif [[ "$close_flag" == "1" ]]; then
      decision="close"
      source="direct"
    fi

    printf '%s|%s|%s|%s|%s|%s\n' \
      "$issue_key" "$close_flag" "$reopen_flag" "$decision" "$source" "$reason"
  done | sort -t'|' -k1,1V
}
