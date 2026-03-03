#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"
rm -f repo-contract-enforcer-diagnostics-*.vsix
vsce package --allow-missing-repository --no-dependencies
bash ./scripts/reinstall-local.sh
