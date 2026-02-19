#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/issue_refs.sh
source "${SCRIPT_DIR}/lib/issue_refs.sh"

usage() {
  cat <<EOF
Usage:
  $0 --on-dev-merge --pr PR_NUMBER [--label LABEL]
  $0 --on-issue-closed --issue ISSUE_NUMBER [--label LABEL]
EOF
}

require_number() {
  local name="$1"
  local value="$2"
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

resolve_repo_name() {
  if [[ -n "${GH_REPO:-}" ]]; then
    echo "$GH_REPO"
    return 0
  fi
  gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true
}

label_exists() {
  local repo="$1"
  local label="$2"
  gh label list -R "$repo" --limit 1000 --json name --jq '.[].name' 2>/dev/null \
    | grep -Fxq "$label"
}

issue_state() {
  local repo="$1"
  local issue_number="$2"
  gh issue view "$issue_number" -R "$repo" --json state -q '.state // ""' 2>/dev/null || true
}

issue_has_label() {
  local repo="$1"
  local issue_number="$2"
  local label="$3"
  gh issue view "$issue_number" -R "$repo" --json labels --jq '.labels[].name' 2>/dev/null \
    | grep -Fxq "$label"
}

extract_closing_issue_numbers() {
  local text="$1"
  parse_closing_issue_refs_from_text "$text" \
    | cut -d'|' -f2 \
    | sed -E 's/^#([0-9]+)$/\1/' \
    | grep -E '^[0-9]+$' \
    | sort -u
}

collect_pr_text_payload() {
  local repo="$1"
  local pr_number="$2"
  local pr_title
  local pr_body
  local commit_messages

  pr_title="$(gh pr view "$pr_number" -R "$repo" --json title -q '.title // ""' 2>/dev/null || true)"
  pr_body="$(gh pr view "$pr_number" -R "$repo" --json body -q '.body // ""' 2>/dev/null || true)"
  commit_messages="$(gh api "repos/${repo}/pulls/${pr_number}/commits" --paginate --jq '.[].commit.message' 2>/dev/null || true)"

  {
    printf '%s\n' "$pr_title"
    printf '%s\n' "$pr_body"
    printf '%s\n' "$commit_messages"
  }
}

mode=""
pr_number=""
issue_number=""
label_name="${DONE_IN_DEV_LABEL:-done-in-dev}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --on-dev-merge)
      mode="dev-merge"
      shift
      ;;
    --on-issue-closed)
      mode="issue-closed"
      shift
      ;;
    --pr)
      pr_number="${2:-}"
      shift 2
      ;;
    --issue)
      issue_number="${2:-}"
      shift 2
      ;;
    --label)
      label_name="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Error: unknown argument '$1'." >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ "$mode" != "dev-merge" && "$mode" != "issue-closed" ]]; then
  echo "Error: one mode is required (--on-dev-merge or --on-issue-closed)." >&2
  usage >&2
  exit 2
fi

require_cmd gh
require_cmd jq

repo_name="$(resolve_repo_name)"
if [[ -z "$repo_name" ]]; then
  echo "Error: unable to resolve repository name." >&2
  exit 3
fi

if ! label_exists "$repo_name" "$label_name"; then
  echo "Warning: label '${label_name}' does not exist in ${repo_name}; skipping."
  exit 0
fi

if [[ "$mode" == "dev-merge" ]]; then
  require_number "--pr" "$pr_number"

  pr_state="$(gh pr view "$pr_number" -R "$repo_name" --json state -q '.state // ""' 2>/dev/null || true)"
  if [[ "$pr_state" != "MERGED" ]]; then
    echo "PR #${pr_number} is not merged; nothing to do."
    exit 0
  fi

  mapfile -t issue_numbers < <(extract_closing_issue_numbers "$(collect_pr_text_payload "$repo_name" "$pr_number")")
  if [[ ${#issue_numbers[@]} -eq 0 ]]; then
    echo "No closing issue refs found for PR #${pr_number}."
    exit 0
  fi

  for n in "${issue_numbers[@]}"; do
    state="$(issue_state "$repo_name" "$n")"
    if [[ -z "$state" ]]; then
      echo "Issue #${n}: unreadable; skipping."
      continue
    fi
    if [[ "$state" != "OPEN" ]]; then
      echo "Issue #${n}: state=${state}; skipping done-in-dev label."
      continue
    fi
    if issue_has_label "$repo_name" "$n" "$label_name"; then
      echo "Issue #${n}: label '${label_name}' already present."
      continue
    fi

    gh issue edit "$n" -R "$repo_name" --add-label "$label_name" >/dev/null
    echo "Issue #${n}: added label '${label_name}'."
  done
  exit 0
fi

require_number "--issue" "$issue_number"

if issue_has_label "$repo_name" "$issue_number" "$label_name"; then
  gh issue edit "$issue_number" -R "$repo_name" --remove-label "$label_name" >/dev/null
  echo "Issue #${issue_number}: removed label '${label_name}'."
else
  echo "Issue #${issue_number}: label '${label_name}' not present."
fi
