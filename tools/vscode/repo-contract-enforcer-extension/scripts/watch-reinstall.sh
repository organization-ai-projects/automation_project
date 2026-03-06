#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

cd "$ROOT_DIR"

echo "Watching extension files. Press Ctrl+C to stop."

last_state=""
while true; do
  current_state="$({
    sha256sum extension.js package.json README.md LICENSE scripts/*.sh 2>/dev/null || true
  } | sha256sum | awk '{print $1}')"

  if [[ "$current_state" != "$last_state" ]]; then
    last_state="$current_state"
    echo "[watch] Change detected -> repack + reinstall"
    bash ./scripts/repack-reinstall.sh || echo "[watch] Reinstall failed; waiting for next change"
  fi

  sleep 2
done
