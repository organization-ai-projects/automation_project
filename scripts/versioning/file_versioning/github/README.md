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
└── generate_pr_description.sh
```

## Files

- `README.md`: This file.
- `TOC.md`: Documentation index for GitHub-only scripts.
- `generate_pr_description.sh`: Generate structured merge PR descriptions from PR metadata and/or local git history.

## Scope

Scripts in this directory should:

- Focus on GitHub PR/issue workflows
- Prefer `gh` data when available
- Provide resilient fallbacks when GitHub API is unavailable

## Scripts

### `generate_pr_description.sh`

Generates a ready-to-paste PR description (e.g., `dev -> main`) by analyzing child PRs and resolved issues.
Supports both PR-number mode (GitHub-enriched) and local dry-run mode.

Usage:

```bash
bash generate_pr_description.sh [--keep-artifacts] MAIN_PR_NUMBER [OUTPUT_FILE]
bash generate_pr_description.sh --dry-run [--base BRANCH] [--head BRANCH] [--create-pr] [--allow-partial-create] [--yes] [OUTPUT_FILE]
bash generate_pr_description.sh --auto [--base BRANCH] [--head BRANCH] [--yes]
```

Key options:

- `--dry-run`: Generate from local history (default base: `dev`, default head: current branch).
- `--base`, `--head`: Explicit branch range for dry-run extraction.
- `--create-pr`: In dry-run mode, optionally create the PR with the generated body.
- `--allow-partial-create`: Allow PR creation even if GitHub enrichment is incomplete.
- `--yes`: Non-interactive confirmation when `--create-pr` is used.
- `--auto`: RAM-first flow (`--dry-run` + `--create-pr`) with in-memory body.
- `--keep-artifacts`: Keep extracted PR/issue intermediate files.

Exit codes (automation contract):

- `0`: Success
- `2`: Usage/arguments error
- `3`: Missing dependency (`gh`/`jq`)
- `4`: Git context error (e.g. missing branch context)
- `5`: No extracted PR data in dry-run
- `6`: Partial GitHub enrichment blocked PR creation
