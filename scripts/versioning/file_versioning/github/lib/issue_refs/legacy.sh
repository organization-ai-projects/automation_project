#!/usr/bin/env bash
# shellcheck shell=bash

# Legacy (awk-based) issue-directive parser used as fallback when `va` is unavailable.

parse_issue_directive_records_legacy_from_text() {
  local text="$1"

  echo "$text" | awk '
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
  '
}
