#!/usr/bin/env bash
set -euo pipefail

# Pre-push checks to ensure code quality before pushing

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/branch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/branch.sh"

require_git_repo
require_cmd cargo

cd "$ROOT_DIR"

info "Running pre-push checks..."

ISSUES=0

# 0. Markdown lint (if npm is available)
if command -v npm &> /dev/null && [[ -f "$ROOT_DIR/package.json" ]]; then
  info "Checking markdown formatting..."
  if npm run lint-md; then
    info "✓ Markdown lint passed."
  else
    warn "⚠ Markdown lint failed."
    ISSUES=$((ISSUES + 1))
  fi
else
  info "ℹ Skipping markdown lint (npm not available or package.json missing)."
fi

# 1. Cargo check
info "Checking workspace..."
if cargo check --workspace --all-targets; then
  info "✓ Cargo check passed."
else
  warn "⚠ Cargo check failed."
  ISSUES=$((ISSUES + 1))
fi

# 2. Format check
info "Checking code formatting..."
if cargo fmt --all -- --check; then
  info "✓ Formatting check passed."
else
  warn "⚠ Formatting issues detected."
  ISSUES=$((ISSUES + 1))
fi

# 3. Clippy
info "Running clippy..."
if cargo clippy --workspace --all-targets -- -D warnings; then
  info "✓ Clippy passed."
else
  warn "⚠ Clippy warnings detected."
  ISSUES=$((ISSUES + 1))
fi

# 4. Check for merge conflicts with base branch
BASE_BRANCH="${BASE_BRANCH:-dev}"
CURRENT_BRANCH="$(get_current_branch)"

if [[ "$CURRENT_BRANCH" != "$BASE_BRANCH" ]] && [[ "$CURRENT_BRANCH" != "main" ]]; then
  info "Checking for potential merge conflicts with $BASE_BRANCH..."

  # Fetch latest
  git_fetch_prune "${REMOTE:-origin}"

  # Check if base branch has commits that current branch doesn't have
  if ! git merge-base --is-ancestor "${REMOTE:-origin}/$BASE_BRANCH" HEAD; then
    warn "⚠ Current branch may need to be updated from $BASE_BRANCH."
    info "Run: git pull ${REMOTE:-origin} $BASE_BRANCH"
  else
    info "✓ Branch is up-to-date with $BASE_BRANCH."
  fi
fi

# 5. Run tests
info "Running tests..."
if cargo test --workspace; then
  info "✓ All tests passed."
else
  warn "⚠ Some tests failed."
  ISSUES=$((ISSUES + 1))
fi

# Summary
echo ""
if [[ $ISSUES -eq 0 ]]; then
  info "✅ All pre-push checks passed! Safe to push."
  exit 0
else
  warn "⚠ Pre-push checks found $ISSUES issue(s). Please fix before pushing."
  exit 1
fi
