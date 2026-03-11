#!/usr/bin/env bash
# shellcheck shell=bash

auto_add_collect_refs_from_payload() {
  local payload="$1"
  local _out_part_of_refs_var="$2"
  local _out_closing_refs_var="$3"
  local -n _out_part_of_refs_ref="$_out_part_of_refs_var"
  local -n _out_closing_refs_ref="$_out_closing_refs_var"

  _out_part_of_refs_ref="$(parse_non_closing_issue_refs_from_text "$payload")"
  _out_closing_refs_ref="$(parse_closing_issue_refs_from_text "$payload")"
}

auto_add_should_close_issue_for_author() {
  local issue_number="$1"
  local repo_name="$2"
  local pr_author="$3"
  local assignees assignee_count sole_assignee

  assignees="$(github_issue_assignee_logins "$repo_name" "$issue_number" || true)"
  assignee_count="$(printf '%s\n' "$assignees" | sed '/^$/d' | wc -l | tr -d '[:space:]')"
  sole_assignee="$(printf '%s\n' "$assignees" | sed '/^$/d' | head -n1)"

  [[ "${assignee_count:-0}" == "1" && "$sole_assignee" == "$pr_author" ]]
}

auto_add_resolve_repo() {
  local current_repo_name="${1:-}"
  if [[ -z "$current_repo_name" ]]; then
    current_repo_name="$(gh_cli_resolve_repo_name)"
  fi
  if [[ -z "$current_repo_name" ]]; then
    echo "Error: unable to determine repository." >&2
    exit 3
  fi
  printf '%s' "$current_repo_name"
}

auto_add_fetch_pr_details_json() {
  local repo_name="$1"
  local pr_number="$2"
  local pr_details_json=""

  if command -v va_exec >/dev/null 2>&1; then
    local pr_state pr_base pr_title pr_body pr_author pr_commits
    pr_state="$(
      va_exec pr field \
        --pr "$pr_number" \
        --repo "$repo_name" \
        --name "state" 2>/dev/null || true
    )"
    pr_base="$(
      va_exec pr field \
        --pr "$pr_number" \
        --repo "$repo_name" \
        --name "base-ref-name" 2>/dev/null || true
    )"
    pr_title="$(
      va_exec pr field \
        --pr "$pr_number" \
        --repo "$repo_name" \
        --name "title" 2>/dev/null || true
    )"
    pr_body="$(
      va_exec pr field \
        --pr "$pr_number" \
        --repo "$repo_name" \
        --name "body" 2>/dev/null || true
    )"
    pr_author="$(
      va_exec pr field \
        --pr "$pr_number" \
        --repo "$repo_name" \
        --name "author-login" 2>/dev/null || true
    )"
    pr_commits="$(
      va_exec pr field \
        --pr "$pr_number" \
        --repo "$repo_name" \
        --name "commit-messages" 2>/dev/null || true
    )"

    if [[ -n "$pr_state" || -n "$pr_base" || -n "$pr_title" || -n "$pr_body" || -n "$pr_author" || -n "$pr_commits" ]]; then
      pr_details_json="$(
        jq -c -n \
          --argjson number "$pr_number" \
          --arg state "$pr_state" \
          --arg base_ref_name "$pr_base" \
          --arg title "$pr_title" \
          --arg body "$pr_body" \
          --arg author_login "$pr_author" \
          --arg commit_messages "$pr_commits" \
          '{number: $number, state: $state, base_ref_name: $base_ref_name, title: $title, body: $body, author_login: $author_login, commit_messages: $commit_messages}' \
          2>/dev/null || true
      )"
    fi
  fi

  if [[ -z "$pr_details_json" ]]; then
    local pr_json pr_commits
    pr_json="$(gh pr view "$pr_number" -R "$repo_name" --json number,state,baseRefName,title,body,author 2>/dev/null || true)"
    if [[ -n "$pr_json" ]]; then
      pr_commits="$(gh api "repos/${repo_name}/pulls/${pr_number}/commits" --paginate --jq '.[].commit.message' 2>/dev/null || true)"
      pr_details_json="$(
        jq -c --arg commit_messages "$pr_commits" \
          '. + { commit_messages: $commit_messages }' <<<"$pr_json" 2>/dev/null || true
      )"
    fi
  fi

  printf '%s' "$pr_details_json"
}

auto_add_pr_field() {
  local repo_name="$1"
  local pr_number="$2"
  local field_name="$3"
  local pr_json_fallback="${4:-}"
  local va_output=""
  local jq_filter=""

  if command -v va_exec >/dev/null 2>&1; then
    va_output="$(
      va_exec pr field \
        --pr "$pr_number" \
        --repo "$repo_name" \
        --name "$field_name" 2>/dev/null || true
    )"
    if [[ -n "$va_output" ]]; then
      printf '%s' "$va_output"
      return 0
    fi
  fi

  case "$field_name" in
  state) jq_filter='.state // ""' ;;
  base-ref-name) jq_filter='.baseRefName // .base_ref_name // ""' ;;
  title) jq_filter='.title // ""' ;;
  body) jq_filter='.body // ""' ;;
  author-login) jq_filter='.author.login // .author_login // ""' ;;
  commit-messages) jq_filter='.commit_messages // ""' ;;
  *)
    return 1
    ;;
  esac

  if [[ -z "$pr_json_fallback" ]]; then
    printf ''
    return 0
  fi
  echo "$pr_json_fallback" | jq -r "$jq_filter"
}

auto_add_build_managed_block() {
  local -a issue_numbers=("$@")

  echo "<!-- auto-closes:start -->"
  echo "### Auto-managed Issue Closures"
  local n
  for n in "${issue_numbers[@]}"; do
    echo "Closes #${n}"
  done
  echo "<!-- auto-closes:end -->"
}

auto_add_closes_run() {
  local auto_add_pr_number=""
  local auto_add_repo_name="${GH_REPO:-}"
  local pr_json pr_state pr_base pr_title pr_body pr_author
  local pr_commits payload_all part_of_refs closing_refs
  local issue_number
  local managed_block body_without_block new_body
  local -a sorted_issue_numbers
  local -A already_closing=()
  local -A closes_to_add=()

  auto_add_parse_cli auto_add_pr_number auto_add_repo_name "$@"
  gh_cli_require_gh_jq
  auto_add_repo_name="$(auto_add_resolve_repo "$auto_add_repo_name")"

  pr_json="$(auto_add_fetch_pr_details_json "$auto_add_repo_name" "$auto_add_pr_number")"
  if [[ -z "$pr_json" ]]; then
    echo "Error: unable to read PR #${auto_add_pr_number}." >&2
    exit 3
  fi

  pr_state="$(auto_add_pr_field "$auto_add_repo_name" "$auto_add_pr_number" "state" "$pr_json")"
  pr_base="$(auto_add_pr_field "$auto_add_repo_name" "$auto_add_pr_number" "base-ref-name" "$pr_json")"
  pr_title="$(auto_add_pr_field "$auto_add_repo_name" "$auto_add_pr_number" "title" "$pr_json")"
  pr_body="$(auto_add_pr_field "$auto_add_repo_name" "$auto_add_pr_number" "body" "$pr_json")"
  pr_author="$(auto_add_pr_field "$auto_add_repo_name" "$auto_add_pr_number" "author-login" "$pr_json")"

  if [[ "$pr_state" != "OPEN" ]]; then
    echo "PR #${auto_add_pr_number} is not open; skipping."
    exit 0
  fi
  if [[ "$pr_base" != "dev" ]]; then
    echo "PR #${auto_add_pr_number} does not target dev; skipping."
    exit 0
  fi
  if [[ -z "$pr_author" ]]; then
    echo "PR #${auto_add_pr_number}: author login unavailable; skipping."
    exit 0
  fi

  pr_commits="$(auto_add_pr_field "$auto_add_repo_name" "$auto_add_pr_number" "commit-messages" "$pr_json")"
  payload_all="$({
    printf '%s\n' "$pr_title"
    printf '%s\n' "$pr_body"
    printf '%s\n' "$pr_commits"
  })"

  auto_add_collect_refs_from_payload "$payload_all" part_of_refs closing_refs

  if [[ -z "$part_of_refs" ]]; then
    echo "PR #${auto_add_pr_number}: no Part of refs detected; nothing to enrich."
    exit 0
  fi

  while IFS= read -r issue_number; do
    [[ -n "$issue_number" ]] && already_closing["$issue_number"]=1
  done < <(auto_add_extract_issue_numbers "$closing_refs")

  while IFS= read -r issue_number; do
    [[ -z "$issue_number" ]] && continue
    if [[ -n "${already_closing[$issue_number]:-}" ]]; then
      continue
    fi

    if auto_add_should_close_issue_for_author "$issue_number" "$auto_add_repo_name" "$pr_author"; then
      closes_to_add["$issue_number"]=1
    fi
  done < <(auto_add_extract_issue_numbers "$part_of_refs")

  if [[ ${#closes_to_add[@]} -eq 0 ]]; then
    echo "PR #${auto_add_pr_number}: no qualifying single-assignee issue found; nothing to enrich."
    exit 0
  fi

  mapfile -t sorted_issue_numbers < <(printf '%s\n' "${!closes_to_add[@]}" | sort -n)
  managed_block="$(auto_add_build_managed_block "${sorted_issue_numbers[@]}")"

  body_without_block="$(auto_add_strip_managed_block "$pr_body")"
  body_without_block="$(printf '%s\n' "$body_without_block" | sed ':a;N;$!ba;s/\n\{3,\}/\n\n/g')"

  if [[ -n "$body_without_block" ]]; then
    new_body="$body_without_block"$'\n\n'"$managed_block"
  else
    new_body="$managed_block"
  fi

  if [[ "$new_body" == "$pr_body" ]]; then
    echo "PR #${auto_add_pr_number}: body already up-to-date."
    exit 0
  fi

  gh pr edit "$auto_add_pr_number" -R "$auto_add_repo_name" --body "$new_body" >/dev/null
  echo "PR #${auto_add_pr_number}: updated body with auto-managed Closes refs."
}
