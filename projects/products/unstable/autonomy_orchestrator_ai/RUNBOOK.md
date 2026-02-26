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
- inspect `reviewer_next_steps` in `orchestrator_run_report.json` when reviewer is configured
- fix binary path, args, env, or artifact contract
- rerun with `--resume`

### Reviewer-Driven Remediation Loop

When `--reviewer-remediation-max-cycles` is greater than `0`, a failed validation with reviewer
`next_step_plan` can trigger bounded reruns of `execution -> validation`.

Operational notes:

- loop is bounded by `--reviewer-remediation-max-cycles`
- executor receives reviewer feedback through environment variables:
  - `ORCHESTRATOR_REMEDIATION_CYCLE`
  - `ORCHESTRATOR_REMEDIATION_STEPS`
- if remediation budget is exhausted, run remains `failed` (fail-closed)

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
  --manager-bin /usr/bin/true \
  --executor-bin /usr/bin/true
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

## Quickstart: Linked Three-AI Delegation

Use the dedicated helper to wire:

- `auto_manager_ai` as planning manager
- `autonomous_dev_ai` as execution agent
- `autonomy_reviewer_ai` as validation reviewer

```bash
cargo run -p autonomy_orchestrator_ai -- linked-stack \
  ./out/orchestrator_linked_ai \
  . \
  "Investigate and propose safe fixes for unstable test failures"
```

By default, this helper:

- builds all four binaries (orchestrator + 3 delegated AIs)
- enables deterministic fallback for `auto_manager_ai`
- runs `autonomous_dev_ai` in `--symbolic-only` mode for safer local validation
- enforces expected output artifacts from delegated binaries
- sets bounded execution retries via `--execution-max-iterations` (default: `2`)

## Quickstart: Native Validation Invocations

When no reviewer binary is available, validation can be delegated to native binary invocations:

```bash
cargo run -p autonomy_orchestrator_ai -- ./out \
  --policy-status allow \
  --ci-status success \
  --review-status approved \
  --validation-bin cargo --validation-arg check --validation-arg -p --validation-arg autonomy_orchestrator_ai \
  --validation-bin cargo --validation-arg test --validation-arg -p --validation-arg autonomy_orchestrator_ai
```

You can also source validation invocations detected during planning:

```bash
cargo run -p autonomy_orchestrator_ai -- ./out \
  --repo-root . \
  --planning-context-artifact ./out/planning/repo_context.json \
  --validation-from-planning-context \
  --policy-status allow \
  --ci-status success \
  --review-status approved
```

## Quickstart: Delivery Lifecycle (Feature-Flagged)

Use delivery flags only after gates are green and outputs are verified.

Dry-run (recommended first):

```bash
cargo run -p autonomy_orchestrator_ai -- ./out \
  --policy-status allow \
  --ci-status success \
  --review-status approved \
  --delivery-enabled \
  --delivery-dry-run \
  --delivery-branch feat/example-delivery \
  --delivery-commit-message "feat: scoped fix" \
  --delivery-pr-enabled \
  --delivery-pr-base dev \
  --delivery-pr-title "Scoped fix" \
  --delivery-pr-body "Automated delivery dry-run"
```

This records delivery actions in `stage_executions` without side effects.

## Resume Operation

```bash
cargo run -p autonomy_orchestrator_ai -- ./out --resume
```

Resume preserves idempotence using stage checkpoint semantics. Completed stages are skipped and tracked as skipped stage executions.
