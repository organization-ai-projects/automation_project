#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VSIX_PATH="$(ls -1t "$ROOT_DIR"/repo-contract-enforcer-diagnostics-*.vsix 2>/dev/null | head -n1 || true)"

if [[ -z "$VSIX_PATH" ]]; then
  echo "No VSIX found. Run: pnpm run vsix:pack"
  exit 1
fi

if command -v code >/dev/null 2>&1; then
  VSCODE_CMD="code"
elif command -v code-insiders >/dev/null 2>&1; then
  VSCODE_CMD="code-insiders"
elif command -v codium >/dev/null 2>&1; then
  VSCODE_CMD="codium"
else
  echo "No VS Code CLI found (code/code-insiders/codium)."
  exit 1
fi

"$VSCODE_CMD" --install-extension "$VSIX_PATH" --force

echo "Installed: $VSIX_PATH"

if [[ "${RELOAD_WINDOW_AFTER_INSTALL:-true}" == "true" ]]; then
  if "$VSCODE_CMD" --command workbench.action.reloadWindow >/dev/null 2>&1; then
    echo "Triggered VS Code window reload."
  else
    echo "Auto-reload command unavailable; run: Developer: Reload Window"
  fi
fi
