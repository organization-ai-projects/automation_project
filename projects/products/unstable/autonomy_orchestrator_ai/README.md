# Autonomy Orchestrator AI (V0 - Binary Orchestration)

`autonomy_orchestrator_ai` is an unstable product that orchestrates autonomous workflows through product binary invocation boundaries.

For bootstrap V0, this crate provides:

- explicit stage state machine (`planning`, `policy_ingestion`, `execution`, `validation`, `closure`)
- explicit terminal states (`done`, `blocked`, `failed`, `timeout`)
- deterministic transition recording with run identifier and timestamp
- JSON run report artifact for audit/replay style supervision
- typed stage execution records for binary invocations (exit code, duration, timeout, artifact checks)
- checkpoint persistence with resume semantics to avoid re-running completed stages
- optional native planning context artifact generation (repo topology + detected validation commands)

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
  --repo-root . \
  --planning-context-artifact ./out/planning/repo_context.json \
  --timeout-ms 30000 \
  --execution-max-iterations 3 \
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
  --executor-arg ./agent_audit.log \
  --reviewer-bin ./target/release/autonomy_reviewer_ai \
  --reviewer-env AUTONOMOUS_REVIEW_STRICT=true \
  --reviewer-arg ./out/review \
  --reviewer-expected-artifact ./out/review/review_report.json
```

If a configured binary fails to spawn, exits non-zero, times out, misses expected artifacts, or produces malformed expected JSON artifacts, the orchestrator fails closed with terminal state `failed` or `timeout`.
For `execution`, you can enable bounded retries with `--execution-max-iterations <N>` (default `1`), so the stage cannot loop forever. Each attempt receives environment variable `ORCHESTRATOR_EXECUTION_ATTEMPT=<n>`.
If a reviewer fails and emits `next_step_plan`, you can enable bounded remediation loops with `--reviewer-remediation-max-cycles <N>` (default `0`).
During remediation reruns, executor receives:

- `ORCHESTRATOR_REMEDIATION_CYCLE=<n>`
- `ORCHESTRATOR_REMEDIATION_STEPS=<joined reviewer next steps>`
For `validation`, you can either:

- provide a dedicated reviewer binary (`--reviewer-bin ...`)
- run native validation invocations with `--validation-bin ...` + `--validation-arg ...` (repeatable)
- load native validation invocations detected during planning with `--validation-from-planning-context` (requires `--planning-context-artifact`)

Supported invocation boundaries are currently:

- `manager` at stage `planning`
- `executor` at stage `execution`
- `reviewer` at stage `validation`
- native `planning.repo_context` action in stage `planning` when `--planning-context-artifact` is set
- native `validation` invocation execution with `--validation-bin` / `--validation-from-planning-context`

When `--resume` is used, stages already marked completed in checkpoint are skipped idempotently. Stage idempotency keys are tracked as `stage:<StageName>` in execution records.
Resume behavior is covered by binary regression tests in `tests/binary_resume_tests.rs`.

## Output

- `orchestrator_run_report.json`
- `orchestrator_checkpoint.json` (default path: `<output_dir>/orchestrator_checkpoint.json`)

This report includes machine-readable stage transitions and terminal outcome.
It also includes `stage_executions` records for every configured binary execution, plus gate decisions, `blocked_reason_codes`, and `reviewer_next_steps` when a reviewer emits a `next_step_plan`.

## E2E Matrix

Deterministic end-to-end regression matrix:

```bash
cargo test -p autonomy_orchestrator_ai --test binary_e2e_matrix_tests
```

Local helper:

```bash
cargo test -p autonomy_orchestrator_ai --test binary_e2e_matrix_tests
```

Linked manager+executor stack helper (Rust runner):

```bash
cargo run -p autonomy_orchestrator_ai -- linked-stack [out_dir] [repo_root] [goal]
```

This helper wires:

- `auto_manager_ai` as `manager` (deterministic fallback mode)
- `autonomous_dev_ai` as `executor` (`--symbolic-only`), with bounded execution retries managed by orchestrator
- `autonomy_reviewer_ai` as `reviewer` in `validation`

Scenario documentation:

- `projects/products/unstable/autonomy_orchestrator_ai/scenarios/e2e_matrix/README.md`

Operator documentation:

- `projects/products/unstable/autonomy_orchestrator_ai/RUNBOOK.md`

## Delivery Notes

- Issue `#676` (orchestrator bootstrap state machine) is implemented in this crate.
- Issue `#675` (typed binary invocation contract and machine-readable execution outcomes) is implemented in this crate.
- Issue `#680` (centralized fail-closed policy/CI/review gates with deterministic blocking reason codes) is implemented in this crate.
