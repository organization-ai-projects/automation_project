# Issue Parent Autolink

This automation links child issues to parent issues from a normalized issue-body field.

## Goal

- Reduce manual parent/child linking in GitHub UI.
- Keep issue hierarchy deterministic and automation-friendly.

## Source

- Workflow: `.github/workflows/issue_parent_autolink.yml`
- Script: `scripts/versioning/file_versioning/github/auto_link_parent_issue.sh`

## Required Issue Field

Issues should include:

```md
## Hierarchy
Parent: #123
```

Supported values:

- `Parent: #<issue_number>` -> child should be linked to parent.
- `Parent: none` -> no parent link required.

## Behavior

On `issues.opened`, `issues.edited`, `issues.reopened`:

1. Read `Parent:` from issue body.
2. Validate value format and parent issue state.
3. If valid, attempt GitHub API sub-issue linking.
4. Post/update deterministic status comment on child issue.
5. Apply labels for invalid/failed automation paths.

## Failure Handling

When automation cannot link the issue, it leaves:

- actionable status comment with remediation,
- labels:
  - `invalid` (field/format problem),
  - `automation-failed` (API/linking failure).

This keeps failures visible and auditable without silent drops.

