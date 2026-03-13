# Script Workflow Inventory

This file is the canonical inventory of user-facing script entrypoints and their workflow category.

## Supported Invocation Paths

- Preferred: run from repository root with explicit relative paths (for example `bash scripts/...`).
- Supported: run from any location if script computes `ROOT_DIR` from `SCRIPT_DIR` and resolves sourced files through `$ROOT_DIR`.
- Unsupported: ad-hoc copied scripts executed outside repository tree.

## User-Facing Workflows

Workflow | Entrypoint | Scope
--- | --- | ---
branching | `versioning_automation git create-branch ...` | Create validated branch from `dev`
branching | `versioning_automation git create-work-branch ...` | Create convention-based work branch
commit_push | `versioning_automation git add-commit-push ...` | Add, commit, push with validation
commit_push | `versioning_automation git push-branch ...` | Push current branch with policy checks
pre_push | `scripts/automation/git_hooks/pre-push` | Pre-push quality gate
pr_creation | `versioning_automation pr generate-description ...` | Canonical PR create/refresh entrypoint (Rust CLI)
issue_creation | `versioning_automation issue create ...` | Canonical direct issue creation entrypoint (Rust CLI)
issue_lifecycle | `versioning_automation issue <read/update/close/reopen/delete> ...` | Canonical issue lifecycle entrypoint (Rust CLI)

## Issue Templates and Validation Profiles

- Direct issues: `.github/ISSUE_TEMPLATE/direct_issue.md`, validated by default issue contract keys (`ISSUE_*`).
- Review follow-up issues: `.github/ISSUE_TEMPLATE/review_followup.md` + `review` label, validated by review contract keys (`ISSUE_REVIEW_*`).

## Integrity and Regression Entrypoints

- Integrity checks: `bash scripts/automation/check_script_integrity.sh`
- Critical shell regressions: `bash scripts/automation/tests/critical_workflows_regression.sh`
