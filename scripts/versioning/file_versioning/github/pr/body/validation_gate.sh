#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Validation Gate section helpers.

pr_build_validation_gate_section() {
  local ci_status_with_symbol="$1"
  local breaking_detected="$2"
  local breaking_scope_crates="$3"
  local breaking_scope_commits="$4"

  {
    echo "### Validation Gate"
    echo ""
    echo "- CI: ${ci_status_with_symbol}"
    if [[ "$breaking_detected" -eq 1 ]]; then
      echo "- Breaking change"
      echo "- Breaking scope:"
      if [[ -n "$breaking_scope_crates" ]]; then
        echo "  - crate(s): ${breaking_scope_crates}"
      else
        echo "  - crate(s): unknown"
      fi
      if [[ -n "$breaking_scope_commits" ]]; then
        echo "  - source commit(s): ${breaking_scope_commits}"
      else
        echo "  - source commit(s): unknown"
      fi
    else
      echo "- No breaking change"
    fi
  }
}

pr_replace_validation_gate_in_body() {
  local original_body="$1"
  local replacement="$2"
  pr_replace_top_level_section_in_body "$original_body" "### Validation Gate" "$replacement"
}

