# Documentation automation

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les scripts d'automatisation transverses au projet.

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
├── build_accounts_ui.sh            # Build du bundle UI accounts
├── build_and_check_ui_bundles.sh   # Build + verification des artefacts UI
├── build_ui_bundles.sh             # Detection + build de tous les bundles UI
├── changed_crates.sh               # Liste les crates modifiees dans un diff
├── check_dependencies.sh           # Detecte dependances obsoletes/manquantes
├── check_merge_conflicts.sh        # Teste les conflits de merge
├── clean_artifacts.sh              # Nettoie les artefacts de build
├── git_add_guard.sh                # Ajout securise avec regles de split
├── pre_add_review.sh               # Pre-check interne avant review
├── pre_push_check.sh               # Validation avant push (checks/tests/conflicts)
├── release_prepare.sh              # Preparation release (version/changelog/tag)
├── setup_hooks.sh                  # Installation des hooks Git
├── sync_docs.sh                    # Synchronisation documentation (placeholder)
└── test_coverage.sh                # Generation des rapports de couverture
```

## Fichiers

- `README.md`: Ce document (version EN canonique).
- `git_hooks/`: Hooks Git de validation commit/push.
- `audit_security.sh`: Audit securite des dependances.
- `build_accounts_ui.sh`: Build UI accounts.
- `build_and_check_ui_bundles.sh`: Build + verification artefacts UI.
- `build_ui_bundles.sh`: Decouverte + build de tous les bundles UI.
- `changed_crates.sh`: Detection des crates modifiees.
- `check_dependencies.sh`: Verification des dependances.
- `check_merge_conflicts.sh`: Detection des conflits de merge.
- `clean_artifacts.sh`: Nettoyage des artefacts.
- `git_add_guard.sh`: Ajout securise avec regles de split.
- `pre_add_review.sh`: Verification avant review interne.
- `pre_push_check.sh`: Validation pre-push.
- `release_prepare.sh`: Preparation release.
- `setup_hooks.sh`: Installation des hooks Git.
- `sync_docs.sh`: Synchronisation de la documentation (placeholder).
- `test_coverage.sh`: Rapport de couverture.

## Ajouter un script d'automatisation

1. **Il agit sur tout le repository?** -> Il va ici.
2. **C'est un workflow de versioning?** -> Il va dans `versioning/`.
3. **C'est une utilitaire reutilisable?** -> Il va dans `common_lib/`.

Documenter la nouvelle entree dans:

- Ce `README`
- La documentation technique des scripts
