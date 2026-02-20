#!/usr/bin/env bash
# Script: branch-creation-check
# Prevents creating or switching to a branch already in use by another worktree
set -euo pipefail

if ! command -v git &>/dev/null; then
    echo "❌ git not found" >&2
    exit 127
fi

if [[ $# -lt 1 ]]; then
    exec git
fi

COMMAND="$1"
shift

# ✅ SAVE arguments BEFORE parsing them
ORIGINAL_ARGS=("$@")

# Only intercept branch commands
if [[ "$COMMAND" != "branch" && "$COMMAND" != "checkout" && "$COMMAND" != "switch" ]]; then
    exec git "$COMMAND" "${ORIGINAL_ARGS[@]}"
fi

# ✅ Parse WITHOUT destroying $@
BRANCH=""
i=0

while [[ $i -lt ${#ORIGINAL_ARGS[@]} ]]; do
    arg="${ORIGINAL_ARGS[i]}"

    case "$arg" in
        -b|-c|--branch|--create|-B|-C|--force-create)
            # Consume the flag AND the value
            if [[ $((i+1)) -lt ${#ORIGINAL_ARGS[@]} ]]; then
                BRANCH="${ORIGINAL_ARGS[i+1]}"
                i=$((i+2))  # ✅ Skip the flag AND the branch
            else
                i=$((i+1))
            fi
            ;;
        *)
            # git branch <name> without a flag
            if [[ "$COMMAND" == "branch" && "$arg" != -* && -z "$BRANCH" ]]; then
                BRANCH="$arg"
            fi
            i=$((i+1))
            ;;
    esac
done

# Validation: is the branch in a worktree?
if [[ -n "$BRANCH" ]]; then
    if git worktree list | grep -qF "[$BRANCH]"; then
        echo "❌ The branch '$BRANCH' is already in use by another worktree:" >&2
        git worktree list | grep -F "[$BRANCH]" >&2
        echo "   Remove it with: git worktree remove <path>" >&2
        exit 1
    fi
fi

# ✅ Pass the ORIGINAL arguments to git
exec git "$COMMAND" "${ORIGINAL_ARGS[@]}"