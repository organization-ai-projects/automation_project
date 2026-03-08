#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"

if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: cargo is required." >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "Error: jq is required." >&2
  exit 1
fi

cd "$ROOT_DIR"

set +e
report_json="$(cargo run -q -p repo_contract_enforcer_ui -- check --root . --mode strict --json)"
check_status=$?
set -e

if [[ $check_status -ne 0 && $check_status -ne 3 ]]; then
  echo "Error: repo_contract_enforcer_ui check failed with unexpected status: $check_status" >&2
  exit "$check_status"
fi

shell_violation_count="$(
  jq '[.violations[] | select(.violation_code | startswith("STRUCT_SHELL_"))] | length' <<<"$report_json"
)"

if [[ "$shell_violation_count" != "0" ]]; then
  echo "Shell contract violations detected by repo_contract_enforcer (strict mode)." >&2
  jq -r '
    .violations[]
    | select(.violation_code | startswith("STRUCT_SHELL_"))
    | "- \(.violation_code) :: \(.path)\(if .line then ":" + (.line|tostring) else "" end) :: \(.message)"
  ' <<<"$report_json" >&2
  exit 1
fi

echo "Enforcer shell contract regression: PASS"
