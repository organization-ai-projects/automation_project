# pr_auto_closes_enrichment.yml Documentation

This workflow auto-enriches open pull requests targeting `dev` with managed `Closes #...` lines when assignment criteria are satisfied.

## Purpose

- Detect `Part of #<n>` references from PR title/body and commit messages.
- If issue `#<n>` has exactly one assignee and that assignee matches the PR author, add `Closes #<n>` to the PR body.
- Keep closure directives deterministic and centrally managed in a dedicated body block.

## Trigger Events

- `pull_request_target` on branch `dev`:
  - `opened`
  - `edited`
  - `synchronize`
  - `reopened`
  - `ready_for_review`
- `workflow_dispatch` with `pr_number`

## Script Entry Point

- `scripts/versioning/file_versioning/github/auto_add_closes_on_dev_pr.sh`

## Managed Block Contract

The workflow writes a dedicated PR body section:

```md
<!-- auto-closes:start -->
### Auto-managed Issue Closures
Closes #123
<!-- auto-closes:end -->
```

On each run, the block is recomputed and replaced idempotently.

## Notes

- This workflow only enriches PR body content; it does not close issues directly.
- `done-in-dev` labeling remains managed by `issue_done_in_dev_status.yml` after merge into `dev`.
