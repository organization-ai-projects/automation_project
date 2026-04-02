# issue_reopen_on_dev_merge.yml Documentation

This workflow synchronizes `Reopen #...` directives for PRs targeting `dev`.

## Purpose

- Reopen issues referenced by `Reopen #...` in PR title/body/commit messages.
- Remove `done-in-dev` label from those reopened issues when present.

## Triggers

- `pull_request` (`opened`, `synchronize`, `reopened`, `edited`, `closed`) on branch `dev`
  - Runs for active work on PRs targeting `dev`
  - Closed-but-not-merged PRs are ignored by the command logic
- `workflow_dispatch`
  - Manual run with required `pr_number`.

## Permissions

- `contents: read`
- `pull-requests: read`
- `issues: write`

## Script Used

- `versioning_automation issue reopen-on-dev`
