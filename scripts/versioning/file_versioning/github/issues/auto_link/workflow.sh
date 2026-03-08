#!/usr/bin/env bash

auto_link_run() {
  local issue_arg=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --issue)
      issue_cli_assign_value_or_usage "$1" "${2:-}" issue_arg auto_link_usage || exit 2
      shift 2
      ;;
    -h | --help)
      auto_link_usage
      exit 0
      ;;
    *)
      issue_cli_unknown_option_with_usage "$1" auto_link_usage
      exit 2
      ;;
    esac
  done

  if [[ -z "$issue_arg" ]]; then
    echo "Erreur: --issue est requis." >&2
    auto_link_usage >&2
    exit 2
  fi

  issue_cli_require_positive_number "--issue" "$issue_arg"
  auto_link_require_deps

  local repo_name
  repo_name="$(issue_gh_resolve_repo_name_or_exit "${GH_REPO:-}" "repository")"

  local repo_owner="${repo_name%%/*}"
  local repo_short_name="${repo_name#*/}"
  local issue_number="$issue_arg"

  local marker="<!-- parent-field-autolink:${issue_number} -->"
  local label_required_missing="issue-required-missing"
  local label_automation_failed="automation-failed"

  local issue_json
  issue_json="$(gh issue view "$issue_number" -R "$repo_name" --json number,title,body,state,url,labels 2>/dev/null || true)"
  if [[ -z "$issue_json" ]]; then
    echo "Erreur: impossible de lire l'issue #${issue_number}." >&2
    exit 4
  fi

  local issue_title issue_body issue_labels_raw
  issue_title="$(echo "$issue_json" | jq -r '.title // ""')"
  issue_body="$(echo "$issue_json" | jq -r '.body // ""')"
  issue_labels_raw="$(echo "$issue_json" | jq -r '(.labels // []) | map(.name) | join("||")')"

  auto_link_validate_contract_or_exit \
    "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
    "$issue_title" "$issue_body" "$issue_labels_raw"

  local parent_raw parent_raw_lc
  parent_raw="$(auto_link_extract_parent_or_exit \
    "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
    "$issue_body")"

  parent_raw_lc="$(echo "$parent_raw" | tr '[:upper:]' '[:lower:]')"
  if [[ "$parent_raw_lc" == "none" || "$parent_raw_lc" == "base" || "$parent_raw_lc" == "epic" ]]; then
    auto_link_handle_parent_none \
      "$repo_name" "$repo_owner" "$repo_short_name" "$issue_number" "$parent_raw_lc" "$marker" \
      "$label_required_missing" "$label_automation_failed"
  fi

  if [[ ! "$parent_raw" =~ ^#[0-9]+$ ]]; then
    auto_link_set_validation_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Invalid \`Parent:\` value: \`${parent_raw}\`." \
      "Expected \`Parent: #<issue_number>\` or one of \`Parent: none|base|epic\`."
    exit 0
  fi

  auto_link_handle_parent_link \
    "$repo_name" "$repo_owner" "$repo_short_name" "$issue_number" "${parent_raw//#/}" "$marker" \
    "$label_required_missing" "$label_automation_failed"
}
