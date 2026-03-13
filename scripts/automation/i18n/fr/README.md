# Documentation automation

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les entrypoints shell actifs pour l'automatisation transverse.
La logique canonique de versioning/GitHub est migree en Rust dans
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
├── audit_security.sh               # Audit securite des dependances
├── audit_issue_status.sh           # Audit des issues ouvertes vs references commits
├── build_accounts_ui.sh            # Build du bundle UI accounts
├── build_and_check_ui_bundles.sh   # Build + verification des artefacts UI
├── build_ui_bundles.sh             # Detection + build de tous les bundles UI
├── changed_crates.sh               # Liste les crates modifiees dans un diff
├── check_dependencies.sh           # Detecte dependances obsoletes/manquantes
├── check_merge_conflicts.sh        # Teste les conflits de merge
├── clean_artifacts.sh              # Nettoie les artefacts de build
├── git_add_guard.sh                # Ajout securise avec regles de split
├── pre_add_review.sh               # Pre-check interne avant review
├── release_prepare.sh              # Preparation release (version/changelog/tag)
└── test_coverage.sh                # Generation des rapports de couverture
```

## Fichiers

- `README.md`: Ce document (version EN canonique).
- `git_hooks/`: Hooks Git de validation commit/push.
- `audit_security.sh`: Audit securite des dependances.
- `audit_issue_status.sh`: Audit des issues ouvertes vs references commits sur un range de branches.
- `build_accounts_ui.sh`: Build UI accounts.
- `build_and_check_ui_bundles.sh`: Build + verification artefacts UI.
- `build_ui_bundles.sh`: Decouverte + build de tous les bundles UI.
- `changed_crates.sh`: Detection des crates modifiees.
- `check_dependencies.sh`: Verification des dependances.
- `check_merge_conflicts.sh`: Detection des conflits de merge.
- `clean_artifacts.sh`: Nettoyage des artefacts.
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
