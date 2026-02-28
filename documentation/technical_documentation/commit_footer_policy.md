# Commit Footer Policy

This document defines strict usage for commit footer keywords and issue references.

## Goal

Keep issue tracking deterministic and avoid ambiguous closure behavior.

## Footer Keywords

- `Closes #<issue>`: Use when the change fully closes an issue.
- `Fixes #<issue>`: Alternative closing keyword with the same closure semantics.
- `Part of #<issue>`: Use when a commit contributes to a larger parent issue but does not close it.
- `Reopen #<issue>`: Use to explicitly prevent closure when an issue was closed prematurely and work is still ongoing.

## Rules

- Use at most one closing keyword per referenced issue in the same commit.
- Do not mix closing and non-closing keywords for the same issue in one commit.
- Use `Part of` when work is partial.
- Use `Reopen` with the same issue number to explicitly neutralize closure in commit/PR text.

## Examples

```text
docs(governance): define branch naming convention

Closes #417
Part of #410
```

```text
fix(scripts/versioning/file_versioning/github): avoid premature close on out-of-sync issue state

Part of #389
Reopen #389
```

## Source of Truth

- This file is the authoritative footer policy.
- `CONTRIBUTING.md` must reference this policy instead of duplicating conflicting wording.

## References

- [CONTRIBUTING.md](../../CONTRIBUTING.md)
