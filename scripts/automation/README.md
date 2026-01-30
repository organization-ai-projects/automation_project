# Automation Scripts

This directory contains scripts for project-wide automation tasks.

## Scope

Scripts here automate tasks that operate on the repository as a whole:

- **Building** - UI bundles, artifacts
- **Testing** - Test coverage, dependency checks
- **Checking** - Code quality, merge conflicts, security audits
- **Releasing** - Version bumps, changelogs, git tags
- **Setup** - Git hooks installation

## Organization

All scripts in this directory:

- Operate on the entire workspace/repository
- Use shared utilities from `scripts/common_lib/`
- Follow the quality standards defined in `scripts_overview.md`

## Current Scripts

- `build_accounts_ui.sh` - Build accounts UI bundle
- `build_ui_bundles.sh` - Discover and build all UI bundles
- `build_and_check_ui_bundles.sh` - Build and verify artifacts
- `pre_push_check.sh` - Pre-push validation (checks, tests, conflicts)
- `pre_add_review.sh` - Pre-add internal review (format, clippy, tests)
- `setup_hooks.sh` - Install git hooks
- `audit_security.sh` - Security audit on dependencies
- `check_dependencies.sh` - Check for outdated/missing dependencies
- `check_merge_conflicts.sh` - Test merge for conflicts
- `changed_crates.sh` - List crates touched in a diff
- `clean_artifacts.sh` - Clean build artifacts
- `test_coverage.sh` - Generate test coverage reports
- `release_prepare.sh` - Prepare releases with version/changelog/tag
- `sync_docs.sh` - Documentation synchronization (placeholder)

## Adding New Automation Scripts

When adding a new automation script:

1. **Does it operate on the whole repository?** → Belongs here
2. **Is it a version control workflow task?** → Belongs in `versioning/`
3. **Is it a reusable utility?** → Belongs in `common/`

Document the script in:

- This README (add to list)
- `documentation/technical_documentation/versioning/file_versioning/scripts_overview.md`
