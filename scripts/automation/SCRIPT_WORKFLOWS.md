# Script Workflow Inventory

This file is the canonical inventory of user-facing script entrypoints and their workflow category.

## Supported Invocation Paths

- Preferred: run from repository root with explicit relative paths (for example `bash scripts/...`).
- Supported: run from any location if script computes `ROOT_DIR` from `SCRIPT_DIR` and resolves sourced files through `$ROOT_DIR`.
- Unsupported: ad-hoc copied scripts executed outside repository tree.

## User-Facing Workflows

Workflow | Entrypoint | Scope
--- | --- | ---
start_work | `scripts/versioning/file_versioning/orchestrators/execute/start_work.sh` | Sync + priority checks + branch creation
branching | `scripts/versioning/file_versioning/git/create_branch.sh` | Create validated branch from `dev`
branching | `scripts/versioning/file_versioning/git/create_work_branch.sh` | Create convention-based work branch
commit_push | `scripts/versioning/file_versioning/git/add_commit_push.sh` | Add, commit, push with validation
commit_push | `scripts/versioning/file_versioning/git/push_branch.sh` | Push current branch with policy checks
pre_push | `scripts/automation/pre_push_check.sh` | Pre-push quality gate
pr_creation | `scripts/versioning/file_versioning/github/generate_pr_description.sh` | Canonical PR create/refresh entrypoint
issue_creation | `scripts/versioning/file_versioning/github/create_direct_issue.sh` | Create direct issue from issue contract
issue_lifecycle | `scripts/versioning/file_versioning/github/manager_issues.sh` | Route create/update/close/reopen issue operations

## Issue Templates and Validation Profiles

- Direct issues: `.github/ISSUE_TEMPLATE/direct_issue.md`, validated by default issue contract keys (`ISSUE_*`).
- Review follow-up issues: `.github/ISSUE_TEMPLATE/review_followup.md` + `review` label, validated by review contract keys (`ISSUE_REVIEW_*`).

## Integrity and Regression Entrypoints

- Integrity checks: `bash scripts/automation/check_script_integrity.sh`
- Critical shell regressions: `bash scripts/automation/tests/critical_workflows_regression.sh`
