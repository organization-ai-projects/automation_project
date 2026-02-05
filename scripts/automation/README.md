# Automation Documentation

This directory contains scripts for project-wide automation tasks.

## Role in the Project

This directory is responsible for automating repository-wide tasks including builds, testing, quality checks, security audits, and release preparation.
It interacts mainly with:

- Cargo workspace and build system
- Test infrastructure and coverage tools
- Git hooks and quality gates
- Dependency management and security scanners
- Release tooling and changelog generators

## Directory Structure

```plaintext
automation/
├── git_hooks/                      # Git hooks for commit validation and pre-push checks
│   ├── commit-msg                  # Validates commit message format
│   ├── pre-commit                  # Runs code formatting before commit
│   ├── pre-push                    # Runs quality checks before push
│   └── install_hooks.sh            # Installs git hooks
├── audit_security.sh               # Security audit on dependencies
├── build_accounts_ui.sh            # Build accounts UI bundle
├── build_and_check_ui_bundles.sh   # Build and verify artifacts
├── build_ui_bundles.sh             # Discover and build all UI bundles
├── changed_crates.sh               # List crates touched in a diff
├── check_dependencies.sh           # Check for outdated/missing dependencies
├── check_merge_conflicts.sh        # Test merge for conflicts
├── clean_artifacts.sh              # Clean build artifacts
├── pre_add_review.sh               # Pre-add internal review (format, clippy, tests)
├── pre_push_check.sh               # Pre-push validation (checks, tests, conflicts)
├── release_prepare.sh              # Prepare releases with version/changelog/tag
├── setup_hooks.sh                  # Install git hooks
├── sync_docs.sh                    # Documentation synchronization (placeholder)
└── test_coverage.sh                # Generate test coverage reports
```

## Files

- `README.md`: This file.
- `git_hooks/`: Git hooks for commit validation and pre-push checks.
- `audit_security.sh`: Security audit on dependencies.
- `build_accounts_ui.sh`: Build accounts UI bundle.
- `build_and_check_ui_bundles.sh`: Build and verify artifacts.
- `build_ui_bundles.sh`: Discover and build all UI bundles.
- `changed_crates.sh`: List crates touched in a diff.
- `check_dependencies.sh`: Check for outdated/missing dependencies.
- `check_merge_conflicts.sh`: Test merge for conflicts.
- `clean_artifacts.sh`: Clean build artifacts.
- `pre_add_review.sh`: Pre-add internal review.
- `pre_push_check.sh`: Pre-push validation.
- `release_prepare.sh`: Prepare releases with version/changelog/tag.
- `setup_hooks.sh`: Install git hooks.
- `sync_docs.sh`: Documentation synchronization (placeholder).
- `test_coverage.sh`: Generate test coverage reports.

## Adding New Automation Scripts

When adding a new automation script:

1. **Does it operate on the whole repository?** → Belongs here
2. **Is it a version control workflow task?** → Belongs in `versioning/`
3. **Is it a reusable utility?** → Belongs in `common/`

Document the script in:

- This README (add to list)
- `documentation/technical_documentation/versioning/file_versioning/scripts_overview.md`
