#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
CRATE_DIR="$ROOT_DIR/projects/products/unstable/autonomous_dev_ai"
FIXTURE_DIR="$CRATE_DIR/fixtures/ci_like_repo"
OUTPUT_DIR="${1:-$CRATE_DIR/artifacts/ci_like_fixture}"

mkdir -p "$OUTPUT_DIR"

AUDIT_LOG="$OUTPUT_DIR/agent_audit.log"
CONFIG_BASE="$OUTPUT_DIR/agent_config"
CONFIG_RON="${CONFIG_BASE}.ron"
RUN_REPLAY_JSON="$OUTPUT_DIR/agent_run_replay.json"
RUN_REPLAY_TXT="$OUTPUT_DIR/agent_run_replay.txt"
RUN_REPORT_JSON="$OUTPUT_DIR/agent_run_report.json"
CHECKPOINT_PATH="$OUTPUT_DIR/agent_checkpoint.json"
PR_DESCRIPTION_OUT="$OUTPUT_DIR/pr_description.md"

rm -f "$AUDIT_LOG" "$RUN_REPLAY_JSON" "$RUN_REPLAY_TXT" "$RUN_REPORT_JSON" "$CHECKPOINT_PATH" "$PR_DESCRIPTION_OUT" "$CONFIG_RON" "${CONFIG_BASE}.bin"

cat > "$CONFIG_RON" <<'RON'
(
    agent_name: "autonomous_dev_ai",
    execution_mode: "ci_bound",
    neural: (
        enabled: true,
        prefer_gpu: false,
        cpu_fallback: true,
        models: {
            "intent": "intent_v1.bin",
            "codegen": "codegen_v2.bin",
            "review": "review_v1.bin",
        },
    ),
    symbolic: (
        strict_validation: true,
        deterministic: true,
    ),
    objectives: [
        (
            name: "task_completion",
            weight: 1.0,
            hard: true,
            threshold: Some(0.5),
        ),
        (
            name: "policy_safety",
            weight: 1.0,
            hard: true,
            threshold: Some(0.0),
        ),
        (
            name: "tests_pass",
            weight: 0.9,
            hard: true,
            threshold: Some(0.7),
        ),
        (
            name: "minimal_diff",
            weight: 0.6,
            hard: false,
            threshold: None,
        ),
        (
            name: "time_budget",
            weight: 0.4,
            hard: false,
            threshold: None,
        ),
    ],
    max_iterations: 8,
    timeout_seconds: Some(600),
)
RON

pushd "$ROOT_DIR" >/dev/null
cargo build -p autonomous_dev_ai --bin autonomous_dev_ai
BIN_PATH="$ROOT_DIR/target/debug/autonomous_dev_ai"
popd >/dev/null

if [[ ! -x "$BIN_PATH" ]]; then
    echo "Binary not found after build: $BIN_PATH" >&2
    exit 1
fi

GOAL="Validate CI-like autonomous flow for issue #649 with tests"
ISSUE_TITLE="feat(autonomous_dev_ai): ci-like fixture validation"
ISSUE_BODY=$'Context\nFixture-based CI-like scenario for autonomous lifecycle validation.\n\nHierarchy\nParent: none'
REVIEW_COMMENTS_JSON='[{"reviewer":"ci-bot","body":"Looks good for fixture scenario","resolved":true}]'

pushd "$FIXTURE_DIR" >/dev/null
AUTONOMOUS_RUN_REPLAY_PATH="$RUN_REPLAY_JSON" \
AUTONOMOUS_RUN_REPLAY_TEXT_PATH="$RUN_REPLAY_TXT" \
AUTONOMOUS_RUN_REPORT_PATH="$RUN_REPORT_JSON" \
AUTONOMOUS_CHECKPOINT_PATH="$CHECKPOINT_PATH" \
AUTONOMOUS_REPO_ROOT="$FIXTURE_DIR" \
AUTONOMOUS_REQUIRE_EXPLORED_FILES=true \
AUTONOMOUS_ISSUE_TITLE="$ISSUE_TITLE" \
AUTONOMOUS_ISSUE_BODY="$ISSUE_BODY" \
AUTONOMOUS_REVIEW_REQUIRED=true \
AUTONOMOUS_REVIEW_COMMENTS_JSON="$REVIEW_COMMENTS_JSON" \
AUTONOMOUS_PR_NUMBER=649 \
AUTONOMOUS_PR_DESCRIPTION_OUTPUT="$PR_DESCRIPTION_OUT" \
"$BIN_PATH" "$GOAL" "$CONFIG_BASE" "$AUDIT_LOG"
popd >/dev/null

if [[ ! -f "$RUN_REPORT_JSON" ]]; then
    echo "Missing run report artifact: $RUN_REPORT_JSON" >&2
    exit 1
fi

if ! rg -q '"final_state"\s*:\s*"Done"' "$RUN_REPORT_JSON"; then
    echo "Run report does not indicate final_state Done" >&2
    exit 1
fi

if ! rg -q '"last_review_outcome"\s*:\s*"Approved"' "$RUN_REPORT_JSON"; then
    echo "Run report does not indicate Approved review outcome" >&2
    exit 1
fi

echo "CI-like fixture scenario completed successfully."
echo "Artifacts:"
echo "  - $RUN_REPORT_JSON"
echo "  - $RUN_REPLAY_JSON"
echo "  - $RUN_REPLAY_TXT"
echo "  - $AUDIT_LOG"
