# Autonomous Developer AI

`autonomous_dev_ai` is an unstable product focused on a safe neuro-symbolic autonomous developer workflow.

This crate is not fully production-complete yet. It contains a functional lifecycle core, policy gates, audit/replay primitives, and controlled tool execution, but parts of end-to-end autonomy remain partial.

## Current Status

- Stage: unstable / iterative hardening
- Issue track: `#647` (baseline completion) still in progress
- Design goal: autonomous-by-default behavior with policy-first safety

## What Is Implemented

- Lifecycle state machine with retry/recovery paths and terminal states
- Policy validation + authz gate before tool execution
- Risk gating (low/medium/high) with explicit approval token for high-risk actions
- Circuit breaker, rollback boundary tracking, and checkpoint persistence
- Run replay and audit logging for post-run analysis
- Structured persistence with transactional state/index artifacts
- PR flow orchestration primitives (metadata, compliance/readiness states, review loop)

## What Is Still Partial

- Full real GitHub integration for PR/review execution (current flow is partially environment-driven)
- End-to-end CI-like autonomous scenario validation with strict closure criteria
- Comprehensive test matrix requested by issue acceptance criteria
- Final documentation/behavior lock for production-grade claims

## Usage

```bash
cargo run -p autonomous_dev_ai -- "Fix the failing tests" ./agent_config ./audit.log
```

## Safety Model

- No action should bypass symbolic policy and authz checks
- Tool execution is allowlist-based and policy-constrained
- High-risk actions require explicit approval token
- Failures are recorded for replay and incident analysis

## Tooling Notes

Implemented tool wrappers include:

- `read_file`
- `run_tests`
- `git_commit`
- `generate_pr_description`

`run_tests` executes real commands (with timeout control), and `git_commit` is restricted by allowlist plus forbidden action checks.

## Development Notes

- Keep claims in this file strictly aligned with current code behavior.
- For detailed implementation status and remaining gaps, see `IMPLEMENTATION.md`.
