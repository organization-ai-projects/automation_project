#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(cd "${ROOT_DIR}/../../.." && pwd)"
BACKEND_DIR="${REPO_ROOT}/projects/products/unstable/repo_contract_enforcer/backend"

cd "$ROOT_DIR"

detect_vscode_cmd() {
  if command -v code >/dev/null 2>&1; then
    echo "code"
    return 0
  fi
  if command -v code-insiders >/dev/null 2>&1; then
    echo "code-insiders"
    return 0
  fi
  if command -v codium >/dev/null 2>&1; then
    echo "codium"
    return 0
  fi
  return 1
}

compute_extension_state() {
  {
    sha256sum extension.js package.json README.md LICENSE scripts/*.sh 2>/dev/null || true
  } | sha256sum | awk '{print $1}'
}

compute_backend_state() {
  if [[ ! -d "$BACKEND_DIR" ]]; then
    echo "missing-backend-dir"
    return 0
  fi
  (
    cd "$BACKEND_DIR"
    find src -type f -name '*.rs' -print0 2>/dev/null
    find . -maxdepth 1 -type f \( -name 'Cargo.toml' -o -name 'Cargo.lock' \) -print0 2>/dev/null
  ) | xargs -0 stat -c '%Y:%s:%n' 2>/dev/null | sha256sum | awk '{print $1}'
}

restart_backend_processes() {
  local vscode_cmd
  vscode_cmd="$(detect_vscode_cmd || true)"
  if [[ -z "$vscode_cmd" ]]; then
    echo "[watch] VS Code CLI not found; skip backend restart trigger"
    return 0
  fi
  if "$vscode_cmd" --reuse-window --command repoContractEnforcer.restartBackend >/dev/null 2>&1; then
    echo "[watch] Triggered backend restart command in VS Code"
  else
    echo "[watch] Backend restart command unavailable; run command manually in VS Code: Repo Contract Enforcer: Restart Backend Processes"
  fi
}

echo "Watching extension and backend files. Press Ctrl+C to stop."

last_extension_state=""
last_backend_state=""
while true; do
  current_extension_state="$(compute_extension_state)"
  current_backend_state="$(compute_backend_state)"

  if [[ "$current_extension_state" != "$last_extension_state" ]]; then
    last_extension_state="$current_extension_state"
    last_backend_state="$current_backend_state"
    echo "[watch] Change detected -> repack + reinstall"
    bash ./scripts/repack-reinstall.sh || echo "[watch] Reinstall failed; waiting for next change"
  elif [[ "$current_backend_state" != "$last_backend_state" ]]; then
    last_backend_state="$current_backend_state"
    echo "[watch] Backend change detected -> restart backend processes only"
    restart_backend_processes
  fi

  sleep 2
done
