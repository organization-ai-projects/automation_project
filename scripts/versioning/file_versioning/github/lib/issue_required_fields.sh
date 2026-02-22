#!/usr/bin/env bash

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
  issue_contract_load || return 1
  if [[ ! "$title" =~ $ISSUE_TITLE_REGEX ]]; then
    echo "invalid_title|title|Title must match regex: ${ISSUE_TITLE_REGEX}"
  fi
}

issue_validate_body() {
  local body="${1:-}"
  local section
  local rule
  local field_name
  local field_regex
  local field_help
  local field_value

  issue_contract_load || return 1

  while IFS= read -r section; do
    section="$(trim_whitespace "$section")"
    [[ -z "$section" ]] && continue
    if ! grep -qF "$section" <<< "$body"; then
      echo "missing_section|${section}|Missing required section: ${section}"
    fi
  done <<< "${ISSUE_REQUIRED_SECTIONS:-}"

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
  done <<< "${ISSUE_REQUIRED_FIELDS:-}"
}

issue_validate_content() {
  local title="${1:-}"
  local body="${2:-}"
  issue_validate_title "$title"
  issue_validate_body "$body"
}
