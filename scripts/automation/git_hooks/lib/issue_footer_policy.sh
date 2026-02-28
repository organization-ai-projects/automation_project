#!/usr/bin/env bash

# Shared issue-footer policy helpers for commit messages.

: "${COMMIT_MSG_RC_SUBJECT_TRAILER:=1}"
: "${COMMIT_MSG_RC_ROOT_PARENT:=1}"
: "${COMMIT_MSG_RC_ASSIGNMENT_POLICY:=1}"

extract_trailer_issue_refs_from_file() {
  local commit_msg_file="$1"
  extract_issue_refs_from_text "$(sed '/^[[:space:]]*#/d' "$commit_msg_file")" || true
}

validate_and_normalize_issue_refs_footer_in_file() {
  local commit_msg_file="$1"
  local commit_subject="$2"
  local issue_ref_re='(^|[[:space:]])(closes|part[[:space:]]+of|reopen|reopens|fixes)[[:space:]]+#[0-9]+([[:space:]]|$)'
  local trailer_line_re='^[[:space:]]*(closes|part[[:space:]]+of|reopen|reopens)[[:space:]]+#[0-9]+[[:space:]]*$'
  local subject_lower
  subject_lower="$(printf '%s' "$commit_subject" | tr '[:upper:]' '[:lower:]')"

  if [[ "$subject_lower" =~ $issue_ref_re ]]; then
    echo "❌ Issue references must be in commit footer, not in subject." >&2
    echo "   Move 'Closes/Part of/Reopen #...' to footer lines." >&2
    return "$COMMIT_MSG_RC_SUBJECT_TRAILER"
  fi

  mapfile -t message_lines < <(sed '/^[[:space:]]*#/d' "$commit_msg_file")
  [[ ${#message_lines[@]} -eq 0 ]] && return 0

  local -a trailers=()
  local -a trailer_keys=()
  local -a content_lines=()
  local line normalized normalized_lower keyword issue_number canonical key

  content_lines+=("${message_lines[0]}")
  for line in "${message_lines[@]:1}"; do
    normalized="$(echo "$line" | sed -E 's/^[[:space:]]+//; s/[[:space:]]+$//')"
    normalized_lower="$(printf '%s' "$normalized" | tr '[:upper:]' '[:lower:]')"
    if [[ "$normalized_lower" =~ ^fixes[[:space:]]+#[0-9]+$ ]]; then
      echo "❌ Invalid issue footer keyword: 'Fixes' is not allowed." >&2
      echo "   Use 'Closes #<issue>' for closure." >&2
      return "$COMMIT_MSG_RC_SUBJECT_TRAILER"
    fi
    if [[ "$normalized_lower" =~ $trailer_line_re ]]; then
      if [[ "$normalized_lower" =~ ^(closes|part[[:space:]]+of|reopen|reopens)[[:space:]]+#([0-9]+)$ ]]; then
        keyword="$(printf '%s' "${BASH_REMATCH[1]}" | tr '[:upper:]' '[:lower:]')"
        issue_number="${BASH_REMATCH[2]}"

        case "$keyword" in
          closes) canonical="Closes #${issue_number}" ;;
          "part of") canonical="Part of #${issue_number}" ;;
          reopen|reopens) canonical="Reopen #${issue_number}" ;;
          *) canonical="$normalized" ;;
        esac

        key="${keyword}#${issue_number}"
        if [[ ! " ${trailer_keys[*]} " =~ " ${key} " ]]; then
          trailer_keys+=("$key")
          trailers+=("$canonical")
        fi
        continue
      fi
    fi
    content_lines+=("$line")
  done

  [[ ${#trailers[@]} -eq 0 ]] && return 0

  while [[ ${#content_lines[@]} -gt 0 ]]; do
    line="${content_lines[$((${#content_lines[@]} - 1))]}"
    [[ "$line" =~ ^[[:space:]]*$ ]] || break
    unset "content_lines[$((${#content_lines[@]} - 1))]"
  done

  local -a compact_lines=()
  local prev_blank=false
  for line in "${content_lines[@]}"; do
    if [[ "$line" =~ ^[[:space:]]*$ ]]; then
      if [[ "$prev_blank" == true ]]; then
        continue
      fi
      prev_blank=true
    else
      prev_blank=false
    fi
    compact_lines+=("$line")
  done

  {
    local i
    for i in "${!compact_lines[@]}"; do
      echo "${compact_lines[$i]}"
    done
    echo
    for line in "${trailers[@]}"; do
      echo "$line"
    done
  } > "$commit_msg_file"

  return 0
}

validate_no_root_parent_refs_in_footer_file() {
  local commit_msg_file="$1"
  local refs
  local action
  local issue_number
  local repo
  local root_parent_refs=()

  if ! command -v gh >/dev/null 2>&1; then
    return 0
  fi

  refs="$(extract_trailer_issue_refs_from_file "$commit_msg_file")"
  [[ -z "$refs" ]] && return 0

  repo="$(resolve_repo_name_with_owner)"
  [[ -z "$repo" ]] && return 0

  while IFS='|' read -r action issue_number; do
    [[ -z "$issue_number" ]] && continue
    if issue_is_root_parent "$issue_number" "$repo"; then
      root_parent_refs+=("${action} #${issue_number}")
    fi
  done <<< "$refs"

  if [[ ${#root_parent_refs[@]} -gt 0 ]]; then
    echo "❌ Invalid issue footer usage in commit message." >&2
    echo "   Root parent issue references are not allowed in commit trailers: ${root_parent_refs[*]}" >&2
    echo "   Use issue refs on child issues only (Part of/Closes/Reopen #<child-issue>)." >&2
    echo "   Parent closure should be handled by child completion workflow, not direct commit trailers." >&2
    echo "   Bypass (emergency only): SKIP_COMMIT_VALIDATION=1 git commit ..." >&2
    return "$COMMIT_MSG_RC_ROOT_PARENT"
  fi

  return 0
}

validate_single_assignee_requires_closes_in_footer_file() {
  local commit_msg_file="$1"
  local refs
  local repo
  local current_login
  local action
  local issue_number
  local -a violations=()

  refs="$(extract_trailer_issue_refs_from_file "$commit_msg_file")"
  [[ -z "$refs" ]] && return 0

  if ! command -v gh >/dev/null 2>&1; then
    return 0
  fi

  repo="$(resolve_repo_name_with_owner)"
  [[ -z "$repo" ]] && return 0

  current_login="$(gh api user --jq '.login' 2>/dev/null || true)"
  [[ -z "$current_login" ]] && return 0

  declare -A has_part_of=()
  declare -A has_closing=()
  declare -A assignee_count=()
  declare -A sole_assignee=()

  while IFS='|' read -r action issue_number; do
    [[ -z "$action" || -z "$issue_number" ]] && continue
    if [[ "$action" == "part of" ]]; then
      has_part_of["$issue_number"]=1
    fi
    if [[ "$action" == "closes" || "$action" == "fixes" ]]; then
      has_closing["$issue_number"]=1
    fi
  done <<< "$refs"

  for issue_number in "${!has_part_of[@]}"; do
    [[ -n "${has_closing[$issue_number]:-}" ]] && continue

    local assignees
    local count
    assignees="$(gh issue view "$issue_number" -R "$repo" --json assignees --jq '.assignees[].login' 2>/dev/null || true)"
    count="$(printf '%s\n' "$assignees" | sed '/^$/d' | wc -l | tr -d '[:space:]')"
    assignee_count["$issue_number"]="${count:-0}"
    sole_assignee["$issue_number"]="$(printf '%s\n' "$assignees" | sed '/^$/d' | head -n1)"

    if [[ "${assignee_count[$issue_number]}" == "1" && "${sole_assignee[$issue_number]}" == "$current_login" ]]; then
      violations+=("#${issue_number} is assigned only to @${current_login}: 'Closes #${issue_number}' is required (Part of only is not allowed)")
    fi
  done

  if [[ ${#violations[@]} -gt 0 ]]; then
    echo "❌ Assignment policy violation in commit footer." >&2
    printf '   - %s\n' "${violations[@]}" >&2
    return "$COMMIT_MSG_RC_ASSIGNMENT_POLICY"
  fi

  return 0
}
