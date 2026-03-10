#!/usr/bin/env bash
# shellcheck shell=bash

auto_add_collect_refs_from_payload() {
  local payload="$1"
  local _out_part_of_refs_var="$2"
  local _out_closing_refs_var="$3"
  local -n _out_part_of_refs_ref="$_out_part_of_refs_var"
  local -n _out_closing_refs_ref="$_out_closing_refs_var"
  local record_type action issue_key
  local -a part_of_rows=()
  local -a closing_rows=()

  while IFS='|' read -r record_type action issue_key; do
    [[ "$record_type" == "EV" ]] || continue
    [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
    if [[ "$action" == "Part of" ]]; then
      part_of_rows+=("Part of|${issue_key}")
    elif [[ "$action" == "Closes" ]]; then
      closing_rows+=("Closes|${issue_key}")
    fi
  done < <(parse_issue_directive_records_from_text "$payload")

  _out_part_of_refs_ref="$(printf '%s\n' "${part_of_rows[@]}" | sed '/^$/d' | sort -u)"
  _out_closing_refs_ref="$(printf '%s\n' "${closing_rows[@]}" | sed '/^$/d' | sort -u)"
}

auto_add_should_close_issue_for_author() {
  local issue_number="$1"
  local repo_name="$2"
  local pr_author="$3"
  local assignees assignee_count sole_assignee

  assignees="$(gh issue view "$issue_number" -R "$repo_name" --json assignees --jq '.assignees[].login' 2>/dev/null || true)"
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

  pr_json="$(gh pr view "$auto_add_pr_number" -R "$auto_add_repo_name" --json number,state,baseRefName,title,body,author 2>/dev/null || true)"
  if [[ -z "$pr_json" ]]; then
    echo "Error: unable to read PR #${auto_add_pr_number}." >&2
    exit 3
  fi

  pr_state="$(echo "$pr_json" | jq -r '.state // ""')"
  pr_base="$(echo "$pr_json" | jq -r '.baseRefName // ""')"
  pr_title="$(echo "$pr_json" | jq -r '.title // ""')"
  pr_body="$(echo "$pr_json" | jq -r '.body // ""')"
  pr_author="$(echo "$pr_json" | jq -r '.author.login // ""')"

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

  pr_commits="$(gh api "repos/${auto_add_repo_name}/pulls/${auto_add_pr_number}/commits" --paginate --jq '.[].commit.message' 2>/dev/null || true)"
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
