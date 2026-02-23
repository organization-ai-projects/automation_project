# Table of Contents

Language: **English** | [Francais](i18n/fr/TOC.md)

This document provides an overview of all documentation files in this directory.

## Documentation

- [README.md](README.md): Main documentation for automation scripts

## Subdirectories

- [git_hooks/TOC.md](git_hooks/TOC.md): Git hooks for commit validation and quality checks

## Scripts

- [audit_security.sh](audit_security.sh): Security audit on dependencies
- [audit_issue_status.sh](audit_issue_status.sh): Audit open issues vs commit references
- [build_accounts_ui.sh](build_accounts_ui.sh): Build accounts UI bundle
- [build_and_check_ui_bundles.sh](build_and_check_ui_bundles.sh): Build and verify artifacts
- [build_ui_bundles.sh](build_ui_bundles.sh): Discover and build all UI bundles
- [changed_crates.sh](changed_crates.sh): List crates touched in a diff
- [check_dependencies.sh](check_dependencies.sh): Check for outdated/missing dependencies
- [check_script_integrity.sh](check_script_integrity.sh): Validate script root-path/sourcing integrity
- [check_merge_conflicts.sh](check_merge_conflicts.sh): Test merge for conflicts
- [clean_artifacts.sh](clean_artifacts.sh): Clean build artifacts
- [git_add_command_override.sh](git_add_command_override.sh): Shell override for git add
- [git_add_guard.sh](git_add_guard.sh): Guarded staging with split-policy checks
- [pre_add_review.sh](pre_add_review.sh): Pre-add internal review (format, clippy, tests)
- [pre_push_check.sh](pre_push_check.sh): Pre-push validation (checks, tests, conflicts)
- [release_prepare.sh](release_prepare.sh): Prepare releases with version/changelog/tag
- [setup_hooks.sh](setup_hooks.sh): Install git hooks
- [SCRIPT_WORKFLOWS.md](SCRIPT_WORKFLOWS.md): Canonical workflow inventory and supported script paths
- [sync_docs.sh](sync_docs.sh): Documentation synchronization
- [test_coverage.sh](test_coverage.sh): Generate test coverage reports
- [tests/critical_workflows_regression.sh](tests/critical_workflows_regression.sh): Minimal shell regression suite for critical workflows
