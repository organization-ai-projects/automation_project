#!/usr/bin/env bash
set -euo pipefail

# Usage: ./start_work.sh
# Orchestrates the complete workflow:
#   1. Synchronize dev with main
#   2. Show high priority issues
#   3. Create branch (from issue or custom)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"

# Save current branch to restore on exit (handle detached HEAD safely)
INITIAL_BRANCH="$(git branch --show-current 2>/dev/null || true)"
trap '[[ -n "${INITIAL_BRANCH:-}" ]] && git switch "$INITIAL_BRANCH" >/dev/null 2>&1 || true' EXIT

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"

# Validate hard dependencies required by all sub-scripts (fail fast)
require_cmd git
require_cmd gh
require_cmd jq
require_git_repo

# Function to validate branch name
validate_branch_name() {
  local branch_name="$1"

  # Check not empty
  [[ -n "$branch_name" ]] || die "Branch name cannot be empty"

  # Check for spaces
  [[ "$branch_name" != *" "* ]] || die "Branch name must not contain spaces"

  # Check for dangerous patterns
  [[ "$branch_name" != *".."* ]] || die "Branch name must not contain '..'"
  [[ "$branch_name" != *"~"* && "$branch_name" != *"^"* && "$branch_name" != *":"* ]] || die "Branch name contains forbidden characters (~ ^ :)"

  # Check valid prefix
  if ! [[ "$branch_name" =~ ^(feat|feature|fix|docs|doc|refactor|test|tests|chore|fixture)/ ]]; then
    die "Branch name must start with a valid prefix: feat/, feature/, fix/, docs/, doc/, refactor/, test/, tests/, chore/, fixture/"
  fi
}

info "🚀 Starting work workflow"
echo ""

# Step 1: Synchronize dev with main
info "Step 1/3: Synchronizing dev with main..."
echo ""
if bash "$SCRIPT_DIR/../read/synch_main_dev.sh"; then
  info "✓ Synchronization complete"
else
  die "Synchronization failed. Please fix issues before continuing."
fi
echo ""

# Step 2: Show priority issues
info "Step 2/3: Checking priority issues..."
echo ""
bash "$SCRIPT_DIR/../read/check_priority_issues.sh"
echo ""

# Step 3: Create branch
info "Step 3/3: Branch creation"
echo ""
echo "Choose an option:"
echo "  [1-9] - Create branch from issue number"
echo "  [c]   - Create custom branch name"
echo "  [s]   - Skip (work on existing branch)"
echo ""
read -rp "Your choice: " choice

case "$choice" in
  [0-9]*)
    # Validate issue number format strictly
    if ! [[ "$choice" =~ ^[0-9]+$ ]]; then
      die "Invalid issue number: $choice"
    fi
    ISSUE_NUM="$choice"

    # Get issue details from GitHub
    info "Fetching issue #$ISSUE_NUM details..."
    ISSUE_DATA=$(gh issue view "$ISSUE_NUM" --json title,labels 2>/dev/null || die "Failed to fetch issue #$ISSUE_NUM")

    ISSUE_TITLE=$(echo "$ISSUE_DATA" | jq -r '.title' | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/-/g' | sed 's/--*/-/g' | sed 's/^-//' | sed 's/-$//' | cut -c1-50)
    ISSUE_LABELS=$(echo "$ISSUE_DATA" | jq -r '.labels // [] | .[].name')

    # Determine branch type from labels (default: feat)
    BRANCH_TYPE="feat"
    if echo "$ISSUE_LABELS" | grep -qi "bug\|fix"; then
      BRANCH_TYPE="fix"
    elif echo "$ISSUE_LABELS" | grep -qi "doc"; then
      BRANCH_TYPE="docs"
    elif echo "$ISSUE_LABELS" | grep -qi "refactor"; then
      BRANCH_TYPE="refactor"
    elif echo "$ISSUE_LABELS" | grep -qi "test"; then
      BRANCH_TYPE="test"
    elif echo "$ISSUE_LABELS" | grep -qi "chore"; then
      BRANCH_TYPE="chore"
    fi

    BRANCH_NAME="${BRANCH_TYPE}/issue-${ISSUE_NUM}-${ISSUE_TITLE}"

    info "Generated branch name: $BRANCH_NAME"
      validate_branch_name "$BRANCH_NAME"
    read -rp "Confirm? [Y/n] " confirm
    if [[ "$confirm" =~ ^[Nn] ]]; then
      read -rp "Enter custom branch name: " BRANCH_NAME
    fi

    bash "$ROOT_DIR/scripts/versioning/file_versioning/git/create_branch.sh" "$BRANCH_NAME"
    ;;

  [Cc])
    # User wants custom branch name
    echo ""
    echo "Branch name must start with one of:"
    echo "  feat/, feature/, fix/, docs/, doc/, refactor/, test/, tests/, chore/, fixture/"
    echo ""
    read -rp "Enter branch name: " BRANCH_NAME

    validate_branch_name "$BRANCH_NAME"
    bash "$ROOT_DIR/scripts/versioning/file_versioning/git/create_branch.sh" "$BRANCH_NAME"
    ;;

  [Ss])
    info "Skipping branch creation. You can work on your current branch."
    ;;

  *)
    die "Invalid choice: $choice"
    ;;
esac

echo ""
info "✅ Workflow complete! Ready to start coding."
