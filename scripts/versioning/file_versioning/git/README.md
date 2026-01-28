# Git Scripts

This directory contains scripts that use **only** the `git` command-line tool.

## Scope

Scripts in this directory should:

- Use only `git` commands (no `gh`, `gitlab-cli`, or other version control platform CLIs)
- Perform pure git operations (branches, commits, working tree, etc.)
- Be platform-agnostic (work with any git hosting: GitHub, GitLab, Gitea, etc.)

## Examples

- Branch management (create, delete, checkout)
- Commit operations
- Working tree state management
- Local repository operations

## Note

If a script needs to interact with GitHub/GitLab APIs or their CLIs, it should be placed at the parent level (`file_versioning/`) as a hybrid script, or in a future platform-specific directory if we create pure platform scripts.
