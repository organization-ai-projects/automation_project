#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"
cd "$ROOT_DIR"

OUT_DIR="${1:-$ROOT_DIR/out/orchestrator_linked_ai}"
REPO_ROOT="${2:-$ROOT_DIR}"
GOAL="${3:-Inspect the repository and produce a safe implementation plan}"
EXECUTION_MAX_ITERATIONS="${EXECUTION_MAX_ITERATIONS:-2}"
TIMEOUT_MS="${TIMEOUT_MS:-30000}"
EXECUTOR_AGENT_MAX_ITERATIONS="${EXECUTOR_AGENT_MAX_ITERATIONS:-20}"
ORCH_VALIDATION_FROM_PLANNING_CONTEXT="${ORCH_VALIDATION_FROM_PLANNING_CONTEXT:-false}"
REVIEWER_STRICT="${REVIEWER_STRICT:-true}"
REVIEWER_VALIDATION_COMMANDS="${REVIEWER_VALIDATION_COMMANDS:-cargo check -p autonomy_orchestrator_ai}"

MANAGER_OUT_DIR="$OUT_DIR/manager"
EXECUTOR_OUT_DIR="$OUT_DIR/executor"
REVIEWER_OUT_DIR="$OUT_DIR/reviewer"
PLANNING_CONTEXT_ARTIFACT="$OUT_DIR/planning/repo_context.json"
EXECUTOR_CONFIG_BASE="$EXECUTOR_OUT_DIR/agent_config"
EXECUTOR_AUDIT_LOG="$EXECUTOR_OUT_DIR/audit.log"
EXECUTOR_RUN_REPORT="$EXECUTOR_OUT_DIR/agent_run_report.json"
EXECUTOR_RUN_REPLAY_JSON="$EXECUTOR_OUT_DIR/agent_run_replay.json"
EXECUTOR_RUN_REPLAY_TEXT="$EXECUTOR_OUT_DIR/agent_run_replay.txt"
EXECUTOR_CHECKPOINT="$EXECUTOR_OUT_DIR/agent_checkpoint.json"

REVIEWER_REPORT="$REVIEWER_OUT_DIR/review_report.json"

mkdir -p "$OUT_DIR" "$MANAGER_OUT_DIR" "$EXECUTOR_OUT_DIR" "$REVIEWER_OUT_DIR" "$(dirname "$PLANNING_CONTEXT_ARTIFACT")"

cat > "${EXECUTOR_CONFIG_BASE}.ron" <<EOF
(
  max_iterations: ${EXECUTOR_AGENT_MAX_ITERATIONS},
)
EOF

echo "[orchestrator] Building binaries (autonomy_orchestrator_ai, auto_manager_ai, autonomous_dev_ai, autonomy_reviewer_ai)..."
cargo build -p autonomy_orchestrator_ai -p auto_manager_ai -p autonomous_dev_ai -p autonomy_reviewer_ai

echo "[orchestrator] Running linked AI stack"
echo "  out_dir=$OUT_DIR"
echo "  repo_root=$REPO_ROOT"
echo "  goal=$GOAL"
echo "  execution_max_iterations=$EXECUTION_MAX_ITERATIONS"
echo "  executor_agent_max_iterations=$EXECUTOR_AGENT_MAX_ITERATIONS"
echo "  validation_from_planning_context=$ORCH_VALIDATION_FROM_PLANNING_CONTEXT"
echo "  reviewer_strict=$REVIEWER_STRICT"
echo "  timeout_ms=$TIMEOUT_MS"
echo

set +e
ORCH_ARGS=(
  "$OUT_DIR"
  --repo-root "$REPO_ROOT"
  --planning-context-artifact "$PLANNING_CONTEXT_ARTIFACT"
  --policy-status allow
  --ci-status success
  --review-status approved
  --timeout-ms "$TIMEOUT_MS"
  --execution-max-iterations "$EXECUTION_MAX_ITERATIONS"
  --manager-bin "$ROOT_DIR/target/debug/auto_manager_ai"
  --manager-env AUTO_MANAGER_ENGINE_AVAILABLE=true
  --manager-env AUTO_MANAGER_RUN_MODE=deterministic_fallback
  --manager-arg "$REPO_ROOT"
  --manager-arg "$MANAGER_OUT_DIR"
  --manager-expected-artifact "$MANAGER_OUT_DIR/action_plan.json"
  --manager-expected-artifact "$MANAGER_OUT_DIR/run_report.json"
  --executor-bin "$ROOT_DIR/target/debug/autonomous_dev_ai"
  --executor-env AUTONOMOUS_REPO_ROOT="$REPO_ROOT"
  --executor-env AUTONOMOUS_RUN_REPORT_PATH="$EXECUTOR_RUN_REPORT"
  --executor-env AUTONOMOUS_RUN_REPLAY_PATH="$EXECUTOR_RUN_REPLAY_JSON"
  --executor-env AUTONOMOUS_RUN_REPLAY_TEXT_PATH="$EXECUTOR_RUN_REPLAY_TEXT"
  --executor-env AUTONOMOUS_CHECKPOINT_PATH="$EXECUTOR_CHECKPOINT"
  --executor-arg --symbolic-only
  --executor-arg "$GOAL"
  --executor-arg "$EXECUTOR_CONFIG_BASE"
  --executor-arg "$EXECUTOR_AUDIT_LOG"
  --executor-expected-artifact "$EXECUTOR_AUDIT_LOG"
  --executor-expected-artifact "$EXECUTOR_RUN_REPORT"
  --executor-expected-artifact "$EXECUTOR_RUN_REPLAY_JSON"
  --executor-expected-artifact "$EXECUTOR_RUN_REPLAY_TEXT"
  --reviewer-bin "$ROOT_DIR/target/debug/autonomy_reviewer_ai"
  --reviewer-arg "$REPO_ROOT"
  --reviewer-arg "$REVIEWER_OUT_DIR"
  --reviewer-arg --manager-action-plan
  --reviewer-arg "$MANAGER_OUT_DIR/action_plan.json"
  --reviewer-arg --manager-run-report
  --reviewer-arg "$MANAGER_OUT_DIR/run_report.json"
  --reviewer-arg --executor-run-report
  --reviewer-arg "$EXECUTOR_RUN_REPORT"
  --reviewer-arg --executor-audit-log
  --reviewer-arg "$EXECUTOR_AUDIT_LOG"
  --reviewer-expected-artifact "$REVIEWER_REPORT"
)

if [[ "${REVIEWER_STRICT,,}" == "true" ]]; then
  ORCH_ARGS+=(--reviewer-arg --strict)
fi

IFS=';;' read -r -a REVIEWER_CMD_ARRAY <<< "$REVIEWER_VALIDATION_COMMANDS"
for cmd in "${REVIEWER_CMD_ARRAY[@]}"; do
  trimmed="$(echo "$cmd" | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')"
  if [[ -n "$trimmed" ]]; then
    ORCH_ARGS+=(--reviewer-arg --validation-command --reviewer-arg "$trimmed")
  fi
done

if [[ "${ORCH_VALIDATION_FROM_PLANNING_CONTEXT,,}" == "true" ]]; then
  ORCH_ARGS+=(--validation-from-planning-context)
fi

cargo run -q -p autonomy_orchestrator_ai -- "${ORCH_ARGS[@]}"
ORCHESTRATOR_RC=$?
set -e

echo
echo "[orchestrator] Completed (exit_code=$ORCHESTRATOR_RC)."
echo "  run_report=$OUT_DIR/orchestrator_run_report.json"
echo "  checkpoint=$OUT_DIR/orchestrator_checkpoint.json"
echo "  planning_context=$PLANNING_CONTEXT_ARTIFACT"
echo "  manager_action_plan=$MANAGER_OUT_DIR/action_plan.json"
echo "  manager_run_report=$MANAGER_OUT_DIR/run_report.json"
echo "  executor_audit_log=$EXECUTOR_AUDIT_LOG"
echo "  executor_run_report=$EXECUTOR_RUN_REPORT"
echo "  executor_run_replay_json=$EXECUTOR_RUN_REPLAY_JSON"
echo "  executor_run_replay_text=$EXECUTOR_RUN_REPLAY_TEXT"
echo "  executor_checkpoint=$EXECUTOR_CHECKPOINT"
echo "  reviewer_report=$REVIEWER_REPORT"

exit "$ORCHESTRATOR_RC"
