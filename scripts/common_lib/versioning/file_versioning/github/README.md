# GitHub Issue Helpers

Shared helper functions for GitHub issue-oriented automation scripts.

## Files

- `commands.sh`: Single remote-provider CLI backend (`vcs_remote_*` wrappers).
- `issue_helpers.sh`: Common helpers for:
  - extracting `#123` references from markdown task-list lines
  - querying sub-issues with GitHub GraphQL
  - upserting marker-based issue comments
- `pull_request_lookup.sh`: Common PR lookup helpers.

## Usage

```bash
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/commands.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/versioning/file_versioning/github/commands.sh"
```

## Notes

- `commands.sh` is the only file in this module allowed to execute the provider CLI directly.
- Other scripts/helpers must use `vcs_remote_*` wrappers.
- They require `gh` and `jq` to be available in the caller environment.
