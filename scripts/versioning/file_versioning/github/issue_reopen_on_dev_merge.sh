#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/issue_refs.sh
source "${SCRIPT_DIR}/lib/issue_refs.sh"

usage() {
  cat <<EOF
Usage:
  $0 --pr PR_NUMBER [--label LABEL]
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

extract_reopen_issue_numbers() {
  local text="$1"
  parse_reopen_issue_refs_from_text "$text" \
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

pr_number=""
label_name="${DONE_IN_DEV_LABEL:-done-in-dev}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --pr)
      pr_number="${2:-}"
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

require_number "--pr" "$pr_number"
require_cmd gh
require_cmd jq

repo_name="$(resolve_repo_name)"
if [[ -z "$repo_name" ]]; then
  echo "Error: unable to resolve repository name." >&2
  exit 3
fi

pr_state="$(gh pr view "$pr_number" -R "$repo_name" --json state -q '.state // ""' 2>/dev/null || true)"
if [[ "$pr_state" != "MERGED" ]]; then
  echo "PR #${pr_number} is not merged; nothing to do."
  exit 0
fi

payload="$(collect_pr_text_payload "$repo_name" "$pr_number")"
mapfile -t reopen_issue_numbers < <(extract_reopen_issue_numbers "$payload")
if [[ ${#reopen_issue_numbers[@]} -eq 0 ]]; then
  echo "No reopen issue refs found for PR #${pr_number}."
  exit 0
fi

label_available="false"
if label_exists "$repo_name" "$label_name"; then
  label_available="true"
fi

for n in "${reopen_issue_numbers[@]}"; do
  state="$(issue_state "$repo_name" "$n")"
  if [[ -z "$state" ]]; then
    echo "Issue #${n}: unreadable; skipping reopen sync."
    continue
  fi

  if [[ "$state" == "CLOSED" ]]; then
    gh issue reopen "$n" -R "$repo_name" >/dev/null
    echo "Issue #${n}: reopened from Reopen ref."
  else
    echo "Issue #${n}: state=${state}; no reopen needed."
  fi

  if [[ "$label_available" == "true" ]] && issue_has_label "$repo_name" "$n" "$label_name"; then
    gh issue edit "$n" -R "$repo_name" --remove-label "$label_name" >/dev/null
    echo "Issue #${n}: removed label '${label_name}' due to Reopen ref."
  fi
done
