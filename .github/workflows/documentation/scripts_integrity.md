# scripts_integrity.yml

Runs script integrity and shell regression checks for automation/versioning tooling.

## Trigger

- Pull requests touching `scripts/**`, issue contracts, or issue templates
- Pushes to `dev` and `main` touching the same paths

## Checks

1. `bash scripts/automation/check_script_integrity.sh`
   - user-facing workflow inventory output
   - `ROOT_DIR` resolution validation
   - sourced helper path validation
   - required helper import checks (for example `git_fetch_prune`)

2. `bash scripts/automation/tests/critical_workflows_regression.sh`
   - direct issue creation dry-run contract test
   - closure neutralizer regression suite
   - done-in-dev status regression suite
   - hook convention guardrails regression suite
   - auto-add closes-on-dev-PR regression suite

## Purpose

Provides a CI gate for script integrity so scripting regressions are caught before merge.
