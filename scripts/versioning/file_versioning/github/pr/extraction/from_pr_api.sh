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

  repo_owner_name="$(pr_gh_optional "resolve repository name" repo view --json nameWithOwner -q '.nameWithOwner')"

  # gh pr view --json commits can be truncated; use paginated API commit headlines.
  commit_headlines=""
  if [[ -n "$repo_owner_name" ]]; then
    commit_headlines="$(pr_gh_optional "fetch commits for PR #${main_pr_number}" api "repos/${repo_owner_name}/pulls/${main_pr_number}/commits" --paginate \
      --jq '.[].commit.message | split("\\n")[0]')"
  fi

  main_pr_body="$(pr_gh_optional "read PR #${main_pr_number} body" pr view "$main_pr_number" --json body -q '.body')"
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
    echo "$commit_headlines" | sed -nE 's/.*Merge pull request #([0-9]+).*/#\1/p'
    echo "$commit_headlines" | sed -nE 's/.*\(#([0-9]+)\)\s*$/#\1/p'
    echo "$main_pr_body" | grep -oE '/pull/[0-9]+' | sed -E 's#^/pull/([0-9]+)$#\#\1#'
    echo "$main_pr_body" | sed -nE 's/.*\bPR[[:space:]]*#([0-9]+).*/#\1/ip'
    echo "$main_pr_body" | sed -nE 's/.*pull request #([0-9]+).*/#\1/ip'
    echo "$main_pr_comments" | grep -oE '/pull/[0-9]+' | sed -E 's#^/pull/([0-9]+)$#\#\1#'
    echo "$main_pr_comments" | sed -nE 's/.*\bPR[[:space:]]*#([0-9]+).*/#\1/ip'
    echo "$main_pr_comments" | sed -nE 's/.*pull request #([0-9]+).*/#\1/ip'
    echo "$timeline_pr_refs"
  } | grep -E '^#[0-9]+$' | sort -u | grep -v "^#${main_pr_number}$" >"$extracted_prs_file"

  pr_debug_log "extract_child_prs(main=#${main_pr_number}) => $(tr '\n' ' ' <"$extracted_prs_file")"
  return 0
}
