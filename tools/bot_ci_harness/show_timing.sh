#!/usr/bin/env bash
# Show timing report for all scenarios
set -euo pipefail

HARNESS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

info() {
  echo "[$(date '+%H:%M:%S')]" "$@" >&2
}

main() {
  info "Running scenarios with timing report..."

  local -a timings
  local total_ms=0

  for f in "$HARNESS_DIR/scenarios/"*.env; do
    local scenario_num
    scenario_num=$(basename "$f" .env | sed 's/^0*//' | cut -d_ -f1)

    local start end duration_ms
    start=$(date +%s%N)

    if ! "$HARNESS_DIR/run_all.sh" --scenario "$scenario_num" >/dev/null 2>&1; then
      info "❌ Scenario $scenario_num failed"
      continue
    fi

    end=$(date +%s%N)
    duration_ms=$(( (end - start) / 1000000 ))
    total_ms=$((total_ms + duration_ms))
    timings+=("$(printf '  %02d: %4dms\n' "$scenario_num" "$duration_ms")")
  done

  info ""
  info "═══════════════════════════════════════════"
  info "⏱️  Timing Report:"
  printf '%s\n' "${timings[@]}"
  info "─────────────────────────────────────────"
  info "Total: $(printf '%dms' "$total_ms")"
  info ""
}

main
