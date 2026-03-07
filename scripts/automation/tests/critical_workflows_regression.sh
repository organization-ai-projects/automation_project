#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

cd "$ROOT_DIR"

echo "[1/10] Inventory + script integrity"
bash scripts/automation/check_script_integrity.sh >/tmp/script_integrity.out
cat /tmp/script_integrity.out

echo "[2/10] Direct issue creation contract (dry-run)"
bash scripts/versioning/file_versioning/github/create_direct_issue.sh \
  --title "fix(shell): regression direct issue contract" \
  --context "Regression context" \
  --problem "Regression problem" \
  --acceptance "Regression criterion" \
  --parent none \
  --dry-run >/tmp/direct_issue_dry_run.out
sed -n '1,40p' /tmp/direct_issue_dry_run.out

echo "[3/10] Issue manager regression suite"
bash scripts/versioning/file_versioning/github/tests/manager_issues_regression.sh

echo "[4/10] create_pr internal guard regression suite"
bash scripts/versioning/file_versioning/orchestrators/read/tests/create_pr_internal_guard_regression.sh

echo "[5/10] Closure neutralizer regression suite"
bash scripts/versioning/file_versioning/github/tests/neutralize_closure_refs_regression.sh

echo "[6/10] Directive conflict guard regression suite"
bash scripts/versioning/file_versioning/github/tests/pr_directive_conflict_guard_regression.sh

echo "[7/10] done-in-dev status regression suite"
bash scripts/versioning/file_versioning/github/tests/issue_done_in_dev_status_regression.sh

echo "[8/10] Hook convention guardrails regression suite"
bash scripts/automation/git_hooks/tests/convention_guardrails_regression.sh

echo "[9/10] auto-add closes on dev PR regression suite"
bash scripts/versioning/file_versioning/github/tests/auto_add_closes_on_dev_pr_regression.sh

echo "[10/10] Enforcer shell contract regression suite"
bash scripts/versioning/file_versioning/github/tests/enforcer_shell_contract_regression.sh

echo "All critical shell workflow regressions passed."
