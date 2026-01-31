#!/usr/bin/env bash
# Run all scenarios in parallel
set -euo pipefail

HARNESS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

info() {
  echo "[$(date '+%H:%M:%S')]" "$@" >&2
}

main() {
  local -a pids=()
  local passed=0
  local failed=0

  info "Running scenarios in parallel..."

  for f in "$HARNESS_DIR/scenarios/"*.env; do
    "$HARNESS_DIR/run_all.sh" --scenario "$(basename "$f" .env | sed 's/^0*//' | cut -d_ -f1)" &
    pids+=($!)
  done

  for pid in "${pids[@]}"; do
    if wait "$pid"; then
      ((passed++))
    else
      ((failed++))
    fi
  done

  info ""
  info "═══════════════════════════════════════════"
  if [[ $failed -eq 0 ]]; then
    info "🎉 All $passed scenarios passed (parallel)!"
  else
    info "❌ $failed failed, $passed passed (parallel)"
    exit 1
  fi
}

main
