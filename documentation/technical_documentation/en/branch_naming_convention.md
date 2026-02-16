# Branch Naming Convention

This document is the source of truth for branch naming in this repository.

## Goal

Keep branch names predictable, readable, and easy to classify in automation and reviews.

## Required Format

```text
<type>/<short-kebab-description>
```

- `type` is mandatory and must be one of the allowed values below.
- `short-kebab-description` must be lowercase and use `-` as separator.

## Allowed Types

- `feat` or `feature`
- `fix`
- `refactor`
- `docs` or `doc`
- `test` or `tests`
- `chore`
- `fixture`
- `sync`
- `enhancement`

## Sub-PR Pattern

When a sub-PR branch is needed, use:

```text
<owner>/sub-pr-<parent-pr-number>
```

Example:

```text
remi-bezot/sub-pr-378
```

## Forbidden Patterns

- Uppercase names (for example `Fix/Parser`).
- Spaces or underscores (for example `fix/json_parser`).
- Missing type prefix (for example `parser-fix`).
- Generic names (for example `tmp`, `test`, `update`).

## Examples

Valid:

- `fix/scripts-breaking-detection`
- `docs/template-conventions`
- `enhancement/pr-description-hardening`
- `remi-bezot/sub-pr-378`

Invalid:

- `Fix/scripts-breaking-detection`
- `fix/scripts_breaking_detection`
- `scripts-breaking-detection`
- `tmp`

## References

- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
