#!/bin/bash
set -euo pipefail

# Usage: ./add_commit_push.sh "Commit message"
# Description: Adds all changes, commits with the provided message, and pushes the current branch.

# Check if a commit message is provided
if [ "$#" -ne 1 ]; then
    echo "Error: You must provide a commit message."
    echo "Usage: $0 \"Commit message\""
    exit 1
fi

COMMIT_MESSAGE="$1"

# Add all changes
echo "Adding changes..."
git add .

# Create the commit
echo "Creating commit..."
git commit -m "$COMMIT_MESSAGE"

# Get the current branch name
BRANCH_NAME=$(git branch --show-current)

# Check if a branch is active
if [ -z "$BRANCH_NAME" ]; then
    echo "Error: No active local branch. You must be on a branch to use this." >&2
    exit 1
fi

# Push the current branch
if git push origin "$BRANCH_NAME"; then
    echo "✓ Changes successfully pushed to branch '$BRANCH_NAME'."
else
    echo "Error: Failed to push changes." >&2
    exit 1
fi