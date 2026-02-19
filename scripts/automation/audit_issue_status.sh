#!/usr/bin/env bash
set -euo pipefail

# Audit open issues against commit references in a branch range.
#
# Outputs:
# - would-close-on-merge: issue referenced by Closes/Fixes/Resolves in range
# - part-of-only: issue referenced by Part of/Related to in range, without closing keyword
# - unreferenced: no issue keyword reference found in range
#
# Usage:
#   scripts/automation/audit_issue_status.sh [--repo OWNER/REPO] [--base origin/main] [--head origin/dev] [--limit 200] [--output /tmp/issue_audit.md]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/../versioning/file_versioning/github/lib/issue_refs.sh"

REPO=""
BASE_REF="origin/main"
HEAD_REF="origin/dev"
LIMIT=200
OUTPUT_FILE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo)
      REPO="${2:-}"
      shift 2
      ;;
    --base)
      BASE_REF="${2:-}"
      shift 2
      ;;
    --head)
      HEAD_REF="${2:-}"
      shift 2
      ;;
    --limit)
      LIMIT="${2:-}"
      shift 2
      ;;
    --output)
      OUTPUT_FILE="${2:-}"
      shift 2
      ;;
    -h|--help)
      cat <<'EOF'
Usage: scripts/automation/audit_issue_status.sh [options]

Options:
  --repo OWNER/REPO   GitHub repository (default: GH_REPO or current gh repo)
  --base REF          Base ref for compare range (default: origin/main)
  --head REF          Head ref for compare range (default: origin/dev)
  --limit N           Max open issues to fetch (default: 200)
  --output FILE       Write markdown report to file (default: stdout only)
EOF
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 2
      ;;
  esac
done

if ! command -v gh >/dev/null 2>&1; then
  echo "Error: command 'gh' not found." >&2
  exit 3
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "Error: command 'jq' not found." >&2
  exit 3
fi

if [[ -z "$REPO" ]]; then
  if [[ -n "${GH_REPO:-}" ]]; then
    REPO="$GH_REPO"
  else
    REPO="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"
  fi
fi

if [[ -z "$REPO" ]]; then
  echo "Error: unable to resolve repository. Pass --repo OWNER/REPO." >&2
  exit 4
fi

RANGE="${BASE_REF}..${HEAD_REF}"

tmpdir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmpdir"
}
trap cleanup EXIT

open_json="$tmpdir/open_issues.json"
messages_file="$tmpdir/commit_messages.txt"
closing_refs="$tmpdir/closing_refs.txt"
part_refs="$tmpdir/part_refs.txt"

gh issue list -R "$REPO" --state open --limit "$LIMIT" --json number,title,url,body,state,labels > "$open_json"

git log "$RANGE" --format=%B > "$messages_file"

closing_lines="$(parse_closing_issue_refs_from_text "$(cat "$messages_file")" || true)"
if [[ -n "$closing_lines" ]]; then
  echo "$closing_lines" | awk -F'|' '{ gsub(/^#/, "", $2); print $2 }' | sort -u > "$closing_refs"
else
  : > "$closing_refs"
fi

part_lines="$(parse_non_closing_issue_refs_from_text "$(cat "$messages_file")" || true)"
if [[ -n "$part_lines" ]]; then
  echo "$part_lines" | awk -F'|' '{ gsub(/^#/, "", $2); print $2 }' | sort -u > "$part_refs"
else
  : > "$part_refs"
fi

report="$tmpdir/report.md"
{
  echo "# Issue Status Audit"
  echo
  echo "- Repository: \`$REPO\`"
  echo "- Range: \`$RANGE\`"
  echo
} > "$report"

total_open="$(jq 'length' "$open_json")"

would_close=0
part_only=0
unreferenced=0
done_in_dev=0

would_close_items="$tmpdir/would_close.md"
part_only_items="$tmpdir/part_only.md"
unreferenced_items="$tmpdir/unreferenced.md"
done_in_dev_items="$tmpdir/done_in_dev.md"
: > "$would_close_items"
: > "$part_only_items"
: > "$unreferenced_items"
: > "$done_in_dev_items"

jq -c '.[]' "$open_json" | while IFS= read -r row; do
  num="$(echo "$row" | jq -r '.number')"
  title="$(echo "$row" | jq -r '.title')"
  url="$(echo "$row" | jq -r '.url')"
  body="$(echo "$row" | jq -r '.body // ""')"
  labels_csv="$(echo "$row" | jq -r '.labels // [] | map(.name | ascii_downcase) | join(",")')"

  parent="(none)"
  parent_line="$(printf '%s\n' "$body" | grep -iE '^[[:space:]]*Parent:[[:space:]]*(#?[0-9]+|none)[[:space:]]*$' | tail -n1 || true)"
  if [[ -n "$parent_line" ]]; then
    parent="$(echo "$parent_line" | sed -E 's/^[[:space:]]*Parent:[[:space:]]*//I')"
  fi

  if [[ "$labels_csv" == *"done-in-dev"* ]]; then
    done_in_dev=$((done_in_dev + 1))
    printf -- "- [#%s](%s) %s (parent: %s)\n" "$num" "$url" "$title" "$parent" >> "$done_in_dev_items"
  elif grep -Fxq "$num" "$closing_refs"; then
    would_close=$((would_close + 1))
    printf -- "- [#%s](%s) %s (parent: %s)\n" "$num" "$url" "$title" "$parent" >> "$would_close_items"
  elif grep -Fxq "$num" "$part_refs"; then
    part_only=$((part_only + 1))
    printf -- "- [#%s](%s) %s (parent: %s)\n" "$num" "$url" "$title" "$parent" >> "$part_only_items"
  else
    unreferenced=$((unreferenced + 1))
    printf -- "- [#%s](%s) %s (parent: %s)\n" "$num" "$url" "$title" "$parent" >> "$unreferenced_items"
  fi
done

# shellcheck disable=SC2034
would_close="$(wc -l < "$would_close_items" | tr -d ' ')"
# shellcheck disable=SC2034
part_only="$(wc -l < "$part_only_items" | tr -d ' ')"
# shellcheck disable=SC2034
unreferenced="$(wc -l < "$unreferenced_items" | tr -d ' ')"
# shellcheck disable=SC2034
done_in_dev="$(wc -l < "$done_in_dev_items" | tr -d ' ')"

{
  echo "## Summary"
  echo
  echo "- Open issues fetched: $total_open"
  echo "- Would close on merge: $would_close"
  echo "- Done in dev (label): $done_in_dev"
  echo "- Part-of-only (not closing): $part_only"
  echo "- Unreferenced in range: $unreferenced"
  echo
  echo "## Done In Dev (Label)"
  echo
  if [[ -s "$done_in_dev_items" ]]; then
    cat "$done_in_dev_items"
  else
    echo "- None"
  fi
  echo
  echo "## Would Close On Merge"
  echo
  if [[ -s "$would_close_items" ]]; then
    cat "$would_close_items"
  else
    echo "- None"
  fi
  echo
  echo "## Part-Of Only"
  echo
  if [[ -s "$part_only_items" ]]; then
    cat "$part_only_items"
  else
    echo "- None"
  fi
  echo
  echo "## Unreferenced"
  echo
  if [[ -s "$unreferenced_items" ]]; then
    cat "$unreferenced_items"
  else
    echo "- None"
  fi
} >> "$report"

if [[ -n "$OUTPUT_FILE" ]]; then
  cp "$report" "$OUTPUT_FILE"
  echo "Generated file: $OUTPUT_FILE"
fi

cat "$report"
