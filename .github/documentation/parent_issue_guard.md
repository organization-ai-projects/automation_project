# Parent Issue Guard

This automation enforces parent/child issue consistency.

## Goals

- Post deterministic status comments on parent issues.
- Re-open parent issues closed too early when required children are still open.
- Suggest closure when all required children are closed.

## Source

- Workflow: `.github/workflows/parent_issue_guard.yml`
- Script: `scripts/versioning/file_versioning/github/parent_issue_guard.sh`

## Detection Model

Parent candidates are evaluated from:

- Direct issue events (`opened`, `edited`, `reopened`, `closed`)
- Child issue events via search of issues referencing `#<child_number>`

Required children are currently read from parent task-list lines in issue body:

- `- [ ] #123 ...`
- `- [x] #123 ...`

## Guard Behavior

- Strict guard is enabled in workflow configuration.
- If open required children remain:
  - parent status comment is updated with remaining children.
  - if parent is closed, it is reopened (`strict guard`).
- If all required children are closed:
  - parent status comment indicates it can be closed.

## Manual Run

Use workflow dispatch input `issue_number` to force evaluation of one parent issue.
