#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/lib/issue_refs.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/lib/directive_resolution.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/../../../common_lib/versioning/file_versioning/github/issue_helpers.sh"

usage() {
  cat <<USAGE
Usage:
  $0 --pr PR_NUMBER [--repo owner/name]

Notes:
  - Detects Closes/Fixes + Reopen directives targeting the same issue in PR body.
  - Requires explicit per-issue decision:
      Directive Decision: #<issue> => close|reopen
  - Writes a deterministic decision/conflict section into PR body.
  - Exits non-zero when unresolved conflicts remain.
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

MARKER="<!-- directive-conflict-guard:${pr_number} -->"
BLOCK_START="<!-- directive-conflicts:start -->"
BLOCK_END="<!-- directive-conflicts:end -->"

upsert_conflict_block_in_body() {
  local body="$1"
  local block="$2"
  local without_block

  without_block="$(
    perl -0777 -pe "s@\n?${BLOCK_START}.*?${BLOCK_END}\n?@@s" <<< "$body"
  )"

  if [[ -z "$block" ]]; then
    printf "%s" "$without_block"
    return
  fi

  printf "%s\n\n%s\n" "$without_block" "$block"
}

pr_json="$(gh pr view "$pr_number" -R "$repo_name" --json body,url,number 2>/dev/null || true)"
if [[ -z "$pr_json" ]]; then
  echo "Error: unable to read PR #${pr_number}." >&2
  exit 4
fi

original_body="$(echo "$pr_json" | jq -r '.body // ""')"
updated_body="$original_body"

declare -A unresolved_conflict
declare -A resolved_conflict
unresolved_count=0
resolved_count=0

commit_messages="$(gh api "repos/${repo_name}/pulls/${pr_number}/commits" --paginate --jq '.[].commit.message' 2>/dev/null || true)"
directive_payload="${commit_messages}"$'\n'"${original_body}"
while IFS='|' read -r issue_key close_flag reopen_flag decision source reason; do
  issue_key="$(trim "$issue_key")"
  [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
  if [[ "$close_flag" != "1" || "$reopen_flag" != "1" ]]; then
    continue
  fi
  case "$source" in
    explicit)
      resolved_conflict["$issue_key"]="${decision} (explicit)"
      resolved_count=$((resolved_count + 1))
      ;;
    inferred)
      resolved_conflict["$issue_key"]="${decision} (inferred from latest directive)"
      resolved_count=$((resolved_count + 1))
      ;;
    unresolved|*)
      if [[ "$reason" == "closes-and-reopen-across-multiple-source-branches" ]]; then
        unresolved_conflict["$issue_key"]="Closes + Reopen detected across multiple source branches; explicit decision required."
      else
        unresolved_conflict["$issue_key"]="Closes + Reopen detected without explicit decision."
      fi
      unresolved_count=$((unresolved_count + 1))
      ;;
  esac
done < <(resolve_issue_directives "$directive_payload" "$original_body" "$commit_messages")

# Apply explicit close decision by neutralizing Reopen refs.
for issue_key in "${!resolved_conflict[@]}"; do
  if [[ "${resolved_conflict[$issue_key]}" != close* ]]; then
    continue
  fi
  escaped_issue_key="$(printf '%s' "$issue_key" | sed 's/[^^]/[&]/g; s/\^/\\^/g')"
  updated_body="$(
    perl -0777 -pe "s/\\b((?:reopen|reopens))\\b(\\s+)(?!rejected\\b)([^\\s]*${escaped_issue_key})\\b/\$1\$2rejected \$3/ig" \
      <<< "$updated_body"
  )"
done

if [[ "$resolved_count" -gt 0 || "$unresolved_count" -gt 0 ]]; then
  conflict_block="${BLOCK_START}
### Issue Directive Decisions
"
  if [[ "$resolved_count" -gt 0 ]]; then
    conflict_block+=$'\n'"Resolved decisions:"$'\n'
    for issue_key in "${!resolved_conflict[@]}"; do
      conflict_block+="- ${issue_key} => ${resolved_conflict[$issue_key]}"$'\n'
    done
  fi
  if [[ "$unresolved_count" -gt 0 ]]; then
    conflict_block+=$'\n'"❌ Unresolved conflicts (merge blocked):"$'\n'
    for issue_key in "${!unresolved_conflict[@]}"; do
      conflict_block+="- ${issue_key}: ${unresolved_conflict[$issue_key]}"$'\n'
    done
    conflict_block+=$'\n'"Required decision format:"$'\n'
    conflict_block+="- \`Directive Decision: #<issue> => close\`"$'\n'
    conflict_block+="- \`Directive Decision: #<issue> => reopen\`"$'\n'
  fi
  conflict_block+="${BLOCK_END}"
  updated_body="$(upsert_conflict_block_in_body "$updated_body" "$conflict_block")"
else
  updated_body="$(upsert_conflict_block_in_body "$updated_body" "")"
fi

if [[ "$updated_body" != "$original_body" ]]; then
  gh pr edit "$pr_number" -R "$repo_name" --body "$updated_body" >/dev/null
fi

if [[ "$unresolved_count" -gt 0 ]]; then
  comment_body="$MARKER
### Directive Conflict Guard

❌ Unresolved Closes/Reopen conflicts detected. Add explicit directive decisions in PR body."
  github_issue_upsert_marker_comment "$repo_name" "$pr_number" "$MARKER" "$comment_body"
  echo "Unresolved directive conflicts detected for PR #${pr_number}." >&2
  exit 8
fi

if [[ "$resolved_count" -gt 0 ]]; then
  comment_body="$MARKER
### Directive Conflict Guard

✅ Directive conflicts resolved via explicit decisions."
  github_issue_upsert_marker_comment "$repo_name" "$pr_number" "$MARKER" "$comment_body"
fi

echo "Directive conflict guard evaluated for PR #${pr_number}."
