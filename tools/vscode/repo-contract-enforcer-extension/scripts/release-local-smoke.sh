#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "[smoke] validating package.json"
jq empty package.json

echo "[smoke] validating shell scripts syntax"
bash -n scripts/*.sh

echo "[smoke] validating extension.js syntax"
node --check extension.js

echo "[smoke] running node tests"
node --test extension.test.js extension.integration.test.js

if ! command -v vsce >/dev/null 2>&1; then
  echo "[smoke] error: 'vsce' not found in PATH"
  echo "[smoke] install it with: npm i -g @vscode/vsce"
  exit 1
fi

echo "[smoke] packaging vsix"
rm -f repo-contract-enforcer-diagnostics-*.vsix
vsce package --allow-missing-repository --no-dependencies

VSIX_PATH="$(ls -1t repo-contract-enforcer-diagnostics-*.vsix 2>/dev/null | head -n1 || true)"
if [[ -z "$VSIX_PATH" ]]; then
  echo "[smoke] error: vsix package not produced"
  exit 1
fi

echo "[smoke] produced: $VSIX_PATH"
sha256sum "$VSIX_PATH"

if [[ "${INSTALL_AFTER_SMOKE:-false}" == "true" ]]; then
  echo "[smoke] installing produced vsix"
  bash ./scripts/reinstall-local.sh
fi

echo "[smoke] done"
