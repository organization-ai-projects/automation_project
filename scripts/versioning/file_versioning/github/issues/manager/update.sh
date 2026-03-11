#!/usr/bin/env bash

manager_parse_update_args() {
  local issue_var_name="$1"
  local repo_var_name="$2"
  local update_title_var_name="$3"
  local update_body_var_name="$4"
  local edit_args_var_name="$5"
  shift 5

  local -n edit_args_ref="$edit_args_var_name"

  printf -v "$issue_var_name" '%s' ""
  printf -v "$repo_var_name" '%s' ""
  printf -v "$update_title_var_name" '%s' ""
  printf -v "$update_body_var_name" '%s' ""
  edit_args_ref=()

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --issue)
      issue_cli_assign_value_or_error "$1" "${2:-}" "$issue_var_name" die_usage
      shift 2
      ;;
    --repo)
      issue_cli_assign_value_or_error "$1" "${2:-}" "$repo_var_name" die_usage
      shift 2
      ;;
    --title | --body | --add-label | --remove-label | --add-assignee | --remove-assignee)
      local opt="$1"
      local opt_value="${2:-}"
      issue_cli_require_option_value "$opt" "$opt_value" die_usage
      case "$opt" in
      --title) printf -v "$update_title_var_name" '%s' "$opt_value" ;;
      --body) printf -v "$update_body_var_name" '%s' "$opt_value" ;;
      esac
      edit_args_ref+=("$opt" "$opt_value")
      shift 2
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *)
      die_usage "Unknown option for update: $1"
      ;;
    esac
  done
}

manager_load_issue_content_for_update() {
  local issue_number="$1"
  local repo="$2"
  issue_gh_issue_json "$repo" "$issue_number" "title,body,labels"
}

cmd_update() {
  local issue_number=""
  local repo=""
  local -a edit_args=()
  local update_title=""
  local update_body=""

  manager_parse_update_args issue_number repo update_title update_body edit_args "$@"

  issue_cli_require_positive_number "--issue" "$issue_number"
  if [[ ${#edit_args[@]} -eq 0 ]]; then
    die_usage "update requires at least one edit option."
  fi

  # Contract guard: when title/body changes, validate resulting issue content.
  if [[ -n "$update_title" || -n "$update_body" ]]; then
    local current_json=""
    local current_title=""
    local current_body=""
    local labels_raw=""
    local effective_title=""
    local effective_body=""
    local validations=""

    current_json="$(manager_load_issue_content_for_update "$issue_number" "$repo" || true)"
    if [[ -z "$current_json" ]]; then
      echo "Error: unable to read issue #${issue_number} before update validation." >&2
      exit 1
    fi

    current_title="$(echo "$current_json" | jq -r '.title // ""')"
    current_body="$(echo "$current_json" | jq -r '.body // ""')"
    labels_raw="$(echo "$current_json" | jq -r '.labels | map(.name) | join("||")')"

    effective_title="$current_title"
    effective_body="$current_body"
    [[ -n "$update_title" ]] && effective_title="$update_title"
    [[ -n "$update_body" ]] && effective_body="$update_body"

    validations="$(issue_validate_content "$effective_title" "$effective_body" "$labels_raw" || true)"
    if [[ -n "$validations" ]]; then
      echo "Error: issue update rejected due to contract violations." >&2
      echo "$validations" | while IFS='|' read -r code field reason; do
        [[ -z "${code:-}" ]] && continue
        echo " - ${code} (${field}): ${reason}" >&2
      done
      exit 1
    fi
  fi

  issue_gh_issue_update "$repo" "$issue_number" "${edit_args[@]}"
  echo "Issue #${issue_number} updated."
}
