#!/usr/bin/env bash
# shellcheck shell=bash

# Specialized issue-reference parsing helpers.

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

