#!/usr/bin/env bash
set -euo pipefail

# Usage: ./check_priority_issues.sh
# Lists high priority and security issues from GitHub

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"

info "Fetching high priority issues..."

# Get issues with high priority label
HIGH_PRIORITY_ISSUES=$(gh issue list --label "high priority" --state open --json number,title,labels,url --jq '.[] | "\(.number)|\(.title)|\(.url)"' 2>/dev/null || echo "")

# Get issues with security label
SECURITY_ISSUES=$(gh issue list --label "security" --state open --json number,title,labels,url --jq '.[] | "\(.number)|\(.title)|\(.url)"' 2>/dev/null || echo "")

# Combine and deduplicate
ALL_PRIORITY_ISSUES=$(printf "%s\n%s" "$HIGH_PRIORITY_ISSUES" "$SECURITY_ISSUES" | grep -v '^$' | sort -u)

if [[ -z "$ALL_PRIORITY_ISSUES" ]]; then
  info "No high priority or security issues found."
  exit 0
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  🔥 HIGH PRIORITY & SECURITY ISSUES"
echo "═══════════════════════════════════════════════════════════════"
echo ""

count=0
while IFS='|' read -r number title url; do
  if [[ -n "$number" ]]; then
    count=$((count + 1))
    echo "[$count] Issue #$number"
    echo "    Title: $title"
    echo "    URL:   $url"
    echo ""
  fi
done <<< "$ALL_PRIORITY_ISSUES"

echo "═══════════════════════════════════════════════════════════════"
echo "Total priority issues: $count"
echo "═══════════════════════════════════════════════════════════════"
