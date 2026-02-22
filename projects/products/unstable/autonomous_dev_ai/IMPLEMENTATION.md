# Autonomous Developer AI - Implementation Status

## Scope

This document tracks real implementation status for `projects/products/unstable/autonomous_dev_ai`, with emphasis on issue `#647`.

It intentionally avoids forward-looking claims presented as already completed.

## Current Reality

Implemented today:

- lifecycle core with retries, failure handling, checkpoints, and terminal states
- symbolic policy validation + authz checks before tool execution
- risk gates (low/medium/high) and explicit high-risk approval token path
- tool wrappers for repository read, tests, git operations, and PR description generation
- run replay + audit logs + memory/persistence indices
- PR/review orchestration primitives and merge-readiness evaluation

Still partial:

- full real GitHub PR/review execution path remains incomplete
- acceptance-level integration/regression matrix for `#647` remains pending
- some operational hardening tracks are still in progress (see issues `#651`-`#655`)

## Recent Progress (during #647 hardening)

- `run_tests` no longer returns synthetic success fallback; it now resolves to real command execution path with timeout.
- tool execution now propagates exit code information through lifecycle metadata and run report artifacts.
- `git_commit` wrapper received stricter safety checks (forbidden force/destructive flags).
- tool timeout execution now attempts explicit process termination on timeout and returns clearer diagnostics.
- repository exploration now inspects real filesystem entries (configurable root + bounded entry count), with optional fail-closed mode.
- post-execution tool result contract checks now enforce `success/exit_code/error` consistency.
- lifecycle PR/review flow reduced simulated behavior:
  - no synthetic PR number derived from issue number
  - strict mode can require explicit PR number (`AUTONOMOUS_REQUIRE_PR_NUMBER=true`)
  - review loop no longer consumes budget on empty feedback batches
- review loop can optionally fetch structured feedback from real PR context via `gh pr view` (`AUTONOMOUS_FETCH_REVIEW_FROM_GH=true`) when a PR number is available, with explicit source tracking in run metadata
- strict fail-closed controls added for non-interactive operation:
  - `AUTONOMOUS_REQUIRE_REAL_PR_CREATION=true` enforces a PR actually created by runtime (not only injected number)
  - `AUTONOMOUS_REQUIRE_GH_REVIEW_SOURCE=true` enforces review input from GitHub PR context
- runtime requirements are now validated fail-fast at run start to reject inconsistent settings before state execution
- PR metadata can now optionally ingest CI/check status from GitHub (`AUTONOMOUS_FETCH_PR_CI_STATUS_FROM_GH=true`) with optional fail-closed enforcement (`AUTONOMOUS_FETCH_PR_CI_STATUS_REQUIRED=true`)
- run report now exposes PR provenance (`real_pr_created`, `pr_number_source`, `pr_ci_status`) for stricter auditability
- a reproducible CI-like fixture scenario is available via `scripts/run_ci_like_fixture.sh` and emits run-report/replay/audit artifacts for manual validation
- objective evaluation now enforces SLO gating only when explicitly requested (`AUTONOMOUS_ENFORCE_SLO_DURING_OBJECTIVE_EVAL=true`) to prevent iterative pre-terminal deadlocks
- PR creation path can now fetch issue context (`title/body`) from GitHub (`AUTONOMOUS_FETCH_ISSUE_CONTEXT_FROM_GH=true`) with optional fail-closed mode (`AUTONOMOUS_FETCH_ISSUE_CONTEXT_REQUIRED=true`), and reports context provenance
- strict issue compliance gating is now available during PR stage (`AUTONOMOUS_REQUIRE_ISSUE_COMPLIANCE=true`) to block non-conformant issue metadata from proceeding
- run report now includes failure distribution telemetry (`failure_kind_counts`, `top_failure_kind`) and last recovery hint (`last_failure_recovery_action`) for safer autonomous operations

## Neural Governance Progress (issue #651)

Implemented:

- model rollout now enforces explicit offline and online evaluation gates before serving/promotion (`Pending -> Canary -> Production`)
- rollout thresholds/scores are runtime-configurable (`AUTONOMOUS_NEURAL_OFFLINE_MIN_SCORE`, `AUTONOMOUS_NEURAL_ONLINE_MIN_SCORE`, `AUTONOMOUS_NEURAL_MIN_CONFIDENCE`, `AUTONOMOUS_NEURAL_OFFLINE_SCORE`, `AUTONOMOUS_NEURAL_ONLINE_SCORE`)
- rollout scores can be sourced from a JSON snapshot file (`AUTONOMOUS_NEURAL_EVAL_FILE`) for less synthetic governance inputs, with env fallback
- active governed model is now selectable at runtime (`AUTONOMOUS_NEURAL_MODEL_NAME`) instead of hardcoded `default-neural`
- JSON snapshot input supports single-model object and multi-model payloads (array or `{ "models": [...] }`)
- neural serving path now fails closed to symbolic fallback when a model is not promoted or confidence is below thresholds
- drift detection triggers immediate rollback to `RolledBack` and disables serving for the impacted model
- rollout gate states and rollback reason are traced in run replay for auditability
- unit tests now cover:
  - gate enforcement before canary/production promotion
  - fallback on low confidence and non-promoted state
  - rollback behavior on drift detection

Still partial:

- offline/online evaluation currently uses local score gates and not yet external benchmark/production telemetry feeds
- rollout policy remains single-model default (`default-neural`) and needs extension for multi-model runtime governance

## Known Gaps vs #647 Acceptance Criteria

- full non-interactive PR flow with real platform integration still incomplete
- required integration/regression test matrix is not yet finalized (by plan, tests are handled last in this branch phase)
- README/implementation docs must continue to be updated whenever behavior changes

## Working Rule

Any claim in this file must be backed by current code behavior in this crate.  
If a capability is partial, it must be labeled partial.
