#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/lib/issue_refs.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/lib/issue_required_fields.sh"

usage() {
  cat <<USAGE
Usage:
  $0 --pr PR_NUMBER [--repo owner/name]

Notes:
  - Detects closure refs in PR body (Closes/Fixes/Resolves #...).
  - If referenced issue is non-compliant with required issue contract, inserts:
      "<keyword> rejected #<issue>"
    to neutralize GitHub auto-close behavior.
  - Posts/updates a deterministic status comment in the PR thread.
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

trim() {
  local s="${1:-}"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf "%s" "$s"
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

if ! command -v gh >/dev/null 2>&1; then
  echo "Error: gh is required." >&2
  exit 3
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "Error: jq is required." >&2
  exit 3
fi
if ! command -v perl >/dev/null 2>&1; then
  echo "Error: perl is required." >&2
  exit 3
fi

if [[ -z "$repo_name" ]]; then
  repo_name="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"
fi
[[ -n "$repo_name" ]] || { echo "Error: unable to determine repository." >&2; exit 3; }

MARKER="<!-- closure-neutralizer:${pr_number} -->"

upsert_pr_comment() {
  local body="$1"
  local comment_id
  comment_id="$({
    gh api "repos/${repo_name}/issues/${pr_number}/comments" --paginate
  } | jq -r --arg marker "$MARKER" '
      map(select((.body // "") | contains($marker)))
      | sort_by(.updated_at)
      | last
      | .id // empty
    ' 2>/dev/null || true)"

  if [[ -n "$comment_id" ]]; then
    gh api -X PATCH "repos/${repo_name}/issues/comments/${comment_id}" \
      -f body="$body" >/dev/null
  else
    gh api "repos/${repo_name}/issues/${pr_number}/comments" \
      -f body="$body" >/dev/null
  fi
}

issue_non_compliance_reason() {
  local issue_number="$1"
  local issue_json
  local labels
  local title
  local body
  local validations
  local first_reason

  issue_json="$(gh issue view "$issue_number" -R "$repo_name" --json labels,title,body 2>/dev/null || true)"
  if [[ -z "$issue_json" ]]; then
    echo ""
    return
  fi

  labels="$(echo "$issue_json" | jq -r '.labels | map(.name) | join("||")')"
  if [[ "$(echo "$labels" | tr '[:upper:]' '[:lower:]')" =~ (^|\|\|)issue-required-missing(\|\||$) ]]; then
    echo "label issue-required-missing is set"
    return
  fi

  title="$(echo "$issue_json" | jq -r '.title // ""')"
  body="$(echo "$issue_json" | jq -r '.body // ""')"
  validations="$(issue_validate_content "$title" "$body" || true)"
  if [[ -z "$validations" ]]; then
    echo ""
    return
  fi
  first_reason="$(echo "$validations" | awk -F'|' 'NF>=3 {print $3; exit}')"
  echo "$first_reason"
}

keyword_pattern_from_action() {
  local action="$1"
  case "$action" in
    Closes) echo "closes|close" ;;
    Fixes) echo "fixes|fix" ;;
    Resolves) echo "resolves|resolve" ;;
    *) echo "" ;;
  esac
}

pr_json="$(gh pr view "$pr_number" -R "$repo_name" --json body,url,number 2>/dev/null || true)"
if [[ -z "$pr_json" ]]; then
  echo "Error: unable to read PR #${pr_number}." >&2
  exit 4
fi

original_body="$(echo "$pr_json" | jq -r '.body // ""')"
updated_body="$original_body"

declare -A seen_ref
declare -A neutralized_reason
declare -A neutralized_action
neutralized_count=0

while IFS='|' read -r action issue_key; do
  issue_key="$(trim "$issue_key")"
  [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
  issue_number="${issue_key//#/}"
  dedupe_key="${action}|${issue_key}"
  if [[ -n "${seen_ref[$dedupe_key]:-}" ]]; then
    continue
  fi
  seen_ref["$dedupe_key"]=1

  reason="$(issue_non_compliance_reason "$issue_number")"
  [[ -n "$reason" ]] || continue

  keyword_pattern="$(keyword_pattern_from_action "$action")"
  [[ -n "$keyword_pattern" ]] || continue

  escaped_issue_key="$(printf '%s' "$issue_key" | sed 's/[^^]/[&]/g; s/\^/\\^/g')"
  updated_body="$(
    perl -0777 -pe "s/\\b((?:${keyword_pattern}))\\b(\\s+)(?!rejected\\b)([^\\s]*${escaped_issue_key})\\b/\\\$1\\\$2rejected \\\$3/ig" \
      <<< "$updated_body"
  )"

  neutralized_reason["$issue_key"]="$reason"
  neutralized_action["$issue_key"]="$action"
  neutralized_count=$((neutralized_count + 1))
done < <(parse_closing_issue_refs_from_text "$original_body")

if [[ "$updated_body" != "$original_body" ]]; then
  gh pr edit "$pr_number" -R "$repo_name" --body "$updated_body" >/dev/null
fi

if [[ "$neutralized_count" -gt 0 ]]; then
  comment_body="$MARKER
### Closure Neutralization Status

⚠️ Non-compliant issue references were neutralized to prevent incorrect auto-close.

"
  for issue_key in "${!neutralized_reason[@]}"; do
    comment_body+="- ${neutralized_action[$issue_key]} rejected ${issue_key}: ${neutralized_reason[$issue_key]}"$'\n'
  done
  comment_body+=$'\n'"How to restore standard auto-close:"$'\n'
  comment_body+="- Fix issue required fields/title contract."$'\n'
  comment_body+="- Remove \`rejected\` from closure lines in PR body."
else
  comment_body="$MARKER
### Closure Neutralization Status

✅ No non-compliant closure refs detected. No neutralization applied."
fi

upsert_pr_comment "$comment_body"

echo "Closure neutralization evaluated for PR #${pr_number}."
