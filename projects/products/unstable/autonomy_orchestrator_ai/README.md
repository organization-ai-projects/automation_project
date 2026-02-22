# Autonomy Orchestrator AI (V0 - Binary Orchestration)

`autonomy_orchestrator_ai` is an unstable product that orchestrates autonomous workflows through product binary invocation boundaries.

For bootstrap V0, this crate provides:

- explicit stage state machine (`planning`, `policy_ingestion`, `execution`, `validation`, `closure`)
- explicit terminal states (`done`, `blocked`, `failed`, `timeout`)
- deterministic transition recording with run identifier and timestamp
- JSON run report artifact for audit/replay style supervision
- typed stage execution records for binary invocations (exit code, duration, timeout, artifact checks)
- checkpoint persistence with resume semantics to avoid re-running completed stages

## Usage

```bash
cargo run -p autonomy_orchestrator_ai -- [output_dir]
```

Resume from persisted checkpoint:

```bash
cargo run -p autonomy_orchestrator_ai -- [output_dir] --resume
```

Optional blocked simulation:

```bash
cargo run -p autonomy_orchestrator_ai -- [output_dir] --simulate-blocked
```

Fail-closed gate controls:

```bash
cargo run -p autonomy_orchestrator_ai -- ./out \
  --policy-status allow \
  --ci-status success \
  --review-status approved
```

If gate flags are omitted, default values are fail-closed (`policy=unknown`, `ci=missing`, `review=missing`) and the run blocks with deterministic reason codes.
Current reason codes:

- `GATE_POLICY_DENIED_OR_UNKNOWN`
- `GATE_CI_NOT_SUCCESS`
- `GATE_REVIEW_NOT_APPROVED`

Binary invocation contract:

```bash
cargo run -p autonomy_orchestrator_ai -- ./out \
  --timeout-ms 30000 \
  --manager-bin ./target/release/auto_manager_ai \
  --manager-env AUTONOMOUS_REPO_ROOT=. \
  --manager-arg . \
  --manager-arg ./out/manager \
  --manager-expected-artifact ./out/manager/action_plan.json \
  --manager-expected-artifact ./out/manager/run_report.json \
  --executor-bin ./target/debug/autonomous_dev_ai \
  --executor-env AUTONOMOUS_REQUIRE_PR_NUMBER=true \
  --executor-arg "Fix failing tests for issue #123" \
  --executor-arg ./agent_config \
  --executor-arg ./agent_audit.log
```

If a configured binary fails to spawn, exits non-zero, times out, misses expected artifacts, or produces malformed expected JSON artifacts, the orchestrator fails closed with terminal state `failed` or `timeout`.
When `--resume` is used, stages already marked completed in checkpoint are skipped idempotently. Stage idempotency keys are tracked as `stage:<StageName>` in execution records.
Resume behavior is covered by binary regression tests in `tests/binary_resume_tests.rs`.

## Output

- `orchestrator_run_report.json`
- `orchestrator_checkpoint.json` (default path: `<output_dir>/orchestrator_checkpoint.json`)

This report includes machine-readable stage transitions and terminal outcome.
It also includes `stage_executions` records for every configured binary execution, plus gate decisions and `blocked_reason_codes`.

## Delivery Notes

- Issue `#676` (orchestrator bootstrap state machine) is implemented in this crate.
- Issue `#675` (typed binary invocation contract and machine-readable execution outcomes) is implemented in this crate.
- Issue `#680` (centralized fail-closed policy/CI/review gates with deterministic blocking reason codes) is implemented in this crate.
