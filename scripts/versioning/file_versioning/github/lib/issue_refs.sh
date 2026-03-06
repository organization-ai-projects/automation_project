#!/usr/bin/env bash

# Shared issue-reference parsing helpers for PR generation and audit scripts.

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
  _parse_issue_refs_by_mode "$text" "close" | sort -u
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

parse_reopen_issue_refs_from_text() {
  local text="$1"
  _parse_issue_refs_by_mode "$text" "reopen" | sort -u
}

_parse_special_issue_refs_by_mode() {
  local text="$1"
  local mode="$2"

  echo "$text" | awk -v mode="$mode" '
    {
      line = $0
      lower = tolower($0)

      if (mode == "duplicate") {
        while (match(lower, /#([0-9]+)[[:space:]]+duplicate[[:space:]]+of[[:space:]]+#([0-9]+)/)) {
          matched = substr(line, RSTART, RLENGTH)
          gsub(/[^0-9]+/, " ", matched)
          split(matched, nums, " ")
          if (nums[1] != "" && nums[2] != "") {
            print "#" nums[1] "|" "#" nums[2]
          }
          lower = substr(lower, RSTART + RLENGTH)
          line = substr(line, RSTART + RLENGTH)
        }
      } else if (mode == "directive_decision") {
        while (match(lower, /directive[[:space:]_-]*decision[[:space:]]*:[[:space:]]*[^[:space:]]*#[0-9]+[[:space:]]*=>[[:space:]]*(close|reopen)/)) {
          matched = substr(lower, RSTART, RLENGTH)
          issue_ref = matched
          decision = matched
          sub(/^.*#/, "#", issue_ref)
          sub(/[[:space:]]*=>.*/, "", issue_ref)
          sub(/^.*=>[[:space:]]*/, "", decision)
          gsub(/[[:space:]]+/, "", issue_ref)
          gsub(/[[:space:]]+/, "", decision)
          if (issue_ref ~ /^#[0-9]+$/ && (decision == "close" || decision == "reopen")) {
            print issue_ref "|" decision
          }
          lower = substr(lower, RSTART + RLENGTH)
        }
      }
    }
  ' | sort -u
}

parse_duplicate_refs_from_text() {
  local text="$1"
  _parse_special_issue_refs_by_mode "$text" "duplicate"
}

parse_directive_decisions_from_text() {
  local text="$1"
  _parse_special_issue_refs_by_mode "$text" "directive_decision"
}

parse_directive_events_from_text() {
  local text="$1"
  # Keep source order (no sort) to preserve event chronology semantics.
  _parse_issue_refs_by_mode "$text" "directive_events"
}

normalize_issue_key() {
  local raw="${1:-}"

  if [[ "$raw" =~ \#([0-9]+) ]]; then
    echo "#${BASH_REMATCH[1]}"
    return 0
  fi

  return 1
}
