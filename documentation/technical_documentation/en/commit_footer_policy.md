# Commit Footer Policy

This document defines strict usage for commit footer keywords and issue references.

## Goal

Keep issue tracking deterministic and avoid ambiguous closure behavior.

## Footer Keywords

- `Closes #<issue>`: Use when the change fully closes an issue.
- `Fixes #<issue>`: Use for bug fixes that resolve incorrect behavior.
- `Resolves #<issue>`: Use when completion is not strictly a bug fix but the issue is fully addressed.
- `Related to #<issue>`: Use for linked context without closure.
- `Part of #<issue>`: Use when a commit contributes to a larger parent issue but does not close it.

## Rules

- Use at most one closing keyword per referenced issue in the same commit.
- Do not mix closing and non-closing keywords for the same issue in one commit.
- Prefer `Closes` for documentation/governance/process issues.
- Prefer `Fixes` for confirmed defects.
- Use `Related to` or `Part of` when work is partial.

## Examples

```text
docs(governance): define branch naming convention

Closes #417
Related to #410
```

```text
fix(scripts/versioning/file_versioning/github): avoid false positive breaking detection

Fixes #389
Part of #403
```

## Source of Truth

- This file is the authoritative footer policy.
- `CONTRIBUTING.md` must reference this policy instead of duplicating conflicting wording.

## References

- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
