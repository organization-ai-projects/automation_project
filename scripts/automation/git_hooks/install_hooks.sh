#!/usr/bin/env bash
set -euo pipefail

# Usage: ./install_hooks.sh
# Installs git hooks into the git hooks directory (supports standard clones and worktrees)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

echo "🔧 Installing git hooks..."
echo ""

# Check if we're in a git repository (works in both standard clones and worktrees)
if ! git -C "$ROOT_DIR" rev-parse --is-inside-work-tree > /dev/null 2>&1; then
  echo "❌ Error: Not in a git repository root"
  exit 1
fi

# shellcheck source=scripts/automation/git_hooks/lib/markdownlint_policy.sh
source "$SCRIPT_DIR/lib/markdownlint_policy.sh"

# Resolve the hooks directory via git (works for standard clones and worktrees)
GIT_HOOKS_DIR="$(git -C "$ROOT_DIR" rev-parse --git-path hooks)"
# Make absolute if relative (standard clone returns relative path; worktree returns absolute)
[[ "$GIT_HOOKS_DIR" == /* ]] || GIT_HOOKS_DIR="$ROOT_DIR/$GIT_HOOKS_DIR"

# Create hooks directory if it doesn't exist
mkdir -p "$GIT_HOOKS_DIR"

# Install pre-commit hook
if [[ -f "$SCRIPT_DIR/pre-commit" ]]; then
  cp "$SCRIPT_DIR/pre-commit" "$GIT_HOOKS_DIR/pre-commit"
  chmod +x "$GIT_HOOKS_DIR/pre-commit"
  echo "✅ Installed pre-commit hook"
else
  echo "⚠️  pre-commit hook not found"
fi

# Install prepare-commit-msg hook
if [[ -f "$SCRIPT_DIR/prepare-commit-msg" ]]; then
  cp "$SCRIPT_DIR/prepare-commit-msg" "$GIT_HOOKS_DIR/prepare-commit-msg"
  chmod +x "$GIT_HOOKS_DIR/prepare-commit-msg"
  echo "✅ Installed prepare-commit-msg hook"
else
  echo "⚠️  prepare-commit-msg hook not found"
fi

# Install commit-msg hook
if [[ -f "$SCRIPT_DIR/commit-msg" ]]; then
  cp "$SCRIPT_DIR/commit-msg" "$GIT_HOOKS_DIR/commit-msg"
  chmod +x "$GIT_HOOKS_DIR/commit-msg"
  echo "✅ Installed commit-msg hook"
else
  echo "⚠️  commit-msg hook not found"
fi

# Install pre-push hook
if [[ -f "$SCRIPT_DIR/pre-push" ]]; then
  cp "$SCRIPT_DIR/pre-push" "$GIT_HOOKS_DIR/pre-push"
  chmod +x "$GIT_HOOKS_DIR/pre-push"
  echo "✅ Installed pre-push hook"
else
  echo "⚠️  pre-push hook not found"
fi

# Install post-checkout hook
if [[ -f "$SCRIPT_DIR/post-checkout" ]]; then
  cp "$SCRIPT_DIR/post-checkout" "$GIT_HOOKS_DIR/post-checkout"
  chmod +x "$GIT_HOOKS_DIR/post-checkout"
  echo "✅ Installed post-checkout hook"
else
  echo "⚠️  post-checkout hook not found"
fi

# Install pre-branch-create hook
if [[ -f "$SCRIPT_DIR/pre-branch-create" ]]; then
  cp "$SCRIPT_DIR/pre-branch-create" "$GIT_HOOKS_DIR/pre-branch-create"
  chmod +x "$GIT_HOOKS_DIR/pre-branch-create"
  echo "✅ Installed pre-branch-create hook"
else
  echo "⚠️  pre-branch-create hook not found"
fi

# Install branch-creation-check hook
if [[ -f "$SCRIPT_DIR/branch-creation-check.sh" ]]; then
  cp "$SCRIPT_DIR/branch-creation-check.sh" "$GIT_HOOKS_DIR/branch-creation-check"
  chmod +x "$GIT_HOOKS_DIR/branch-creation-check"
  echo "✅ Installed branch-creation-check hook"
else
  echo "⚠️  branch-creation-check.sh not found"
fi

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Git hooks installed successfully!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Hooks installed:"
echo "  • pre-commit  - Runs formatting and branch checks before commit"
echo "  • prepare-commit-msg - Auto-generates conventional commit subject"
echo "  • commit-msg  - Validates commit message format"
echo "  • pre-push    - Runs fmt, clippy, tests before push"
echo "  • post-checkout - Warns on root-parent issue refs in branch history"
echo "  • branch-creation-check - Validates branch creation rules"
echo ""
echo "Bypass options (emergency only):"
echo "  • SKIP_PRE_COMMIT=1 git commit ..."
echo "  • SKIP_PREPARE_COMMIT_MSG=1 git commit ..."
echo "  • SKIP_COMMIT_VALIDATION=1 git commit ..."
echo "  • SKIP_POST_CHECKOUT_CONVENTION_WARN=1 git checkout ..."
echo "  • SKIP_PRE_PUSH=1 git push"
echo "  • ALLOW_PART_OF_ONLY_PUSH=1 git push"
echo ""

expected_mdl_version="$(markdownlint_policy_expected_version)"
global_mdl_bin="$(command -v markdownlint-cli2 2>/dev/null || true)"
global_mdl_version="$(markdownlint_policy_version_of_bin "$global_mdl_bin" || true)"
local_mdl_bin="$ROOT_DIR/node_modules/.bin/markdownlint-cli2"
local_mdl_version=""
if [[ -x "$local_mdl_bin" ]]; then
  local_mdl_version="$(markdownlint_policy_version_of_bin "$local_mdl_bin" || true)"
fi

echo "Markdown lint tool check:"
if [[ -z "$expected_mdl_version" ]]; then
  echo "  • expected version: not found in package.json"
else
  echo "  • expected version (package.json): $expected_mdl_version"
fi
if [[ -n "$global_mdl_bin" ]]; then
  echo "  • global: $global_mdl_bin (version: ${global_mdl_version:-unknown})"
else
  echo "  • global: not found"
fi
if [[ -x "$local_mdl_bin" ]]; then
  echo "  • local: $local_mdl_bin (version: ${local_mdl_version:-unknown})"
else
  echo "  • local: not found"
fi

if [[ -n "$expected_mdl_version" ]]; then
  if [[ "$global_mdl_version" == "$expected_mdl_version" || "$local_mdl_version" == "$expected_mdl_version" ]]; then
    echo "  • status: ✅ compatible markdownlint-cli2 found"
  else
    echo "  • status: ⚠️ no compatible markdownlint-cli2 detected"
    echo "    Use global version $expected_mdl_version or local node_modules with that version."
  fi
fi
