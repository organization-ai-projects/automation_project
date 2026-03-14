#!/usr/bin/env bash
# Script: branch-creation-check
# Prevents creating or switching to a branch already in use by another worktree.
# Canonical logic lives in `versioning_automation git branch-creation-check`.
set -euo pipefail

if ! command -v versioning_automation >/dev/null 2>&1; then
	echo "❌ versioning_automation not found" >&2
	echo "   Build/install it, then retry." >&2
	exit 127
fi

exec versioning_automation git branch-creation-check "$@"
