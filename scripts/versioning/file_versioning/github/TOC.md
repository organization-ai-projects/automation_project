# Table of Contents

Language: **English** | [Francais](i18n/fr/TOC.md)

## Documentation

- [README.md](README.md): GitHub automation overview and canonical Rust entrypoints

## Regression Tests

- [tests/generate_pr_description_regression.sh](tests/generate_pr_description_regression.sh): PR description generation regression matrix
- [tests/refresh_pr_issue_extraction_regression.sh](tests/refresh_pr_issue_extraction_regression.sh): PR/issue extraction refresh regression matrix
- [tests/auto_add_closes_on_dev_pr_regression.sh](tests/auto_add_closes_on_dev_pr_regression.sh): Auto-managed `Closes #...` enrichment checks
- [tests/pr_directive_conflict_guard_regression.sh](tests/pr_directive_conflict_guard_regression.sh): Directive conflict guard checks
- [tests/auto_link_parent_issue_regression.sh](tests/auto_link_parent_issue_regression.sh): Parent auto-link checks
- [tests/issue_done_in_dev_status_regression.sh](tests/issue_done_in_dev_status_regression.sh): `done-in-dev` lifecycle checks
- [tests/issue_reopen_on_dev_merge_regression.sh](tests/issue_reopen_on_dev_merge_regression.sh): `Reopen #...` synchronization checks
- [tests/neutralize_closure_refs_regression.sh](tests/neutralize_closure_refs_regression.sh): Closure neutralization and reevaluate checks
- [tests/parent_issue_guard_regression.sh](tests/parent_issue_guard_regression.sh): Parent guard checks
- [tests/closure_hygiene_on_main_merge_regression.sh](tests/closure_hygiene_on_main_merge_regression.sh): Main-merge closure hygiene checks
- [tests/manager_issues_regression.sh](tests/manager_issues_regression.sh): Issue lifecycle command checks
- [tests/shellcheck_regression.sh](tests/shellcheck_regression.sh): Shell lint checks for remaining script harnesses
- [tests/enforcer_shell_contract_regression.sh](tests/enforcer_shell_contract_regression.sh): Enforcer shell contract checks

## Navigation

- [Back to File Versioning TOC](../TOC.md)
