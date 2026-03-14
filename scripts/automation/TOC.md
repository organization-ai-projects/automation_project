# Table of Contents

Language: **English** | [Francais](i18n/fr/TOC.md)

This document provides an overview of all documentation files in this directory.

## Documentation

- [README.md](README.md): Main documentation for automation scripts

## Subdirectories

- [git_hooks/TOC.md](git_hooks/TOC.md): Git hooks for commit validation and quality checks

## Scripts

- [audit_issue_status.sh](audit_issue_status.sh): Audit open issues vs commit references
- [check_script_integrity.sh](check_script_integrity.sh): Validate script root-path/sourcing integrity
- [git_add_command_override.sh](git_add_command_override.sh): Shell override for git add
- [git_add_guard.sh](git_add_guard.sh): Guarded staging with split-policy checks
- [pre_add_review.sh](pre_add_review.sh): Pre-add internal review (format, clippy, tests)
- [release_prepare.sh](release_prepare.sh): Prepare releases with version/changelog/tag
- [SCRIPT_WORKFLOWS.md](SCRIPT_WORKFLOWS.md): Canonical workflow inventory and supported script paths
- [test_coverage.sh](test_coverage.sh): Generate test coverage reports
- [tests/critical_workflows_regression.sh](tests/critical_workflows_regression.sh): Critical cross-workflow regression suite
- [tests/enforcer_shell_contract_regression.sh](tests/enforcer_shell_contract_regression.sh): Enforcer strict-mode guard on shell-structure violations

## Migrated Rust Commands

- `versioning_automation automation audit-security`
- `versioning_automation automation build-accounts-ui`
- `versioning_automation automation build-ui-bundles`
- `versioning_automation automation build-and-check-ui-bundles`
- `versioning_automation automation changed-crates [<ref1>] [<ref2>] [--output-format paths]`
- `versioning_automation automation check-dependencies`
- `versioning_automation automation check-merge-conflicts`
- `versioning_automation automation clean-artifacts`
