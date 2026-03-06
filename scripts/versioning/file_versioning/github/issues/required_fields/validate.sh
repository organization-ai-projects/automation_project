#!/usr/bin/env bash
# shellcheck shell=bash

# Issue title/body validation helpers.

issue_validate_title() {
  local title="${1:-}"
  local labels_raw="${2:-}"
  local profile

  issue_contract_load || return 1
  profile="$(issue_contract_profile_for_labels "$labels_raw")"
  issue_validate_title_with_profile "$title" "$profile"
}

issue_validate_title_with_profile() {
  local title="${1:-}"
  local profile="${2:-default}"
  local regex_key
  local regex

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

  issue_contract_load || return 1
  profile="$(issue_contract_profile_for_labels "$labels_raw")"
  issue_validate_body_with_profile "$body" "$profile"
}

issue_validate_body_with_profile() {
  local body="${1:-}"
  local profile="${2:-default}"
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

  sections_key="$(issue_contract_key_for_profile "$profile" "REQUIRED_SECTIONS")"
  fields_key="$(issue_contract_key_for_profile "$profile" "REQUIRED_FIELDS")"
  required_sections="$(issue_contract_get "$sections_key")"
  required_fields="$(issue_contract_get "$fields_key")"

  while IFS= read -r section; do
    section="$(trim_whitespace "$section")"
    [[ -z "$section" ]] && continue
    if ! issue_body_has_section "$body" "$section"; then
      echo "missing_section|${section}|Missing required section: ${section}"
    fi
  done <<<"${required_sections:-}"

  while IFS= read -r rule; do
    [[ -z "$rule" ]] && continue
    IFS=$'\t' read -r field_name field_regex field_help <<<"$rule"
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
  done <<<"${required_fields:-}"
}

issue_validate_content() {
  local title="${1:-}"
  local body="${2:-}"
  local labels_raw="${3:-}"
  local profile

  issue_contract_load || return 1
  profile="$(issue_contract_profile_for_labels "$labels_raw")"
  issue_validate_title_with_profile "$title" "$profile"
  issue_validate_body_with_profile "$body" "$profile"
}
