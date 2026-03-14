# Documentation automation

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les entrypoints shell actifs pour l'automatisation transverse.
La logique canonique de versioning et des automatisations migrees est en Rust dans
`tools/versioning_automation`.

## Role dans le projet

Ce repertoire automatise les operations repository-wide: build, tests, quality checks, audits de securite et preparation des releases.
Il interagit principalement avec:

- Le workspace Cargo et le systeme de build
- L'infrastructure de test et les outils de couverture
- Les hooks Git et quality gates
- Les scanners de securite et la gestion des dependances
- Les outils de release et de changelog

## Structure du repertoire

```plaintext
automation/
├── git_hooks/                      # Hooks Git pour la validation commit/push
│   ├── commit-msg                  # Verifie le format du message de commit
│   ├── pre-commit                  # Lance le formatage avant commit
│   ├── prepare-commit-msg          # Genere automatiquement le sujet de commit
│   ├── pre-push                    # Lance les checks qualite avant push
│   └── install_hooks.sh            # Installe les hooks Git
├── audit_issue_status.sh           # Audit des issues ouvertes vs references commits
├── git_add_guard.sh                # Ajout securise avec regles de split
├── pre_add_review.sh               # Pre-check interne avant review
├── release_prepare.sh              # Preparation release (version/changelog/tag)
└── test_coverage.sh                # Generation des rapports de couverture
```

## Fichiers

- `README.md`: Ce document (version EN canonique).
- `git_hooks/`: Hooks Git de validation commit/push.
- `audit_issue_status.sh`: Audit des issues ouvertes vs references commits sur un range de branches.
- `git_add_guard.sh`: Ajout securise avec regles de split.
- `pre_add_review.sh`: Verification avant review interne.
- `release_prepare.sh`: Preparation release.
- `test_coverage.sh`: Rapport de couverture.

Hook pre-push canonique: `scripts/automation/git_hooks/pre-push`.

## Ajouter un script d'automatisation

1. **Il agit sur tout le repository?** -> Il va ici.
2. **C'est de la logique de workflow Git/GitHub versioning?** -> Il va dans `tools/versioning_automation` (Rust CLI).
3. **C'est un utilitaire shell reutilisable?** -> Il va dans `scripts/common_lib/`.

Documenter la nouvelle entree dans:

- Ce `README`
- `TOC.md` (obligatoire)
- `SCRIPT_WORKFLOWS.md` si c'est un entrypoint utilisateur

## Commandes Rust migrees

- `versioning_automation automation audit-security`
- `versioning_automation automation build-accounts-ui`
- `versioning_automation automation build-ui-bundles`
- `versioning_automation automation build-and-check-ui-bundles`
- `versioning_automation automation changed-crates [<ref1>] [<ref2>] [--output-format paths]`
- `versioning_automation automation check-dependencies`
- `versioning_automation automation check-merge-conflicts`
- `versioning_automation automation clean-artifacts`
