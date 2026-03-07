#!/usr/bin/env bash

reevaluate_usage() {
  cat <<USAGE
Usage:
  issues/reevaluate/run.sh --issue ISSUE_NUMBER [--repo owner/name]

Notes:
  - Finds all open PRs referencing the given issue number via closing keywords
    (Closes/Fixes #N).
  - Re-evaluates closure neutralization for each such PR.
  - Useful when an issue is edited and may now satisfy (or violate) compliance.
USAGE
}

reevaluate_pr_body_references_issue() {
  local issue_number="$1"
  local body="$2"
  local target_issue_key="#${issue_number}"
  local issue_key

  while IFS='|' read -r _ issue_key; do
    [[ "$issue_key" == "$target_issue_key" ]] && return 0
  done < <(parse_all_closing_issue_refs_from_text "$body")

  return 1
}

reevaluate_main() {
  local issue_number=""
  local repo_name="${GH_REPO:-}"

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --issue)
      issue_cli_assign_value_or_usage "$1" "${2:-}" issue_number reevaluate_usage || exit 2
      shift 2
      ;;
    --repo)
      issue_cli_assign_value_or_usage "$1" "${2:-}" repo_name reevaluate_usage || exit 2
      shift 2
      ;;
    -h | --help)
      reevaluate_usage
      exit 0
      ;;
    *)
      issue_cli_unknown_option_with_usage "$1" reevaluate_usage
      exit 2
      ;;
    esac
  done

  [[ -n "$issue_number" ]] || {
    echo "Error: --issue is required." >&2
    reevaluate_usage >&2
    exit 2
  }
  issue_cli_require_positive_number "--issue" "$issue_number"

  issue_gh_require_gh_and_jq

  repo_name="$(issue_gh_resolve_repo_name_or_exit "$repo_name" "repository")"

  local neutralizer="${ISSUES_DIR}/neutralize/run.sh"
  if [[ ! -x "$neutralizer" ]]; then
    chmod +x "$neutralizer"
  fi

  local pr_numbers
  pr_numbers="$({
    gh api "repos/${repo_name}/pulls?state=open&per_page=100" --paginate --jq '.[]. | [.number, (.body // "")] | @tsv' 2>/dev/null |
      while IFS=$'\t' read -r pr_num pr_body; do
        [[ -n "$pr_num" ]] || continue
        if reevaluate_pr_body_references_issue "$issue_number" "$pr_body"; then
          printf '%s\n' "$pr_num"
        fi
      done
  } || true)"

  if [[ -z "$pr_numbers" ]]; then
    echo "No open PRs found referencing issue #${issue_number}."
    exit 0
  fi

  local evaluated_count=0
  local pr_num
  while IFS= read -r pr_num; do
    [[ -n "$pr_num" ]] || continue
    echo "Re-evaluating PR #${pr_num} (references issue #${issue_number})..."
    bash "$neutralizer" --pr "$pr_num" --repo "$repo_name"
    evaluated_count=$((evaluated_count + 1))
  done <<<"$pr_numbers"

  echo "Re-evaluation complete. ${evaluated_count} PR(s) evaluated."
}
