#!/usr/bin/env bash

# Shared issue-reference parsing helpers for PR generation and audit scripts.

parse_closing_issue_refs_from_text() {
  local text="$1"
  echo "$text" | awk '
    {
      line = $0
      lower = tolower($0)
      while (match(lower, /(closes|close|fixes|fix|resolves|resolve)[[:space:]]+[^[:space:]]*#[0-9]+/)) {
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
        issue_ref = parts[n]
        sub(/^.*#/, "#", issue_ref)

        if (token ~ /^clos/) {
          action = "Closes"
        } else if (token ~ /^fix/) {
          action = "Fixes"
        } else {
          action = "Resolves"
        }

        if (issue_ref ~ /^#[0-9]+$/) {
          print action "|" issue_ref
        }

        lower = substr(lower, RSTART + RLENGTH)
        line = substr(line, RSTART + RLENGTH)
      }
    }
  ' | sort -u
}

parse_non_closing_issue_refs_from_text() {
  local text="$1"
  echo "$text" | awk '
    {
      line = $0
      lower = tolower($0)
      while (match(lower, /(part[[:space:]]+of|related[[:space:]]+to)[[:space:]]+[^[:space:]]*#[0-9]+/)) {
        if (RSTART > 1 && substr(lower, RSTART - 1, 1) ~ /[[:alnum:]_]/) {
          lower = substr(lower, RSTART + 1)
          line = substr(line, RSTART + 1)
          continue
        }

        matched = substr(line, RSTART, RLENGTH)
        matched_lower = substr(lower, RSTART, RLENGTH)
        n = split(matched, parts, /[[:space:]]+/)
        split(matched_lower, parts_lower, /[[:space:]]+/)
        token_a = parts_lower[1]
        token_b = parts_lower[2]
        issue_ref = parts[n]
        sub(/^.*#/, "#", issue_ref)

        action = "Related to"
        if (token_a == "part" && token_b == "of") {
          action = "Part of"
        }

        if (issue_ref ~ /^#[0-9]+$/) {
          print action "|" issue_ref
        }

        lower = substr(lower, RSTART + RLENGTH)
        line = substr(line, RSTART + RLENGTH)
      }
    }
  ' | sort -u
}

parse_neutralized_closing_issue_refs_from_text() {
  local text="$1"
  # Matches "closes/fixes/resolves rejected #N" (previously neutralized refs).
  # The [^[:space:]]* segment allows optional owner/repo prefixes (e.g. "org/repo#42"),
  # consistent with parse_closing_issue_refs_from_text.
  echo "$text" | awk '
    {
      line = $0
      lower = tolower($0)
      while (match(lower, /(closes?|fixes?|resolves?)[[:space:]]+rejected[[:space:]]+[^[:space:]]*#[0-9]+/)) {
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
        issue_ref = parts[n]
        sub(/^.*#/, "#", issue_ref)

        if (token ~ /^clos/) {
          action = "Closes"
        } else if (token ~ /^fix/) {
          action = "Fixes"
        } else {
          action = "Resolves"
        }

        if (issue_ref ~ /^#[0-9]+$/) {
          print action "|" issue_ref
        }

        lower = substr(lower, RSTART + RLENGTH)
        line = substr(line, RSTART + RLENGTH)
      }
    }
  ' | sort -u
}

parse_duplicate_refs_from_text() {
  local text="$1"
  echo "$text" | awk '
    BEGIN { IGNORECASE = 1 }
    {
      line = $0
      while (match(line, /#([0-9]+)[[:space:]]+duplicate[[:space:]]+of[[:space:]]+#([0-9]+)/)) {
        matched = substr(line, RSTART, RLENGTH)
        gsub(/[^0-9]+/, " ", matched)
        split(matched, nums, " ")
        if (nums[1] != "" && nums[2] != "") {
          print "#" nums[1] "|" "#" nums[2]
        }
        line = substr(line, RSTART + RLENGTH)
      }
    }
  ' | sort -u
}

normalize_issue_key() {
  local raw="${1:-}"
  local normalized

  normalized="$(echo "$raw" | sed -nE 's/.*#([0-9]+).*/#\1/p')"
  if [[ "$normalized" =~ ^#[0-9]+$ ]]; then
    echo "$normalized"
    return 0
  fi

  return 1
}
