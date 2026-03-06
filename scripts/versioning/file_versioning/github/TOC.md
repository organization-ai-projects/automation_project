# Table of Contents

Language: **English** | [Francais](i18n/fr/TOC.md)

This document provides an overview of all documentation files in this directory.

## Documentation

- [README.md](README.md): Main documentation for GitHub CLI scripts

## Scripts

- [auto_link_parent_issue.sh](auto_link_parent_issue.sh): Auto-link child issues to parent issues from `Parent:` issue-body field
- [auto_add_closes_on_dev_pr/run.sh](auto_add_closes_on_dev_pr/run.sh): Auto-enrich open PRs targeting `dev` with managed `Closes #...` lines when assignment criteria are met
- [create_direct_issue.sh](create_direct_issue.sh): Internal direct-issue contract script (deprecated as user-facing entrypoint)
- [generate_pr_description.sh](generate_pr_description.sh): Generate structured merge PR descriptions from GitHub metadata
- [issue_done_in_dev_status.sh](issue_done_in_dev_status.sh): Manage `done-in-dev` status label on dev merges and issue closure
- [issue_reopen_on_dev_merge.sh](issue_reopen_on_dev_merge.sh): Reopen issues referenced by `Reopen #...` on merged PRs into `dev` and clear `done-in-dev` label
- [neutralize_non_compliant_closure_refs.sh](neutralize_non_compliant_closure_refs.sh): Neutralize `Closes/Fixes/Resolves` refs when target issues are non-compliant or explicitly reopened
- [manager_issues.sh](manager_issues.sh): Route issue lifecycle operations (create, read, update, close, reopen, soft-delete) with deterministic validation
- [parent_issue_guard/run.sh](parent_issue_guard/run.sh): Guard parent issue closure and publish parent/child status summaries
- [lib/classification.sh](lib/classification.sh): Classification and issue-action helpers used by the generator
- [issues/required_fields/module.sh](issues/required_fields/module.sh): Shared required issue title/body contract validator
- [lib/rendering.sh](lib/rendering.sh): Rendering helpers for sections and dynamic PR titles
- [tests/generate_pr_description_regression.sh](tests/generate_pr_description_regression.sh): Regression matrix for CLI argument and mode behavior
- [tests/auto_add_closes_on_dev_pr_regression.sh](tests/auto_add_closes_on_dev_pr_regression.sh): Regression tests for automatic `Closes #...` enrichment on dev-targeting PRs
- [tests/issue_done_in_dev_status_regression.sh](tests/issue_done_in_dev_status_regression.sh): Regression tests for done-in-dev label lifecycle automation
- [tests/issue_reopen_on_dev_merge_regression.sh](tests/issue_reopen_on_dev_merge_regression.sh): Regression tests for `Reopen #...` sync on dev-merge
- [tests/manager_issues_regression.sh](tests/manager_issues_regression.sh): Regression tests for manager_issues create/read/update/close/reopen/soft-delete flows

## Navigation

- [Back to File Versioning TOC](../TOC.md)
