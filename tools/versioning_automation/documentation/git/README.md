# Git Automation Documentation

Git operations are now handled by `versioning_automation git ...`.

## Workflows

For post-merge branch synchronization details, see:

- [sync_after_pr.md](sync_after_pr.md)

## Canonical Commands

- `versioning_automation git create-branch [name] [--remote origin] [--base dev]`
- `versioning_automation git create-work-branch <type> <description> [--remote origin] [--base dev]`
- `versioning_automation git push-branch [--remote origin]`
- `versioning_automation git add-commit-push <message> [--no-verify] [--remote origin]`
- `versioning_automation git delete-branch <name> [--force] [--remote origin] [--base dev]`
- `versioning_automation git finish-branch [name] [--remote origin] [--base dev]`
- `versioning_automation git create-after-delete [--remote origin] [--base dev]`
- `versioning_automation git clean-local-gone [--remote origin]`
- `versioning_automation git clean-branches [--dry-run] [--remote origin] [--base dev]`
- `versioning_automation git cleanup-after-pr [--delete-only] [--remote origin] [--base dev]`

## Commit Message Format

`add-commit-push` enforces:

- `<type>(<scope>): <message>`
- or `<type>: <message>`

Allowed types:

- `feature`, `feat`, `fix`, `fixture`, `doc`, `docs`, `refactor`, `test`, `tests`, `chore`, `perf`
