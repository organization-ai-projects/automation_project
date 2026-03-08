#!/usr/bin/env bash

auto_link_validate_contract_or_exit() {
  local repo_name="$1"
  local issue_number="$2"
  local marker="$3"
  local label_required_missing="$4"
  local label_automation_failed="$5"
  local issue_title="$6"
  local issue_body="$7"
  local issue_labels_raw="$8"

  local contract_errors
  contract_errors="$(issue_validate_content "$issue_title" "$issue_body" "$issue_labels_raw" || true)"
  if [[ -n "$contract_errors" ]]; then
    local summary_lines=""
    while IFS='|' read -r _ _ message; do
      [[ -z "$message" ]] && continue
      summary_lines+="- ${message}"$'\n'
    done <<<"$contract_errors"
    auto_link_set_validation_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Issue body/title is non-compliant with required issue format." \
      "Detected problems:

${summary_lines}
Expected contract source: \`.github/issue_required_fields.conf\`."
    exit 0
  fi
}

auto_link_extract_parent_or_exit() {
  local repo_name="$1"
  local issue_number="$2"
  local marker="$3"
  local label_required_missing="$4"
  local label_automation_failed="$5"
  local issue_body="$6"

  local parent_raw
  parent_raw="$(auto_link_extract_parent_field_value "$issue_body")"
  parent_raw="$(auto_link_trim "${parent_raw:-}")"

  if [[ -z "$parent_raw" ]]; then
    auto_link_set_validation_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Missing required field \`Parent:\` in issue body." \
      "Expected format:
\n- \`Parent: #<issue_number>\` for child issues
\n- \`Parent: none\` for independent issues
\n- \`Parent: base\` for cascade root issues
\n- \`Parent: epic\` for epic umbrella issues"
    exit 0
  fi

  echo "$parent_raw"
}
