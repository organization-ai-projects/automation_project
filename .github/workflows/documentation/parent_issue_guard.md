# parent_issue_guard.yml

## Purpose

Keeps parent issues consistent with required child issue state.

## Triggers

- `issues`: `opened`, `edited`, `reopened`, `closed`
- `workflow_dispatch` with optional `issue_number`

## Behavior

- Evaluates current issue as parent candidate.
- Evaluates parent candidates that reference a changed child issue.
- Posts/updates a deterministic status comment on parent.
- Reopens parent when strict guard is enabled and required children remain open.

## Script

This workflow delegates core logic to:

- `scripts/versioning/file_versioning/github/parent_issue_guard.sh`
