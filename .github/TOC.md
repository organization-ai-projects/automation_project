# Table of Contents

This document provides an overview of all documentation files in this directory.

## Workflows

- [ci_main.yml](workflows/ci_main.yml): Handles CI tasks for the `main` branch.
- [ci_dev.yml](workflows/ci_dev.yml): Handles CI tasks for the `dev` branch.
- [ci_reusable.yml](workflows/ci_reusable.yml): A reusable workflow for common CI steps.
- [scripts_integrity.yml](workflows/scripts_integrity.yml): Script integrity and shell regression validation gate.
- [automation_rustfmt.yml](workflows/automation_rustfmt.yml): Applies rustfmt on PRs.
- [automation_sync.yml](workflows/automation_sync.yml): Syncs `main` into `dev` after merge.
- [issue_done_in_dev_status.yml](workflows/issue_done_in_dev_status.yml): Adds/removes `done-in-dev` label based on dev merges and issue closures.
- [pr_auto_closes_enrichment.yml](workflows/pr_auto_closes_enrichment.yml): Auto-adds managed `Closes #...` lines to open PRs targeting `dev` when assignment criteria are met.
- [pr_closure_neutralizer.yml](workflows/pr_closure_neutralizer.yml): Neutralizes closure refs to non-compliant issues (`... rejected #...`) and comments remediation.

## Documentation

- [GitHub Documentation TOC](documentation/TOC.md): Index for GitHub-specific documentation.
- [Workflows Overview](documentation/workflows_overview.md): General overview of the workflows.
- [Workflows Documentation](workflows/documentation/README.md): Detailed documentation for each workflow.

## Related Governance Docs

- [Branch Naming Convention](../documentation/technical_documentation/branch_naming_convention.md): Formal branch naming policy and examples.
- [Commit Footer Policy](../documentation/technical_documentation/commit_footer_policy.md): Rules for `Part of`/`Closes`/`Fixes` and `Reopen` override.
- [Documentation Ownership Map](../documentation/technical_documentation/documentation_ownership_map.md): Ownership responsibilities for documentation areas.
- [Labels Taxonomy](../documentation/technical_documentation/labels_taxonomy.md): Label policy for issues and pull requests.
