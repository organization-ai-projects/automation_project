#!/usr/bin/env bash

cmd_update() {
  local issue_number=""
  local repo=""
  local -a edit_args=()
  local update_title=""
  local update_body=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --issue)
      issue_number="${2:-}"
      shift 2
      ;;
    --repo)
      repo="${2:-}"
      shift 2
      ;;
    --title | --body | --add-label | --remove-label | --add-assignee | --remove-assignee)
      [[ -n "${2:-}" ]] || die_usage "$1 requires a value."
      if [[ "$1" == "--title" ]]; then
        update_title="${2:-}"
      elif [[ "$1" == "--body" ]]; then
        update_body="${2:-}"
      fi
      edit_args+=("$1" "${2:-}")
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

  ensure_number "--issue" "$issue_number"
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

    if [[ -n "$repo" ]]; then
      current_json="$(gh issue view "$issue_number" -R "$repo" --json title,body,labels)"
    else
      current_json="$(gh issue view "$issue_number" --json title,body,labels)"
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

  local -a cmd=(gh issue edit "$issue_number")
  if [[ -n "$repo" ]]; then
    cmd+=(-R "$repo")
  fi
  cmd+=("${edit_args[@]}")
  "${cmd[@]}" >/dev/null
  echo "Issue #${issue_number} updated."
}
