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

## Known Gaps vs #647 Acceptance Criteria

- end-to-end fixture-based autonomous scenario under CI-like conditions still needs explicit validation artifacts
- full non-interactive PR flow with real platform integration still incomplete
- required integration/regression test matrix is not yet finalized (by plan, tests are handled last in this branch phase)
- README/implementation docs must continue to be updated whenever behavior changes

## Working Rule

Any claim in this file must be backed by current code behavior in this crate.  
If a capability is partial, it must be labeled partial.
