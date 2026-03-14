# Table des matieres

Langue : [English](../../TOC.md) | **Francais**

Ce document fournit une vue d'ensemble des fichiers de documentation de ce dossier.

## Documentation

- [README.md](../../README.md) : documentation principale des scripts d'automatisation

## Sous-repertoires

- [git_hooks/TOC.md](../../git_hooks/TOC.md) : hooks Git pour validation de commit et controle qualite

## Scripts

- [check_script_integrity.sh](../../check_script_integrity.sh) : verification de l'integrite des scripts (sources/ROOT_DIR/entrypoints)
- [git_add_command_override.sh](../../git_add_command_override.sh) : override shell optionnel pour git add
- [git_add_guard.sh](../../git_add_guard.sh) : ajout securise avec regles de split
- [SCRIPT_WORKFLOWS.md](../../SCRIPT_WORKFLOWS.md) : inventaire canonique des workflows et entrypoints
- [tests/critical_workflows_regression.sh](../../tests/critical_workflows_regression.sh) : suite de regression des workflows critiques
- [tests/enforcer_shell_contract_regression.sh](../../tests/enforcer_shell_contract_regression.sh) : garde-fou enforcer sur la structure shell

## Commandes Rust migrees

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
