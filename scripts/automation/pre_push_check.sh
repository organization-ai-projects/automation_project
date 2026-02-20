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
# shellcheck source=scripts/common_lib/automation/rust_checks.sh
source "$ROOT_DIR/scripts/common_lib/automation/rust_checks.sh"

require_git_repo
require_cmd cargo

cd "$ROOT_DIR"

info "Running pre-push checks..."

ISSUES=0

# 0. Markdown lint (if pnpm is available and dependencies are installed)
if command -v pnpm &> /dev/null && [[ -f "$ROOT_DIR/package.json" ]]; then
  if [[ -d "$ROOT_DIR/node_modules" ]] && [[ -f "$ROOT_DIR/node_modules/.bin/markdownlint-cli2" ]]; then
    info "Checking markdown formatting..."
    if pnpm run lint-md; then
      info "✓ Markdown lint passed."
    else
      warn "⚠ Markdown lint failed."
      ISSUES=$((ISSUES + 1))
    fi
  else
    info "ℹ Skipping markdown lint (dependencies not installed). Run 'pnpm install' to enable."
  fi
else
  info "ℹ Skipping markdown lint (pnpm not available or package.json missing)."
fi

# 1. Cargo check
info "Checking workspace..."
if rust_checks_run_check --workspace --all-targets; then
  info "✓ Cargo check passed."
else
  warn "⚠ Cargo check failed."
  ISSUES=$((ISSUES + 1))
fi

# 2. Format check
info "Checking code formatting..."
if rust_checks_run_fmt_check; then
  info "✓ Formatting check passed."
else
  warn "⚠ Formatting issues detected."
  ISSUES=$((ISSUES + 1))
fi

# 3. Clippy
info "Running clippy..."
if rust_checks_run_clippy --workspace --all-targets; then
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
if rust_checks_run_tests --workspace; then
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
