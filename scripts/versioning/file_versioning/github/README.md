# GitHub Scripts Documentation

This directory contains scripts focused on GitHub workflows and PR metadata generation.

## Role in the Project

This directory is reserved for GitHub-facing operations.
It interacts mainly with:

- GitHub API (via `gh` CLI)
- Local git history (for dry-run/fallback generation)
- Repository settings and configurations
- GitHub Actions workflows
- Organization and team management

## Directory Structure

```
github/
├── README.md (this file)
├── TOC.md
├── auto_link_parent_issue.sh
├── create_direct_issue.sh
├── manager_issues.sh
├── generate_pr_description.sh
├── issue_done_in_dev_status.sh
├── neutralize_non_compliant_closure_refs.sh
├── parent_issue_guard.sh
├── lib/
│   ├── classification.sh
│   ├── issue_required_fields.sh
│   └── rendering.sh
└── tests/
    └── generate_pr_description_regression.sh
```

## Files

- `README.md`: This file.
- `TOC.md`: Documentation index for GitHub-only scripts.
- `generate_pr_description.sh`: Generate structured merge PR descriptions from PR metadata and/or local git history.
- `auto_link_parent_issue.sh`: Parse `Parent:` field and auto-link child issues to parent issues via GitHub API.
- `create_direct_issue.sh`: Internal create contract script used by manager routing (direct usage deprecated).
- `manager_issues.sh`: Unified issue lifecycle entrypoint for create/update/close/reopen operations.
- `issue_done_in_dev_status.sh`: Add `done-in-dev` on merged PRs into `dev` from closure refs, and remove it when issues close.
- `neutralize_non_compliant_closure_refs.sh`: Replace closure refs with `... rejected #...` when referenced issues are non-compliant.
- `parent_issue_guard.sh`: Evaluate parent/child issue status and prevent premature parent closure.
- `lib/classification.sh`: PR/issue classification helpers extracted from the main script.
- `lib/issue_required_fields.sh`: Shared validator for issue contracts (default direct-issue contract + review-followup contract keyed by `review` label).
- `lib/rendering.sh`: Output rendering helpers extracted from the main script.
- `tests/generate_pr_description_regression.sh`: Regression matrix for CLI modes and argument validation.
- `tests/issue_done_in_dev_status_regression.sh`: Regression checks for done-in-dev add/remove workflow behavior.
- `tests/manager_issues_regression.sh`: Regression checks for manager_issues lifecycle routing behavior.

## Scope

Scripts in this directory should:

- Focus on GitHub PR/issue workflows
- Prefer `gh` data when available
- Provide resilient fallbacks when GitHub API is unavailable

Issue contract routing:

- Default issues use `.github/issue_required_fields.conf` keys `ISSUE_*`.
- Review follow-up issues (label `review`) use `ISSUE_REVIEW_*` keys from the same contract file.
- Direct creation through `manager_issues.sh create` routes to `create_direct_issue.sh` and applies label `issue` by default.
- User-facing create flow must use `manager_issues.sh create` (not direct invocation of `create_direct_issue.sh`).

## Scripts

### `generate_pr_description.sh`

Generates a ready-to-paste PR description (e.g., `dev -> main`) by analyzing child PRs and resolved issues.
Supports both PR-number mode (GitHub-enriched) and local dry-run mode.
Generated body includes:

- `### Description`
- `### Scope`
- `### Compatibility`
- `### Issues Resolved`
- `### Key Changes`
- `### Testing`
- `### Additional Notes`

Usage:

```bash
bash generate_pr_description.sh [--keep-artifacts] [--debug] [--duplicate-mode MODE] [--auto-edit PR_NUMBER] MAIN_PR_NUMBER [OUTPUT_FILE]
bash generate_pr_description.sh --dry-run [--base BRANCH] [--head BRANCH] [--create-pr] [--allow-partial-create] [--duplicate-mode MODE] [--debug] [--auto-edit PR_NUMBER] [--yes] [OUTPUT_FILE]
bash generate_pr_description.sh --auto [--base BRANCH] [--head BRANCH] [--debug] [--yes]
```

Key options:

- `--dry-run`: Generate from local history (default base: `dev`, default head: current branch).
- `--base`, `--head`: Explicit branch range for dry-run extraction.
- `--create-pr`: In dry-run mode, optionally create the PR with the generated body.
- `--allow-partial-create`: Allow PR creation even if GitHub enrichment is incomplete.
- `--auto-edit PR_NUMBER`: Generate body in memory and update an existing PR directly (no output file).
- `--duplicate-mode MODE`: Duplicate handling mode (`safe` or `auto-close`).
- `--yes`: Non-interactive confirmation when `--create-pr` is used.
- `--debug`: Enable extraction and classification traces on stderr.
- `--auto`: RAM-first flow (`--dry-run` + `--create-pr`) with in-memory body.
- `--keep-artifacts`: Keep extracted PR/issue intermediate files.

Compatibility behavior:

- Default output is non-breaking:
  - `- Non-breaking change.`
- When breaking signals are detected:
  - `- Breaking change.`
- Compatibility switches to breaking only when explicit signals are detected in analyzed data:
  - checked `- [x] Breaking change` in PR body content
  - conventional-commit breaking marker (`!`) in PR/commit titles
  - `BREAKING CHANGE:` footer in analyzed PR/commit body text
  - `breaking` label on linked PRs/issues (when available via GitHub enrichment)

Scope behavior:

- `Scope` is always emitted with deterministic fallback:
  - `- Not explicitly provided.`

Duplicate handling:

- Default: disabled (no duplicate comment/closure action).
- `--duplicate-mode safe`: posts a standardized comment on detected duplicate issue references.
- `--duplicate-mode auto-close`: posts duplicate comment and closes duplicate issue.
- In `--dry-run`, duplicate mode is simulation-only (deterministic output, no mutation).

Dependency behavior:

- `gh` is required for:
  - main PR mode
  - `--create-pr`
  - `--auto-edit`
  - duplicate actions outside dry-run
- Pure local dry-run (`--dry-run` without online actions) works without `gh`.

Exit codes (automation contract):

- `0`: Success
- `2`: Usage/arguments error
- `3`: Missing dependency (`gh`/`jq`)
- `4`: Git context error (e.g. missing branch context)
- `5`: No extracted PR data in dry-run
- `6`: Partial GitHub enrichment blocked PR creation

Regression tests:

```bash
bash tests/generate_pr_description_regression.sh
```

Troubleshooting:

- See `.github/documentation/pr_generator_troubleshooting.md`.
