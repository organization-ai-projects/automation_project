# GitHub Helpers

Reusable remote-provider helpers for GitHub-oriented automation scripts.

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

## Wrapper API

All wrappers are variadic and forward extra args with `"$@"`.

- Primitive:
  - `vcs_remote_run <gh-subcommand...>`
  - `vcs_remote_api <api-args...>`
- Repo:
  - `vcs_remote_repo_view <args...>`
- Pull requests:
  - `vcs_remote_pr_list <args...>`
  - `vcs_remote_pr_view <args...>`
  - `vcs_remote_pr_edit <args...>`
  - `vcs_remote_pr_create <args...>`
  - `vcs_remote_pr_merge <args...>`
- Issues:
  - `vcs_remote_issue_list <args...>`
  - `vcs_remote_issue_view <args...>`
  - `vcs_remote_issue_edit <args...>`
  - `vcs_remote_issue_create <args...>`
  - `vcs_remote_issue_close <args...>`
  - `vcs_remote_issue_reopen <args...>`
- Labels:
  - `vcs_remote_label_list <args...>`
  - `vcs_remote_label_edit <args...>`
  - `vcs_remote_label_create <args...>`
  - `vcs_remote_label_delete <args...>`

## Examples

```bash
vcs_remote_pr_view 123 -R owner/repo --json title,body
vcs_remote_issue_edit 456 -R owner/repo --add-label bug
vcs_remote_label_list -R owner/repo --limit 1000 --json name --jq '.[].name'
vcs_remote_api "repos/owner/repo/issues/456/comments" --paginate
```

## Notes

- `commands.sh` is the only file in this module allowed to execute the provider CLI directly.
- Other scripts/helpers must use `vcs_remote_*` wrappers.
- They require `gh` and `jq` to be available in the caller environment.
