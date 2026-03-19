# issue_reopen_on_dev_merge.yml Documentation

This workflow synchronizes `Reopen #...` directives when a PR is merged into `dev`.

## Purpose

- Reopen issues referenced by `Reopen #...` in merged PR title/body/commit messages.
- Remove `done-in-dev` label from those reopened issues when present.

## Triggers

- `pull_request` (`closed`) on branch `dev`
  - Runs only when PR is actually merged.
- `workflow_dispatch`
  - Manual run with required `pr_number`.

## Permissions

- `contents: read`
- `pull-requests: read`
- `issues: write`

## Script Used

- `versioning_automation issue reopen-on-dev`
