# issue_done_in_dev_status.yml Documentation

This workflow maintains the `done-in-dev` issue label lifecycle.

## Purpose

- Add `done-in-dev` when a PR is merged into `dev` and references issue closures (`Closes #...`).
- Remove `done-in-dev` when an issue is closed.

## Triggers

- `pull_request` (`closed`) on branch `dev`
  - Runs only when PR is actually merged.
- `issues` (`closed`)
  - Removes `done-in-dev` from the closed issue if present.
- `workflow_dispatch`
  - Manual run for either:
    - `dev_merge` mode with `pr_number`
    - `issue_closed` mode with `issue_number`

## Permissions

- `contents: read`
- `pull-requests: read`
- `issues: write`

## Script Used

- `scripts/versioning/file_versioning/github/issue_done_in_dev_status.sh`

The script:

- parses closure refs from PR title/body/commit messages,
- labels referenced open issues with `done-in-dev`,
- removes that label when an issue closes.
