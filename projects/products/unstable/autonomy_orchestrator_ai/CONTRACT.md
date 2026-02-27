# Manager Executor Reviewer Contract (v1)

This document defines the orchestration contract used by `autonomy_orchestrator_ai`.

## Stage Boundaries

- `planning`: manager binary invocation and optional `planning.repo_context` generation.
- `execution`: executor binary invocation, with bounded retries.
- `validation`: reviewer binary invocation or native typed validation invocations.
- `closure`: gate evaluation and optional delivery lifecycle.

## Invocation Inputs

For each delegated role (`manager`, `executor`, `reviewer`) the orchestrator accepts:

- `--<role>-bin <path>`
- `--<role>-arg <value>` (repeatable)
- `--<role>-env <KEY=VALUE>` (repeatable)
- `--<role>-expected-artifact <path>` (repeatable)

Native validation invocations:

- `--validation-bin <path>` (repeatable)
- `--validation-arg <value>` (repeatable, bound to latest `--validation-bin`)
- `--validation-env <KEY=VALUE>` (repeatable, bound to latest `--validation-bin`)
- `--validation-from-planning-context`

Delivery lifecycle (feature-flagged):

- `--delivery-enabled`
- `--delivery-dry-run`
- `--delivery-branch <name>`
- `--delivery-commit-message <message>`
- `--delivery-pr-enabled`
- `--delivery-pr-number <number>` (when set, performs PR update instead of create)
- `--delivery-pr-base <branch>`
- `--delivery-pr-title <title>`
- `--delivery-pr-body <body>`

## Required Outputs

- `orchestrator_run_report.json`
- `orchestrator_checkpoint.json`

The run report contains:

- stage transitions
- stage execution records
- terminal state
- gate decisions
- blocked reason codes
- reviewer next-step plan propagation

Planning context artifact (`--planning-context-artifact`) may include:

- `planning_feedback` (schema version `1`) extracted from previous outcome artifacts
- sources: prior `orchestrator_run_report.json` and `next_actions.bin`
- fields: terminal state, blocked reason codes, reviewer next steps, recommended actions, validation outcomes
- safeguards: deterministic sort+dedupe, bounded list sizes, bounded text length

## Error Taxonomy

`StageExecutionStatus` values:

- `success`
- `failed`
- `timeout`
- `spawn_failed`
- `artifact_missing`
- `skipped`

Gate reason codes:

- `GATE_POLICY_DENIED_OR_UNKNOWN`
- `GATE_CI_NOT_SUCCESS`
- `GATE_REVIEW_NOT_APPROVED`

## Idempotency Keys

Recorded in `stage_executions[].idempotency_key`:

- stage keys: `stage:<StageName>`
- execution attempts: `stage:Execution:attempt:<n>`
- execution budget exhaustion: `stage:Execution:budget`
- validation invocations: `stage:Validation:command:<n>`
- planning context artifact action: `stage:Planning:repo_context`
- delivery lifecycle steps: `stage:Closure:delivery:<n>`

## Resume Safety

- checkpoint marks completed stages and terminal state
- `--resume` skips completed stages idempotently
- remediation loop resets only `execution` and `validation` checkpoints
- closure is executed once per run path before terminal mark
