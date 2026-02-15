#!/usr/bin/env bash
set -euo pipefail

# Usage: ./add_commit_push.sh "Commit message" [--no-verify]
# Description: Adds all changes, commits, then pushes via push_branch.sh
#
# The script enforces conventional commit message format:
#   <type>(<scope>): <message> or <type>: <message>
#
# Allowed types: feature, feat, fix, fixture, doc, docs, refactor, test, tests, chore
#
# Use --no-verify to bypass validation (not recommended)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../.." && pwd)"
PUSH_SCRIPT="$SCRIPT_DIR/push_branch.sh"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/staging.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/staging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commit.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/commit.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commit_format.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/commit_format.sh"

require_git_repo

if [[ ! -f "$PUSH_SCRIPT" ]]; then
  die "push_branch.sh not found at: $PUSH_SCRIPT"
fi

if [[ ! -x "$PUSH_SCRIPT" ]]; then
  die "push_branch.sh is not executable. Run: chmod +x $PUSH_SCRIPT"
fi

# Validate commit message format
validate_commit_message() {
  local message="$1"

  # Validate format: <type>(<scope>): <message> or <type>: <message>
  # Allows multiple scopes separated by commas and dot-scopes: feat(ci,.github): message
  if ! validate_commit_message_format "$message" '[[:space:]]'; then
    echo "❌ Invalid commit message format!" >&2
    echo "" >&2
    echo "Expected format:" >&2
    echo "  <type>(<scope>): <message>" >&2
    echo "  or" >&2
    echo "  <type>: <message>" >&2
    echo "" >&2
    echo "Allowed types:" >&2
    echo "  feature, feat, fix, fixture, doc, docs, refactor, test, tests, chore" >&2
    echo "" >&2
    echo "Examples:" >&2
    echo "  feat(projects/libraries/security): add token validation helper" >&2
    echo "  feat(ci,scripts): add workflows and sync script" >&2
    echo "  docs(.github): add default PR template" >&2
    echo "  fix: resolve null pointer exception" >&2
    echo "  docs(readme): update installation instructions" >&2
    echo "  refactor(projects/products/accounts/backend): simplify error handling" >&2
    echo "  test: add unit tests for validator" >&2
    echo "  chore: update dependencies" >&2
    echo "" >&2
    echo "Your commit message:" >&2
    echo "  $message" >&2
    echo "" >&2
    echo "To bypass validation (not recommended):" >&2
    echo "  $0 \"$message\" --no-verify" >&2
    echo "  or: SKIP_COMMIT_VALIDATION=1 git commit -m \"$message\"" >&2
    return 1
  fi
  return 0
}

# Parse arguments
if [[ "$#" -lt 1 || "$#" -gt 2 ]]; then
  die "Usage: $0 \"Commit message\" [--no-verify]"
fi

COMMIT_MESSAGE="$1"
NO_VERIFY=false

if [[ "$#" -eq 2 ]]; then
  if [[ "$2" == "--no-verify" ]]; then
    NO_VERIFY=true
    warn "⚠️  WARNING: Bypassing commit message validation. This is not recommended."
  else
    die "Invalid option: $2. Use --no-verify to bypass validation."
  fi
fi

# Validate commit message unless bypassed
if [[ "$NO_VERIFY" == false ]]; then
  if ! validate_commit_message "$COMMIT_MESSAGE"; then
    exit 1
  fi
fi

info "Adding all changes..."
git_add_all

info "Committing with message: $COMMIT_MESSAGE"
if [[ "$NO_VERIFY" == true ]]; then
  git commit --no-verify -m "$COMMIT_MESSAGE"
else
  git_commit "$COMMIT_MESSAGE"
fi

info "Pushing via push_branch.sh..."
"$PUSH_SCRIPT"

info "✅ Commit and push completed successfully."
