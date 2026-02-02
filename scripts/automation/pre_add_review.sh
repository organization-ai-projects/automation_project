#!/usr/bin/env bash
set -euo pipefail

# Pre-add reviewer: checks code before staging
# Runs fmt, clippy, tests, and pattern checks

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"

require_git_repo
require_cmd cargo

cd "$ROOT_DIR"

info "Running pre-add review..."

ISSUES=0

# 1. Check formatting
info "Checking code formatting..."
if cargo fmt --all -- --check; then
  info "✓ Code is properly formatted."
else
  warn "⚠ Code formatting issues detected. Run: cargo fmt"
  ISSUES=$((ISSUES + 1))
fi

# 2. Run clippy
info "Running clippy..."
if cargo clippy --workspace --all-targets -- -D warnings; then
  info "✓ No clippy warnings."
else
  warn "⚠ Clippy warnings detected."
  ISSUES=$((ISSUES + 1))
fi

# 3. Run tests
info "Running tests..."
if cargo test --workspace; then
  info "✓ All tests passed."
else
  warn "⚠ Some tests failed."
  ISSUES=$((ISSUES + 1))
fi

# 4. Check for problematic patterns
info "Checking for problematic patterns..."

PATTERNS=("unwrap(" "expect(" "todo!" "unimplemented!" "panic!")
FOUND_PATTERNS=0

for pattern in "${PATTERNS[@]}"; do
  if git diff --cached --unified=0 | grep -E "^\+" | grep -v "^+++" | grep -q "$pattern"; then
    warn "⚠ Found '$pattern' in staged changes."
    FOUND_PATTERNS=$((FOUND_PATTERNS + 1))
  fi
done

if [[ $FOUND_PATTERNS -eq 0 ]]; then
  info "✓ No problematic patterns found."
else
  warn "⚠ Found $FOUND_PATTERNS problematic pattern(s) in staged changes."
  info "Consider reviewing uses of unwrap, expect, todo, unimplemented, and panic."
  ISSUES=$((ISSUES + 1))
fi

# 5. Summarize touched crates
info "Summarizing touched crates..."
TOUCHED_CRATES=$(git diff --cached --name-only | grep -E "^projects/(libraries|products)/" | cut -d/ -f1-4 | sort -u || true)

if [[ -n "$TOUCHED_CRATES" ]]; then
  info "Touched crates:"
  echo "$TOUCHED_CRATES" | sed 's/^/  - /'
else
  info "No crates touched."
fi

# Summary
echo ""
if [[ $ISSUES -eq 0 ]]; then
  info "✅ Pre-add review passed! Ready to stage changes."
  exit 0
else
  warn "⚠ Pre-add review found $ISSUES issue(s). Please review before staging."
  exit 1
fi
