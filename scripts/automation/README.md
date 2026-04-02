# Automation Documentation

This directory contains active shell entrypoints for project-wide automation tasks.
Versioning and migrated automation logic are canonical in Rust under
`tools/versioning_automation`.

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
│   ├── prepare-commit-msg          # Auto-generates commit subject
│   ├── pre-push                    # Runs quality checks before push
│   └── (installed via `versioning_automation automation install-hooks`)
├── git_add_command_override.sh     # Shell override for git add -> staging guard
├── git_add_guard.sh                # Guarded staging with split-policy checks
├── check_script_integrity.sh       # Validate script sourcing/root-path integrity
├── tests/
│   ├── critical_workflows_regression.sh # Critical cross-workflow regression suite
│   └── enforcer_shell_contract_regression.sh # Enforcer check for shell-structure violations
├── SCRIPT_WORKFLOWS.md             # Canonical workflow inventory + entrypoints
└── tests/                          # Shell regression/integration tests
```

## Files

For the exhaustive, always-updated list, use:

- `TOC.md`

High-level groups:

- `git_hooks/`: Git hooks for commit validation and pre-push checks.
- quality/security/build scripts: `check_*.sh`.
- git safety helpers: `git_add_guard.sh`, `git_add_command_override.sh`.
- canonical pre-push hook: `scripts/automation/git_hooks/pre-push`.
- regression/integrity guards: `check_script_integrity.sh`, `tests/*.sh`, `SCRIPT_WORKFLOWS.md`.

## Optional shell override for `git add`

If you want to keep using `git add ...` (without a custom alias), source:

```bash
source /absolute/path/to/repo/scripts/automation/git_add_command_override.sh
```

After sourcing, only `git add` is overridden; all other `git` commands are unchanged.

## Adding New Automation Scripts

When adding a new automation script:

1. **Does it operate on the whole repository?** → Belongs here
2. **Is it Git/GitHub versioning automation logic?** → Belongs in `tools/versioning_automation` (Rust CLI)
3. **Is it a reusable shell utility?** → Belongs in `scripts/common_lib/`

Document the script in:

- This README (add to list)
- `TOC.md` (required)
- `SCRIPT_WORKFLOWS.md` when it is a user-facing workflow entrypoint

## Migrated Rust Commands

Use these commands directly instead of removed shell wrappers:

- `versioning_automation automation audit-security`
- `versioning_automation automation build-accounts-ui`
- `versioning_automation automation build-ui-bundles`
- `versioning_automation automation build-and-check-ui-bundles`
- `versioning_automation automation changed-crates [<ref1>] [<ref2>] [--output-format paths]`
- `versioning_automation automation check-dependencies`
- `versioning_automation automation check-merge-conflicts`
- `versioning_automation automation clean-artifacts`
- `versioning_automation automation pre-add-review`
- `versioning_automation automation test-coverage`
- `versioning_automation automation audit-issue-status [--repo owner/name] [--base origin/main] [--head origin/dev] [--limit <n>] [--output <file>]`
- `versioning_automation automation release-prepare <version> [--auto-changelog]`
- `versioning_automation automation install-hooks`
