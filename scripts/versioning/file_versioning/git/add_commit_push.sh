#!/usr/bin/env bash
set -euo pipefail

# Usage: ./add_commit_push.sh "Commit message"
# Description: Adds all changes, commits, then pushes via push_branch.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/staging.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/staging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commit.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/commit.sh"

require_git_repo

if [[ "$#" -ne 1 ]]; then
  die "You must provide a commit message. Usage: $0 \"Commit message\""
fi

COMMIT_MESSAGE="$1"

info "Adding all changes..."
git_add_all

info "Committing with message: $COMMIT_MESSAGE"
git_commit "$COMMIT_MESSAGE"

info "Pushing via push_branch.sh..."
"$SCRIPT_DIR/push_branch.sh"

info "✅ Commit and push completed successfully."
