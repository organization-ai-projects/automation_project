# Table des matieres

Langue : [English](../../TOC.md) | **Francais**

Ce document fournit une vue d'ensemble des fichiers de documentation de ce dossier.

## Documentation

- [README.md](../../README.md) : documentation principale des scripts d'automatisation

## Sous-repertoires

- [git_hooks/TOC.md](../../git_hooks/TOC.md) : hooks Git pour validation de commit et controle qualite

## Scripts

- [audit_issue_status.sh](../../audit_issue_status.sh) : audit des issues ouvertes vs references commits
- [check_script_integrity.sh](../../check_script_integrity.sh) : verification de l'integrite des scripts (sources/ROOT_DIR/entrypoints)
- [git_add_command_override.sh](../../git_add_command_override.sh) : override shell optionnel pour git add
- [git_add_guard.sh](../../git_add_guard.sh) : ajout securise avec regles de split
- [release_prepare.sh](../../release_prepare.sh) : preparation release (version/changelog/tag)
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
