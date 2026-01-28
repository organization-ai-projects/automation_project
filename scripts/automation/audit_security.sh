#!/usr/bin/env bash
set -euo pipefail

# Run security audit on Rust dependencies
# Checks for known vulnerabilities using cargo-audit

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"

require_git_repo
require_cmd cargo

cd "$ROOT_DIR"

info "Running security audit..."

# Check if cargo-audit is installed
if ! command -v cargo-audit >/dev/null 2>&1; then
  warn "cargo-audit not found. Installing..."
  cargo install cargo-audit || die "Failed to install cargo-audit"
fi

# Update advisory database
info "Updating advisory database..."
cargo audit fetch || warn "Failed to update advisory database"

# Run audit
info "Checking for security vulnerabilities..."
if cargo audit --json > /tmp/audit-report.json 2>&1; then
  info "✅ No security vulnerabilities found!"
  RESULT=0
else
  warn "⚠ Security vulnerabilities detected!"
  RESULT=1

  # Parse and display vulnerabilities
  if command -v jq >/dev/null 2>&1; then
    VULNS=$(jq -r '.vulnerabilities.list[] | "  - \(.advisory.id): \(.advisory.title) (\(.package.name) \(.package.version))"' /tmp/audit-report.json 2>/dev/null || echo "")
    if [[ -n "$VULNS" ]]; then
      echo ""
      echo "Vulnerabilities found:"
      echo "$VULNS"
    fi
  fi

  # Show full report
  info ""
  info "Full report:"
  cargo audit
fi

# Optional: Check for unmaintained dependencies
info ""
info "Checking for unmaintained dependencies..."
if cargo audit --deny warnings 2>&1 | grep -i "unmaintained"; then
  warn "Some dependencies may be unmaintained."
fi

# Cleanup
rm -f /tmp/audit-report.json

exit $RESULT
