#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

usage() {
  cat <<USAGE
Usage:
  $0 --issue ISSUE_NUMBER [--repo owner/name]

Notes:
  - Finds all open PRs referencing the given issue number via closing keywords
    (Closes #N).
  - Re-evaluates closure neutralization for each such PR.
  - Useful when an issue is edited and may now satisfy (or violate) compliance.
USAGE
}

require_number() {
  local name="$1"
  local value="${2:-}"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Error: ${name} must be a positive integer." >&2
    exit 2
  fi
}

issue_number=""
repo_name="${GH_REPO:-}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --issue)
      issue_number="${2:-}"
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

[[ -n "$issue_number" ]] || { echo "Error: --issue is required." >&2; usage >&2; exit 2; }
require_number "--issue" "$issue_number"

if ! command -v gh >/dev/null 2>&1; then
  echo "Error: gh is required." >&2
  exit 3
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "Error: jq is required." >&2
  exit 3
fi

if [[ -z "$repo_name" ]]; then
  repo_name="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"
fi
[[ -n "$repo_name" ]] || { echo "Error: unable to determine repository." >&2; exit 3; }

NEUTRALIZER="${SCRIPT_DIR}/neutralize_non_compliant_closure_refs.sh"
if [[ ! -x "$NEUTRALIZER" ]]; then
  chmod +x "$NEUTRALIZER"
fi

# Find all open PRs whose body contains a closing reference to this issue number.
pr_numbers="$(
  # The [^\s]* segment intentionally allows optional owner/repo prefixes
  # (e.g. "org/repo#42") consistent with parse_closing_issue_refs_from_text.
  gh api "repos/${repo_name}/pulls?state=open&per_page=100" --paginate 2>/dev/null \
  | jq -r --argjson issue "$issue_number" '
      .[] |
      select(
        .body != null and (
          .body | test(
            "(?i)\\b(closes?)\\s+(rejected\\s+)?[^\\s]*#" + ($issue | tostring) + "\\b"
          )
        )
      ) |
      .number | tostring
    ' 2>/dev/null \
  || true
)"

if [[ -z "$pr_numbers" ]]; then
  echo "No open PRs found referencing issue #${issue_number}."
  exit 0
fi

evaluated_count=0
while IFS= read -r pr_num; do
  [[ -n "$pr_num" ]] || continue
  echo "Re-evaluating PR #${pr_num} (references issue #${issue_number})..."
  bash "$NEUTRALIZER" --pr "$pr_num" --repo "$repo_name"
  evaluated_count=$((evaluated_count + 1))
done <<< "$pr_numbers"

echo "Re-evaluation complete. ${evaluated_count} PR(s) evaluated."
