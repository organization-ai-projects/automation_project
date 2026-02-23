#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

cd "$ROOT_DIR"

echo "[1/4] Inventory + script integrity"
bash scripts/automation/check_script_integrity.sh >/tmp/script_integrity.out
cat /tmp/script_integrity.out

echo "[2/4] Direct issue creation contract (dry-run)"
bash scripts/versioning/file_versioning/github/create_direct_issue.sh \
  --title "fix(shell): regression direct issue contract" \
  --context "Regression context" \
  --problem "Regression problem" \
  --acceptance "Regression criterion" \
  --parent none \
  --dry-run >/tmp/direct_issue_dry_run.out
cat /tmp/direct_issue_dry_run.out | sed -n '1,40p'

echo "[3/4] Closure neutralizer regression suite"
bash scripts/versioning/file_versioning/github/tests/neutralize_closure_refs_regression.sh

echo "[4/4] done-in-dev status regression suite"
bash scripts/versioning/file_versioning/github/tests/issue_done_in_dev_status_regression.sh

echo "All critical shell workflow regressions passed."
