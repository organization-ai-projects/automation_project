#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# Shared validator for required issue title/body format.
# Contract source of truth: .github/issue_required_fields.conf

set -u

issue_contract_file() {
  local root=""
  root="$(git rev-parse --show-toplevel 2>/dev/null || true)"
  if [[ -n "$root" && -f "$root/.github/issue_required_fields.conf" ]]; then
    echo "$root/.github/issue_required_fields.conf"
    return
  fi
  echo ".github/issue_required_fields.conf"
}

issue_contract_load() {
  local contract
  contract="$(issue_contract_file)"
  if [[ ! -f "$contract" ]]; then
    echo "Missing issue contract file: ${contract}" >&2
    return 1
  fi
  # shellcheck disable=SC1090
  source "$contract"
}

issue_contract_profile_for_labels() {
  local labels_raw="${1:-}"
  local lower_labels
  lower_labels="$(echo "$labels_raw" | tr '[:upper:]' '[:lower:]')"
  if [[ "$lower_labels" =~ (^|\|\|)review(\|\||$) ]]; then
    echo "review"
    return
  fi
  echo "default"
}

issue_contract_key_for_profile() {
  local profile="${1:-default}"
  local base_key="${2:-}"
  if [[ -z "$base_key" ]]; then
    echo ""
    return
  fi
  if [[ "$profile" == "review" ]]; then
    echo "ISSUE_REVIEW_${base_key}"
    return
  fi
  echo "ISSUE_${base_key}"
}

issue_contract_get() {
  local key="${1:-}"
  if [[ -z "$key" ]]; then
    echo ""
    return
  fi
  # shellcheck disable=SC2154
  echo "${!key-}"
}

trim_whitespace() {
  local s="${1:-}"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf "%s" "$s"
}

issue_extract_field_value() {
  local body="${1:-}"
  local field="${2:-}"
  awk -v field="$field" '
    BEGIN { IGNORECASE = 1 }
    {
      line = $0
      pattern = "^[[:space:]]*" field "[[:space:]]*:[[:space:]]*"
      if (line ~ pattern) {
        sub(pattern, "", line)
        print line
        exit
      }
    }
  ' <<< "$body"
}

issue_validate_title() {
  local title="${1:-}"
  local labels_raw="${2:-}"
  local profile
  local regex_key
  local regex

  issue_contract_load || return 1
  profile="$(issue_contract_profile_for_labels "$labels_raw")"
  regex_key="$(issue_contract_key_for_profile "$profile" "TITLE_REGEX")"
  regex="$(issue_contract_get "$regex_key")"
  if [[ -z "$regex" ]]; then
    echo "invalid_contract|title|Missing contract key: ${regex_key}"
    return
  fi
  if [[ ! "$title" =~ $regex ]]; then
    echo "invalid_title|title|Title must match regex: ${regex}"
  fi
}

issue_validate_body() {
  local body="${1:-}"
  local labels_raw="${2:-}"
  local profile
  local sections_key
  local fields_key
  local required_sections
  local required_fields
  local section
  local rule
  local field_name
  local field_regex
  local field_help
  local field_value

  issue_contract_load || return 1
  profile="$(issue_contract_profile_for_labels "$labels_raw")"
  sections_key="$(issue_contract_key_for_profile "$profile" "REQUIRED_SECTIONS")"
  fields_key="$(issue_contract_key_for_profile "$profile" "REQUIRED_FIELDS")"
  required_sections="$(issue_contract_get "$sections_key")"
  required_fields="$(issue_contract_get "$fields_key")"

  while IFS= read -r section; do
    section="$(trim_whitespace "$section")"
    [[ -z "$section" ]] && continue
    if ! grep -qF "$section" <<< "$body"; then
      echo "missing_section|${section}|Missing required section: ${section}"
    fi
  done <<< "${required_sections:-}"

  while IFS= read -r rule; do
    [[ -z "$rule" ]] && continue
    IFS=$'\t' read -r field_name field_regex field_help <<< "$rule"
    field_name="$(trim_whitespace "${field_name:-}")"
    field_regex="$(trim_whitespace "${field_regex:-}")"
    field_help="$(trim_whitespace "${field_help:-}")"
    [[ -z "$field_name" || -z "$field_regex" ]] && continue

    field_value="$(issue_extract_field_value "$body" "$field_name")"
    field_value="$(trim_whitespace "${field_value:-}")"
    if [[ -z "$field_value" ]]; then
      echo "missing_field|${field_name}|Missing required field: ${field_name}:"
      continue
    fi
    if [[ ! "$field_value" =~ $field_regex ]]; then
      echo "invalid_field|${field_name}|Invalid ${field_name}: '${field_value}' (expected: ${field_help})"
    fi
  done <<< "${required_fields:-}"
}

issue_validate_content() {
  local title="${1:-}"
  local body="${2:-}"
  local labels_raw="${3:-}"
  issue_validate_title "$title" "$labels_raw"
  issue_validate_body "$body" "$labels_raw"
}

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

  lower_labels="$(echo "$labels_raw" | tr '[:upper:]' '[:lower:]')"
  if [[ "$lower_labels" =~ (^|\|\|)issue-required-missing(\|\||$) ]]; then
    echo "label issue-required-missing is set on issue"
    return
  fi

  validations="$(issue_validate_content "$title" "$body" "$labels_raw" || true)"
  if [[ -z "$validations" ]]; then
    echo ""
    return
  fi

  first_reason="$(issue_first_validation_reason "$validations")"
  echo "$first_reason"
}

issue_fetch_non_compliance_reason() {
  local issue_number="${1:-}"
  local repo_name="${2:-}"
  local issue_json
  local labels_raw
  local title
  local body

  if [[ -n "$repo_name" ]]; then
    issue_json="$(gh issue view "$issue_number" -R "$repo_name" --json labels,title,body 2>/dev/null || true)"
  else
    issue_json="$(gh issue view "$issue_number" --json labels,title,body 2>/dev/null || true)"
  fi
  if [[ -z "$issue_json" ]]; then
    echo ""
    return
  fi

  labels_raw="$(echo "$issue_json" | jq -r '.labels | map(.name) | join("||")')"
  title="$(echo "$issue_json" | jq -r '.title // ""')"
  body="$(echo "$issue_json" | jq -r '.body // ""')"

  issue_non_compliance_reason_from_content "$title" "$body" "$labels_raw"
}
