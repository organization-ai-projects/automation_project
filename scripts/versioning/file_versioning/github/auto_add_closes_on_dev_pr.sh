#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/issue_refs.sh
source "${SCRIPT_DIR}/lib/issue_refs.sh"

usage() {
  cat <<EOF
Usage:
  $0 --pr PR_NUMBER [--repo owner/name]

Description:
  - Targets open PRs into dev.
  - Detects "Part of #N" refs from PR body + commits.
  - If issue #N has exactly one assignee and that assignee is the PR author,
    ensures the PR body contains "Closes #N" in a managed block.
EOF
}

require_number() {
  local name="$1"
  local value="${2:-}"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Error: ${name} must be a positive integer." >&2
    exit 2
  fi
}

require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: command '${cmd}' is required." >&2
    exit 3
  fi
}

extract_issue_numbers() {
  local refs="$1"
  printf '%s\n' "$refs" \
    | cut -d'|' -f2 \
    | sed -nE 's/^#([0-9]+)$/\1/p' \
    | sort -u
}

strip_managed_block() {
  local body="$1"
  awk '
    BEGIN { in_block = 0 }
    /^<!-- auto-closes:start -->$/ { in_block = 1; next }
    /^<!-- auto-closes:end -->$/ { in_block = 0; next }
    { if (!in_block) print }
  ' <<< "$body"
}

pr_number=""
repo_name="${GH_REPO:-}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --pr)
      pr_number="${2:-}"
      shift 2
      ;;
    --repo)
      repo_name="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Error: unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

[[ -n "$pr_number" ]] || { echo "Error: --pr is required." >&2; usage >&2; exit 2; }
require_number "--pr" "$pr_number"
require_cmd gh
require_cmd jq

if [[ -z "$repo_name" ]]; then
  repo_name="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"
fi
[[ -n "$repo_name" ]] || { echo "Error: unable to determine repository." >&2; exit 3; }

pr_json="$(gh pr view "$pr_number" -R "$repo_name" --json number,state,baseRefName,title,body,author 2>/dev/null || true)"
[[ -n "$pr_json" ]] || { echo "Error: unable to read PR #${pr_number}." >&2; exit 3; }

pr_state="$(echo "$pr_json" | jq -r '.state // ""')"
pr_base="$(echo "$pr_json" | jq -r '.baseRefName // ""')"
pr_title="$(echo "$pr_json" | jq -r '.title // ""')"
pr_body="$(echo "$pr_json" | jq -r '.body // ""')"
pr_author="$(echo "$pr_json" | jq -r '.author.login // ""')"

if [[ "$pr_state" != "OPEN" ]]; then
  echo "PR #${pr_number} is not open; skipping."
  exit 0
fi
if [[ "$pr_base" != "dev" ]]; then
  echo "PR #${pr_number} does not target dev; skipping."
  exit 0
fi
if [[ -z "$pr_author" ]]; then
  echo "PR #${pr_number}: author login unavailable; skipping."
  exit 0
fi

pr_commits="$(gh api "repos/${repo_name}/pulls/${pr_number}/commits" --paginate --jq '.[].commit.message' 2>/dev/null || true)"

payload_all="$({
  printf '%s\n' "$pr_title"
  printf '%s\n' "$pr_body"
  printf '%s\n' "$pr_commits"
})"

part_of_refs="$(parse_non_closing_issue_refs_from_text "$payload_all")"
if [[ -z "$part_of_refs" ]]; then
  echo "PR #${pr_number}: no Part of refs detected; nothing to enrich."
  exit 0
fi

closing_refs="$(parse_closing_issue_refs_from_text "$payload_all")"
declare -A already_closing=()
while IFS= read -r issue_number; do
  [[ -n "$issue_number" ]] && already_closing["$issue_number"]=1
done < <(extract_issue_numbers "$closing_refs")

declare -A closes_to_add=()
while IFS= read -r issue_number; do
  [[ -z "$issue_number" ]] && continue
  if [[ -n "${already_closing[$issue_number]:-}" ]]; then
    continue
  fi

  assignees="$(gh issue view "$issue_number" -R "$repo_name" --json assignees --jq '.assignees[].login' 2>/dev/null || true)"
  assignee_count="$(printf '%s\n' "$assignees" | sed '/^$/d' | wc -l | tr -d '[:space:]')"
  sole_assignee="$(printf '%s\n' "$assignees" | sed '/^$/d' | head -n1)"

  if [[ "${assignee_count:-0}" == "1" && "$sole_assignee" == "$pr_author" ]]; then
    closes_to_add["$issue_number"]=1
  fi
done < <(extract_issue_numbers "$part_of_refs")

if [[ ${#closes_to_add[@]} -eq 0 ]]; then
  echo "PR #${pr_number}: no qualifying single-assignee issue found; nothing to enrich."
  exit 0
fi

mapfile -t sorted_issue_numbers < <(printf '%s\n' "${!closes_to_add[@]}" | sort -n)

managed_block="$({
  echo "<!-- auto-closes:start -->"
  echo "### Auto-managed Issue Closures"
  for n in "${sorted_issue_numbers[@]}"; do
    echo "Closes #${n}"
  done
  echo "<!-- auto-closes:end -->"
})"

body_without_block="$(strip_managed_block "$pr_body")"
body_without_block="$(printf '%s\n' "$body_without_block" | sed ':a;N;$!ba;s/\n\{3,\}/\n\n/g')"

if [[ -n "$body_without_block" ]]; then
  new_body="$body_without_block"$'\n\n'"$managed_block"
else
  new_body="$managed_block"
fi

if [[ "$new_body" == "$pr_body" ]]; then
  echo "PR #${pr_number}: body already up-to-date."
  exit 0
fi

gh pr edit "$pr_number" -R "$repo_name" --body "$new_body" >/dev/null
echo "PR #${pr_number}: updated body with auto-managed Closes refs."
