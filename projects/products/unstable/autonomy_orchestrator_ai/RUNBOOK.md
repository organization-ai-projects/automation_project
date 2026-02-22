# Autonomy Orchestrator Operator Runbook

This runbook defines how to run, supervise, and troubleshoot the autonomy orchestration pipeline in deterministic mode.

## Execution Modes

- Local dry-run mode: deterministic, no platform side effects.
- CI-like mode: binary-to-binary orchestration with strict gate checks and persisted artifacts.
- Resume mode: restart from checkpoint without re-running completed stages.

## Required Artifacts

Each run must persist:

- `orchestrator_run_report.json`
- `orchestrator_checkpoint.json`

The run report is the primary supervision artifact for:

- terminal state
- gate decisions
- blocked reason codes
- stage execution outcomes

## Governance Contract

### Gate Rules (Fail-Closed)

The orchestrator evaluates three mandatory gates in deterministic order:

- `policy`
- `ci`
- `review`

Accepted pass states:

- `policy=allow`
- `ci=success`
- `review=approved`

Any missing or non-pass state blocks closure (`terminal_state=blocked`).

### Blocking Reason Codes

Deterministic codes used for blocked decisions:

- `GATE_POLICY_DENIED_OR_UNKNOWN`
- `GATE_CI_NOT_SUCCESS`
- `GATE_REVIEW_NOT_APPROVED`

### Approval Requirements

The orchestrator delegates action execution to product binaries. Approval policy for high-risk actions is enforced in downstream executors (not bypassed by orchestrator):

- `autonomous_dev_ai` high-risk pathways require explicit approval token semantics and policy/authz gates.

### Closure Criteria

A run is closure-ready only when:

- `terminal_state=done`
- all gate decisions are `passed=true`
- `blocked_reason_codes` is empty
- required artifacts are present and valid

## Failure Taxonomy and Remediation

### Terminal State: `blocked`

Typical causes:

- policy denied/unknown
- CI missing/pending/failing
- review missing/changes requested

Remediation:

- inspect `gate_decisions` and `blocked_reason_codes` in `orchestrator_run_report.json`
- provide explicit pass gate signals only when real external checks justify it
- rerun with `--resume` after corrective action

### Terminal State: `failed`

Typical causes:

- binary spawn error
- non-zero exit code
- missing expected artifact
- malformed expected JSON artifact
- checkpoint persistence failure

Remediation:

- inspect `stage_executions[].error`
- fix binary path, args, env, or artifact contract
- rerun with `--resume`

### Terminal State: `timeout`

Typical causes:

- invoked binary exceeded `--timeout-ms`

Remediation:

- increase timeout if justified
- reduce command workload or split stage responsibilities
- rerun with `--resume`

## Quickstart: Deterministic Dry-Run

```bash
cargo run -p autonomy_orchestrator_ai -- ./out \
  --policy-status allow \
  --ci-status success \
  --review-status approved \
  --manager-bin /bin/sh \
  --manager-arg -c \
  --manager-arg "exit 0" \
  --executor-bin /bin/sh \
  --executor-arg -c \
  --executor-arg "exit 0"
```

## Quickstart: CI-Like Orchestration

```bash
cargo run -p autonomy_orchestrator_ai -- ./out \
  --policy-status allow \
  --ci-status success \
  --review-status approved \
  --manager-bin ./target/release/auto_manager_ai \
  --manager-arg . \
  --manager-arg ./out/manager \
  --manager-expected-artifact ./out/manager/action_plan.json \
  --manager-expected-artifact ./out/manager/run_report.json \
  --executor-bin ./target/debug/autonomous_dev_ai \
  --executor-env AUTONOMOUS_NON_INTERACTIVE_PROFILE=orchestrator_v1 \
  --executor-env AUTONOMOUS_REQUIRE_PR_NUMBER=true \
  --executor-env AUTONOMOUS_FETCH_PR_CI_STATUS_FROM_GH=true \
  --executor-env AUTONOMOUS_FETCH_PR_CI_STATUS_REQUIRED=true \
  --executor-env AUTONOMOUS_REVIEW_REQUIRED=true \
  --executor-env AUTONOMOUS_REQUIRE_ISSUE_COMPLIANCE=true
```

## Resume Operation

```bash
cargo run -p autonomy_orchestrator_ai -- ./out --resume
```

Resume preserves idempotence using stage checkpoint semantics. Completed stages are skipped and tracked as skipped stage executions.
