# Versioning Scripts

This directory contains scripts for managing versions at different levels.

## Scope

Scripts here handle:

- **File-level versioning** - Version control workflows (branches, commits, PRs)
- **System-level versioning** - Release versioning (v1.0.0, changelog, tags)

## Structure

- **`file_versioning/`** - Version control workflows (branches, PRs, syncing)
  - `git/` - Pure git operations
  - `github/` - GitHub CLI operations (reserved, currently empty)
  - Root level scripts - Hybrid git + GitHub operations

For details, see `file_versioning/README.md`

## Current Scripts

### File Versioning (`file_versioning/`)

- **Branch management** - Create, delete, clean, manage branches
- **Pull request automation** - Create PRs, watch CI, sync labels
- **Repository synchronization** - Sync main↔dev branches

See `file_versioning/README.md` for complete list and organization.

## Adding New Versioning Scripts

When adding a versioning script:

1. **Is it about version control workflows?** → Place in `file_versioning/`
2. **Is it about release versions?** → Place at this level
3. **Does it need git only?** → `file_versioning/git/`
4. **Does it need GitHub only?** → `file_versioning/github/`
5. **Does it need multiple tools?** → `file_versioning/` root level

Document the script in:

- Relevant README (this file or `file_versioning/README.md`)
- `documentation/technical_documentation/versioning/file_versioning/scripts_overview.md`
