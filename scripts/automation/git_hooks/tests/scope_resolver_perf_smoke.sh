#!/usr/bin/env bash
set -euo pipefail

# Basic perf smoke test for scope resolver.
# Goal: detect obvious regressions (not micro-benchmarking).

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../.." && pwd)"

# shellcheck source=scripts/common_lib/automation/scope_resolver.sh
source "$ROOT_DIR/scripts/common_lib/automation/scope_resolver.sh"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

FILE_LIST="$TMP_DIR/files.txt"
for i in $(seq 1 1200); do
  printf 'projects/libraries/security/src/file_%04d.rs\n' "$i" >> "$FILE_LIST"
done

FILES="$(cat "$FILE_LIST")"
START_NS="$(date +%s%N)"
SCOPES="$(collect_scopes_from_files "$FILES")"
END_NS="$(date +%s%N)"

ELAPSED_MS=$(( (END_NS - START_NS) / 1000000 ))
THRESHOLD_MS="${SCOPE_RESOLVER_PERF_THRESHOLD_MS:-15000}"

echo "scope_resolver_perf_smoke: ${ELAPSED_MS}ms (threshold: ${THRESHOLD_MS}ms)"
echo "resolved scopes:"
printf '%s\n' "$SCOPES" | sed 's/^/  - /'

if [[ "$ELAPSED_MS" -gt "$THRESHOLD_MS" ]]; then
  echo "FAIL: scope resolver perf smoke exceeded threshold" >&2
  exit 1
fi

echo "PASS"
