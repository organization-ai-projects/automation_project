# PR Generator Troubleshooting Playbook

This playbook covers common failure modes for:

- `scripts/versioning/file_versioning/github/generate_pr_description.sh`

## Quick Checks

1. Verify branch context:
   - `git branch --show-current`
   - `git status --short`
2. Verify tool dependencies:
   - `gh --version`
   - `jq --version`
   - `git --version`
3. Verify GitHub auth (for online modes):
   - `gh auth status`

## Common Symptoms

### No extracted PRs in dry-run

Symptom:

- Warning about extraction on `base..head`.
- Empty "Key Changes".

Checks:

- Validate refs used by default or override explicitly:
  - `--base dev --head <branch>`
- Inspect commit range:
  - `git log --oneline <base>..<head>`

### "gh is missing" dependency error

Symptom:

- Exit code `3`.
- Message: `la commande 'gh' est introuvable`.

Notes:

- Pure local dry-run works without `gh`.
- `gh` is required for:
  - main PR mode
  - `--create-pr`
  - `--auto-edit` / `--refresh-pr`
  - duplicate actions outside dry-run

### Existing PR already exists when using --auto

Symptom:

- GitHub reports an existing PR and create step does not return a new URL.

Action:

- Regenerate and update existing PR body:
  - `--dry-run --auto-edit <PR_NUMBER> --yes`
  - or alias: `--dry-run --refresh-pr <PR_NUMBER> --yes`

### Issues section is empty or incomplete

Checks:

- Ensure commit/PR body contains closure keywords:
  - `Closes #...`
  - optional neutralization override: `Reopen #...` on the same issue
- Confirm references are issue numbers (not PR numbers).
- Re-run with trace:
  - `--debug`

### Duplicate handling did not close issues

Checks:

- Confirm mode:
  - `--duplicate-mode safe` (comment only)
  - `--duplicate-mode auto-close` (comment + close)
- In `--dry-run`, duplicate mode is simulation-only.

## Recommended Debug Command

```bash
bash scripts/versioning/file_versioning/github/generate_pr_description.sh \
  --dry-run \
  --base dev \
  --head "$(git branch --show-current)" \
  --debug \
  pr_description_debug.md
```

## Exit Codes

- `0`: success
- `2`: usage error
- `3`: missing dependency
- `4`: git context error
- `5`: no extracted PR data for auto-create flow
- `6`: partial enrichment blocked PR creation
