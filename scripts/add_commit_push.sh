#!/bin/bash
set -euo pipefail

# Usage: ./add_commit_push.sh "Commit message"
# Description: Adds all changes, commits, then pushes via push_branch.sh

if [[ "$#" -ne 1 ]]; then
  echo "Error: You must provide a commit message."
  echo "Usage: $0 \"Commit message\"" >&2
  exit 1
fi

COMMIT_MESSAGE="$1"

echo "=== git add ==="
git add .

echo "=== git commit ==="
git commit -m "$COMMIT_MESSAGE"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "=== git push (via push_branch.sh) ==="
"$SCRIPT_DIR/push_branch.sh"

echo "✓ Commit and push completed successfully."
