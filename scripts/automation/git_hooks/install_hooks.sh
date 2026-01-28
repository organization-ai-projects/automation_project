#!/usr/bin/env bash
set -euo pipefail

# Usage: ./install_hooks.sh
# Installs git hooks into .git/hooks/

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"
GIT_HOOKS_DIR="$ROOT_DIR/.git/hooks"

echo "🔧 Installing git hooks..."
echo ""

# Check if we're in a git repository
if [[ ! -d "$ROOT_DIR/.git" ]]; then
  echo "❌ Error: Not in a git repository root"
  exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p "$GIT_HOOKS_DIR"

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

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Git hooks installed successfully!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Hooks installed:"
echo "  • commit-msg  - Validates commit message format"
echo "  • pre-push    - Runs fmt, clippy, tests before push"
echo ""
echo "Bypass options (emergency only):"
echo "  • SKIP_COMMIT_VALIDATION=1 git commit ..."
echo "  • SKIP_PRE_PUSH=1 git push"
echo ""
