# Documentation des automatisations Git

Langue : [English](../../README.md) | **Francais**

Les operations Git sont maintenant gerees par `versioning_automation git ...`.

## Workflow

Pour la synchronisation apres merge PR, voir:

- [sync_after_pr.md](../../sync_after_pr.md)

## Commandes canoniques

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
