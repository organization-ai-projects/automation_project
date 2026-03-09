#!/usr/bin/env bash
# shellcheck shell=bash

# Core issue-reference parsing helpers.

_parse_issue_refs_by_mode() {
  local text="$1"
  local mode="$2"

  echo "$text" | awk -v mode="$mode" '
    BEGIN {
      pattern = ""
      if (mode == "close") {
        pattern = "(closes|fixes)[[:space:]]+[^[:space:]]*#[0-9]+"
      } else if (mode == "all_close") {
        pattern = "(closes|fixes)[[:space:]]+(rejected[[:space:]]+)?[^[:space:]]*#[0-9]+"
      } else if (mode == "neutralized_close") {
        pattern = "(closes|fixes)[[:space:]]+rejected[[:space:]]+[^[:space:]]*#[0-9]+"
      } else if (mode == "non_close") {
        pattern = "(part[[:space:]]+of)[[:space:]]+[^[:space:]]*#[0-9]+"
      } else if (mode == "reopen") {
        pattern = "(reopen|reopens)[[:space:]]+[^[:space:]]*#[0-9]+"
      } else if (mode == "directive_events") {
        pattern = "(closes|fixes|reopen|reopens)[[:space:]]+[^[:space:]]*#[0-9]+"
      }
    }
    {
      line = $0
      lower = tolower($0)

      while (pattern != "" && match(lower, pattern)) {
        if (RSTART > 1 && substr(lower, RSTART - 1, 1) ~ /[[:alnum:]_]/) {
          lower = substr(lower, RSTART + 1)
          line = substr(line, RSTART + 1)
          continue
        }

        matched = substr(line, RSTART, RLENGTH)
        matched_lower = substr(lower, RSTART, RLENGTH)
        n = split(matched, parts, /[[:space:]]+/)
        split(matched_lower, parts_lower, /[[:space:]]+/)

        token = parts_lower[1]
        token_b = parts_lower[2]
        issue_ref = parts[n]
        sub(/^.*#/, "#", issue_ref)

        action = ""
        if (mode == "close" || mode == "all_close" || mode == "neutralized_close") {
          if (token == "closes" || token == "fixes") {
            action = "Closes"
          }
        } else if (mode == "non_close") {
          if (token == "part" && token_b == "of") {
            action = "Part of"
          }
        } else if (mode == "reopen") {
          if (token == "reopen" || token == "reopens") {
            action = "Reopen"
          }
        } else if (mode == "directive_events") {
          if (token == "closes" || token == "fixes") {
            action = "Closes"
          } else if (token == "reopen" || token == "reopens") {
            action = "Reopen"
          }
        }

        if (issue_ref ~ /^#[0-9]+$/ && action != "") {
          print action "|" issue_ref
        }

        lower = substr(lower, RSTART + RLENGTH)
        line = substr(line, RSTART + RLENGTH)
      }
    }
  '
}

parse_closing_issue_refs_from_text() {
  local text="$1"
  parse_issue_directive_records_from_text "$text" | awk -F'|' '$1 == "EV" && $2 == "Closes" { print $2 "|" $3 }' | sort -u
}

parse_pr_body_closing_issue_refs_from_text() {
  local text="$1"
  # Semantic alias: PR body parsing rules currently match generic closing-reference rules.
  parse_closing_issue_refs_from_text "$text"
}

parse_non_closing_issue_refs_from_text() {
  local text="$1"
  _parse_issue_refs_by_mode "$text" "non_close" | sort -u
}

parse_neutralized_closing_issue_refs_from_text() {
  local text="$1"
  _parse_issue_refs_by_mode "$text" "neutralized_close" | sort -u
}

parse_all_closing_issue_refs_from_text() {
  local text="$1"
  _parse_issue_refs_by_mode "$text" "all_close" | sort -u
}

parse_issue_directive_records_from_text() {
  local text="$1"
  local native_output

  if native_output="$(_parse_issue_directive_records_via_va "$text" 2>/dev/null)"; then
    [[ -n "$native_output" ]] && printf '%s\n' "$native_output"
    return 0
  fi

  echo "$text" | awk '
    {
      line = $0
      lower = tolower($0)

      # Directive decisions: "directive decision: #123 => close|reopen"
      decision_line = line
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
      duplicate_line = line
      duplicate_lower = lower
      while (match(duplicate_lower, /#([0-9]+)[[:space:]]+duplicate[[:space:]]+of[[:space:]]+#([0-9]+)/)) {
        matched = substr(duplicate_line, RSTART, RLENGTH)
        gsub(/[^0-9]+/, " ", matched)
        split(matched, nums, " ")
        if (nums[1] != "" && nums[2] != "") {
          print "DUP|#" nums[1] "|#" nums[2]
        }
        duplicate_lower = substr(duplicate_lower, RSTART + RLENGTH)
        duplicate_line = substr(duplicate_line, RSTART + RLENGTH)
      }

      # Directive events: closes/fixes/reopen/reopens #N
      event_line = line
      event_lower = lower
      while (match(event_lower, /(closes|fixes|reopen|reopens)[[:space:]]+[^[:space:]]*#[0-9]+/)) {
        if (RSTART > 1 && substr(event_lower, RSTART - 1, 1) ~ /[[:alnum:]_]/) {
          event_lower = substr(event_lower, RSTART + 1)
          event_line = substr(event_line, RSTART + 1)
          continue
        }

        matched = substr(event_line, RSTART, RLENGTH)
        matched_lower = substr(event_lower, RSTART, RLENGTH)
        n = split(matched, parts, /[[:space:]]+/)
        split(matched_lower, parts_lower, /[[:space:]]+/)

        token = parts_lower[1]
        issue_ref = parts[n]
        sub(/^.*#/, "#", issue_ref)

        action = ""
        if (token == "closes" || token == "fixes") {
          action = "Closes"
        } else if (token == "reopen" || token == "reopens") {
          action = "Reopen"
        }

        if (issue_ref ~ /^#[0-9]+$/ && action != "") {
          print "EV|" action "|" issue_ref
        }

        event_lower = substr(event_lower, RSTART + RLENGTH)
        event_line = substr(event_line, RSTART + RLENGTH)
      }
    }
  '
}

_parse_issue_directive_records_via_va() {
  local text="$1"
  local tmp_file
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

  tmp_file="$(mktemp)"
  printf '%s' "$text" >"$tmp_file"
  if "${cmd[@]}" --input-file "$tmp_file" --format plain; then
    rm -f "$tmp_file"
    return 0
  fi
  rm -f "$tmp_file"
  return 1
}

parse_directive_events_from_text() {
  local text="$1"
  parse_issue_directive_records_from_text "$text" | awk -F'|' '$1 == "EV" { print $2 "|" $3 }'
}

parse_reopen_issue_refs_from_text() {
  local text="$1"
  parse_issue_directive_records_from_text "$text" | awk -F'|' '$1 == "EV" && $2 == "Reopen" { print $2 "|" $3 }' | sort -u
}

parse_duplicate_refs_from_text() {
  local text="$1"
  parse_issue_directive_records_from_text "$text" | awk -F'|' '$1 == "DUP" { print $2 "|" $3 }' | sort -u
}

parse_directive_decisions_from_text() {
  local text="$1"
  parse_issue_directive_records_from_text "$text" | awk -F'|' '$1 == "DEC" { print $2 "|" $3 }' | sort -u
}

normalize_issue_key() {
  local raw="${1:-}"

  if [[ "$raw" =~ \#([0-9]+) ]]; then
    echo "#${BASH_REMATCH[1]}"
    return 0
  fi

  return 1
}
