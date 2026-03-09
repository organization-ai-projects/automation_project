#!/usr/bin/env bash
# shellcheck shell=bash

# Core issue-reference parsing helpers.

_issue_refs_cache_key_for_text() {
  local text="$1"
  # cksum returns checksum + byte-length; stable enough for in-process cache keys.
  printf '%s' "$text" | cksum | awk '{ print $1 ":" $2 }'
}

_parse_issue_directive_event_refs() {
  local text="$1"
  local event_action="$2"
  local emitted_action="${3:-$event_action}"
  parse_issue_directive_records_from_text "$text" | awk -F'|' -v event_action="$event_action" -v emitted_action="$emitted_action" '$1 == "EV" && $2 == event_action { print emitted_action "|" $3 }'
}

_parse_issue_directive_records_by_type() {
  local text="$1"
  local record_type="$2"
  parse_issue_directive_records_from_text "$text" | awk -F'|' -v record_type="$record_type" '$1 == record_type { print $2 "|" $3 }'
}

parse_closing_issue_refs_from_text() {
  local text="$1"
  _parse_issue_directive_event_refs "$text" "Closes" | sort -u
}

parse_pr_body_closing_issue_refs_from_text() {
  local text="$1"
  # Semantic alias: PR body parsing rules currently match generic closing-reference rules.
  parse_closing_issue_refs_from_text "$text"
}

parse_non_closing_issue_refs_from_text() {
  local text="$1"
  _parse_issue_directive_event_refs "$text" "Part of" | sort -u
}

parse_neutralized_closing_issue_refs_from_text() {
  local text="$1"
  _parse_issue_directive_event_refs "$text" "Closes rejected" "Closes" | sort -u
}

parse_all_closing_issue_refs_from_text() {
  local text="$1"
  parse_issue_directive_records_from_text "$text" | awk -F'|' '
    $1 == "EV" && ($2 == "Closes" || $2 == "Closes rejected") { print "Closes|" $3 }
  ' | sort -u
}

parse_issue_directive_records_from_text() {
  local text="$1"
  local native_output
  local cache_key

  if [[ "${issue_refs_records_cache_initialized:-0}" != "1" ]]; then
    declare -gA issue_refs_records_cache
    issue_refs_records_cache_initialized="1"
  fi

  cache_key="$(_issue_refs_cache_key_for_text "$text")"
  if [[ -v "issue_refs_records_cache[$cache_key]" ]]; then
    [[ -n "${issue_refs_records_cache[$cache_key]}" ]] && printf '%s\n' "${issue_refs_records_cache[$cache_key]}"
    return 0
  fi

  if native_output="$(_parse_issue_directive_records_via_va "$text" 2>/dev/null)"; then
    issue_refs_records_cache["$cache_key"]="$native_output"
    [[ -n "$native_output" ]] && printf '%s\n' "$native_output"
    return 0
  fi

  native_output="$(echo "$text" | awk '
    {
      lower = tolower($0)

      # Directive decisions: "directive decision: #123 => close|reopen"
      decision_lower = lower
      while (match(decision_lower, /directive[[:space:]_-]*decision[[:space:]]*:[[:space:]]*[^[:space:]]*#[0-9]+[[:space:]]*=>[[:space:]]*(close|reopen)/)) {
        matched = substr(decision_lower, RSTART, RLENGTH)
        issue_ref = matched
        decision = matched
        sub(/^.*#/, "#", issue_ref)
        sub(/[[:space:]]*=>.*/, "", issue_ref)
        sub(/^.*=>[[:space:]]*/, "", decision)
        gsub(/[[:space:]]+/, "", issue_ref)
        gsub(/[[:space:]]+/, "", decision)
        if (issue_ref ~ /^#[0-9]+$/ && (decision == "close" || decision == "reopen")) {
          print "DEC|" issue_ref "|" decision
        }
        decision_lower = substr(decision_lower, RSTART + RLENGTH)
      }

      # Duplicate declarations: "#123 duplicate of #456"
      duplicate_lower = lower
      while (match(duplicate_lower, /#([0-9]+)[[:space:]]+duplicate[[:space:]]+of[[:space:]]+#([0-9]+)/)) {
        matched = substr(duplicate_lower, RSTART, RLENGTH)
        gsub(/[^0-9]+/, " ", matched)
        split(matched, nums, " ")
        if (nums[1] != "" && nums[2] != "") {
          print "DUP|#" nums[1] "|#" nums[2]
        }
        duplicate_lower = substr(duplicate_lower, RSTART + RLENGTH)
      }

      # Directive events: closes/fixes/reopen/reopens/part of #N
      event_lower = lower
      while (match(event_lower, /(closes|fixes|reopen|reopens|part[[:space:]]+of)[[:space:]]+(rejected[[:space:]]+)?[^[:space:]]*#[0-9]+/)) {
        if (RSTART > 1 && substr(event_lower, RSTART - 1, 1) ~ /[[:alnum:]_]/) {
          event_lower = substr(event_lower, RSTART + 1)
          continue
        }

        matched_lower = substr(event_lower, RSTART, RLENGTH)
        n = split(matched_lower, parts_lower, /[[:space:]]+/)

        token = parts_lower[1]
        issue_ref = parts_lower[n]
        sub(/^.*#/, "#", issue_ref)
        rejected_token = parts_lower[2]

        action = ""
        if (token == "closes" || token == "fixes") {
          if (rejected_token == "rejected") {
            action = "Closes rejected"
          } else {
            action = "Closes"
          }
        } else if (token == "part" && parts_lower[2] == "of") {
          action = "Part of"
        } else if (token == "reopen" || token == "reopens") {
          action = "Reopen"
        }

        if (issue_ref ~ /^#[0-9]+$/ && action != "") {
          print "EV|" action "|" issue_ref
        }

        event_lower = substr(event_lower, RSTART + RLENGTH)
      }
    }
  ')"
  issue_refs_records_cache["$cache_key"]="$native_output"
  [[ -n "$native_output" ]] && printf '%s\n' "$native_output"
}

_parse_issue_directive_records_via_va() {
  local text="$1"
  local -a cmd

  cmd=()
  if [[ -n "${VA_PR_DIRECTIVES_BIN:-}" ]]; then
    cmd=("${VA_PR_DIRECTIVES_BIN}" pr directives)
  elif command -v va >/dev/null 2>&1; then
    cmd=(va pr directives)
  elif command -v versioning_automation >/dev/null 2>&1; then
    cmd=(versioning_automation pr directives)
  else
    return 1
  fi

  if printf '%s' "$text" | "${cmd[@]}" --stdin --format plain; then
    return 0
  fi
  return 1
}

parse_directive_events_from_text() {
  local text="$1"
  _parse_issue_directive_records_by_type "$text" "EV"
}

parse_reopen_issue_refs_from_text() {
  local text="$1"
  _parse_issue_directive_event_refs "$text" "Reopen" | sort -u
}

parse_duplicate_refs_from_text() {
  local text="$1"
  _parse_issue_directive_records_by_type "$text" "DUP" | sort -u
}

parse_directive_decisions_from_text() {
  local text="$1"
  _parse_issue_directive_records_by_type "$text" "DEC" | sort -u
}

normalize_issue_key() {
  local raw="${1:-}"

  if [[ "$raw" =~ \#([0-9]+) ]]; then
    echo "#${BASH_REMATCH[1]}"
    return 0
  fi

  return 1
}
