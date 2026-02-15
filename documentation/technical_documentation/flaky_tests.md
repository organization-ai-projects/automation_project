# Flaky Test Stabilization Guide

This document defines the workspace workflow for flaky test triage and
stabilization.

## Goal

- Track known flaky tests in one place.
- Apply deterministic fixes first.
- Keep CI and pre-push behavior predictable.

## Current Inventory

| Test | Area | Classification | Status | Mitigation |
| --- | --- | --- | --- | --- |
| `store::tests::audit_buffer::test_manual_flush` | `projects/products/stable/accounts/backend` | Timing / async I/O visibility | Stabilized | Poll for on-disk visibility after manual flush |

## Classification

- Timing/race conditions
- Shared state collisions
- I/O visibility/latency
- Async polling assumptions
- Environment dependencies

## Remediation Workflow

1. Reproduce the flaky test locally with repeated runs.
2. Classify the failure mode.
3. Stabilize with deterministic setup/assertions (polling, isolation, ordering).
4. Add or update a regression test assertion.
5. Update this inventory row with status and mitigation.

## CI and Pre-Push Guidance

- Do not bypass flaky failures silently.
- If stabilization is not immediate, isolate with a tracked follow-up issue.
- Keep this inventory updated in the same PR as remediation work.
