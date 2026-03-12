#!/usr/bin/env bash
# shellcheck shell=bash

# Non-compliance reason extraction helpers.

issue_first_validation_reason() {
  local validations="${1:-}"
  echo "$validations" | awk -F'|' 'NF>=3 {print $3; exit}'
}

issue_non_compliance_reason_from_content() {
  local title="${1:-}"
  local body="${2:-}"
  local labels_raw="${3:-}"
  local lower_labels
  local validations
  local first_reason

  if first_reason="$(
    va_exec issue non-compliance-reason \
      --title "$title" \
      --body "$body" \
      --labels-raw "$labels_raw" 2>/dev/null
  )"; then
    printf '%s' "$first_reason"
    return
  fi

  lower_labels="$(echo "$labels_raw" | tr '[:upper:]' '[:lower:]')"
  if [[ "$lower_labels" =~ (^|\|\|)issue-required-missing(\|\||$) ]]; then
    echo "label issue-required-missing is set on issue"
    return
  fi

  if ! validations="$(issue_validate_content "$title" "$body" "$labels_raw")"; then
    echo "issue contract could not be loaded"
    return
  fi
  if [[ -z "$validations" ]]; then
    echo ""
    return
  fi

  first_reason="$(issue_first_validation_reason "$validations")"
  echo "$first_reason"
}
