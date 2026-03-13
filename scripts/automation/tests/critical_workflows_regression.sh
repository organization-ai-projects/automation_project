#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

cd "$ROOT_DIR"

echo "[0/8] Build versioning_automation binary"
cargo build -p versioning_automation >/tmp/versioning_automation_build.out
tail -n 5 /tmp/versioning_automation_build.out || true

echo "[1/8] Inventory + script integrity"
bash scripts/automation/check_script_integrity.sh >/tmp/script_integrity.out
cat /tmp/script_integrity.out

echo "[2/8] Direct issue creation contract (dry-run)"
target/debug/versioning_automation issue create \
  --title "fix(shell): regression direct issue contract" \
  --context "Regression context" \
  --problem "Regression problem" \
  --acceptance "Regression criterion" \
  --assignee "octocat" \
  --related-issue "#12" \
  --related-pr "#34" \
  --parent none \
  --dry-run >/tmp/direct_issue_dry_run.out
sed -n '1,40p' /tmp/direct_issue_dry_run.out

echo "[3/8] Rust unit test suite (versioning_automation)"
cargo test -q -p versioning_automation

echo "[4/8] Closure neutralizer regression suite"
cargo test -q -p versioning_automation pr::tests::open_referencing_issue

echo "[5/8] Directive conflict guard regression suite"
cargo test -q -p versioning_automation pr::tests::directive_conflict_guard

echo "[6/8] done-in-dev status regression suite"
cargo test -q -p versioning_automation issues::tests::sync_project_status

echo "[7/8] Hook convention guardrails regression suite"
bash scripts/automation/git_hooks/tests/convention_guardrails_regression.sh

echo "[8/8] Enforcer shell contract regression suite"
bash scripts/automation/tests/enforcer_shell_contract_regression.sh

echo "All critical workflow regressions passed."
