#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# GitHub-backed extraction helpers.

pr_extract_child_prs() {
  local commit_headlines
  local main_pr_body
  local main_pr_comments
  local repo_owner_name
  local timeline_pr_refs
  local timeline_pr_refs_raw
  local va_child_refs

  repo_owner_name="$(pr_gh_optional "resolve repository name" repo view --json nameWithOwner -q '.nameWithOwner')"

  if command -v va_exec >/dev/null 2>&1; then
    if [[ -n "$repo_owner_name" ]]; then
      va_child_refs="$(
        va_exec pr child-pr-refs \
          --pr "$main_pr_number" \
          --repo "$repo_owner_name" 2>/dev/null || true
      )"
    else
      va_child_refs="$(
        va_exec pr child-pr-refs \
          --pr "$main_pr_number" 2>/dev/null || true
      )"
    fi
  fi

  if [[ -n "$va_child_refs" ]]; then
    printf '%s\n' "$va_child_refs" | grep -E '^#[0-9]+$' | sort -u | grep -v "^#${main_pr_number}$" >"$extracted_prs_file"
    pr_debug_log "extract_child_prs(main=#${main_pr_number}) => $(tr '\n' ' ' <"$extracted_prs_file")"
    return 0
  fi

  # gh pr view --json commits can be truncated; use paginated API commit headlines.
  commit_headlines=""
  if [[ -n "$repo_owner_name" ]]; then
    commit_headlines="$(pr_gh_optional "fetch commits for PR #${main_pr_number}" api "repos/${repo_owner_name}/pulls/${main_pr_number}/commits" --paginate \
      --jq '.[].commit.message | split("\\n")[0]')"
  fi

  main_pr_body="$(pr_get_pr_body "$main_pr_number" "read PR #${main_pr_number} body")"
  main_pr_comments="$(pr_gh_optional "read PR #${main_pr_number} comments" pr view "$main_pr_number" --json comments -q '.comments[].body')"
  timeline_pr_refs=""
  if [[ -n "$repo_owner_name" ]]; then
    timeline_pr_refs_raw="$(pr_gh_optional "read cross-reference timeline for #${main_pr_number}" api "repos/${repo_owner_name}/issues/${main_pr_number}/timeline" --paginate \
      --jq '.[] | select(.event=="cross-referenced") | select(.source.issue.pull_request.url != null) | .source.issue.number')"
    if [[ -n "$timeline_pr_refs_raw" ]]; then
      timeline_pr_refs="$(printf "%s\n" "$timeline_pr_refs_raw" | sed -nE 's/^([0-9]+)$/#\1/p')"
    fi
  fi

  if [[ -z "$commit_headlines" && -z "$main_pr_body" && -z "$main_pr_comments" && -z "$timeline_pr_refs" ]]; then
    return 1
  fi

  {
    pr_extract_pr_refs_from_headlines "$commit_headlines"
    pr_extract_pr_refs_from_text "$main_pr_body"
    pr_extract_pr_refs_from_text "$main_pr_comments"
    echo "$timeline_pr_refs"
  } | grep -E '^#[0-9]+$' | sort -u | grep -v "^#${main_pr_number}$" >"$extracted_prs_file"

  pr_debug_log "extract_child_prs(main=#${main_pr_number}) => $(tr '\n' ' ' <"$extracted_prs_file")"
  return 0
}
