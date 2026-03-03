# GitHub Issue Helpers

Shared helper functions for GitHub issue-oriented automation scripts.

## Files

- `issue_helpers.sh`: Common helpers for:
  - extracting `#123` references from markdown task-list lines
  - querying sub-issues with GitHub GraphQL
  - upserting marker-based issue comments

## Usage

```bash
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh"
```

## Notes

- These helpers are intended to be sourced by scripts under `scripts/versioning/file_versioning/github/`.
- They require `gh` and `jq` to be available in the caller environment.
