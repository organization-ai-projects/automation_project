#!/usr/bin/env bash
# Run scenarios stopping at first failure
set -euo pipefail

HARNESS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

main() {
  for f in "$HARNESS_DIR/scenarios/"*.env; do
    if ! "$HARNESS_DIR/run_all.sh" --scenario "$(basename "$f" .env | sed 's/^0*//' | cut -d_ -f1)"; then
      exit 1
    fi
  done

  echo ""
  echo "🎉 All scenarios passed (fail-fast)"
}

main
