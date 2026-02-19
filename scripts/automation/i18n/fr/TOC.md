# Table des matieres

Langue : [English](../../TOC.md) | **Francais**

Ce document fournit une vue d'ensemble des fichiers de documentation de ce dossier.

## Documentation

- [README.md](../../README.md) : documentation principale des scripts d'automatisation

## Sous-repertoires

- [git_hooks/TOC.md](../../git_hooks/TOC.md) : hooks Git pour validation de commit et controle qualite

## Scripts

- [audit_security.sh](../../audit_security.sh) : audit securite des dependances
- [build_accounts_ui.sh](../../build_accounts_ui.sh) : build du bundle UI accounts
- [build_and_check_ui_bundles.sh](../../build_and_check_ui_bundles.sh) : build et verification des artefacts
- [build_ui_bundles.sh](../../build_ui_bundles.sh) : decouverte et build de tous les bundles UI
- [changed_crates.sh](../../changed_crates.sh) : liste les crates touchees dans un diff
- [check_dependencies.sh](../../check_dependencies.sh) : verification dependances obsoletes/manquantes
- [check_merge_conflicts.sh](../../check_merge_conflicts.sh) : test de merge pour conflits
- [clean_artifacts.sh](../../clean_artifacts.sh) : nettoyage des artefacts de build
- [git_add_guard.sh](../../git_add_guard.sh) : ajout securise avec regles de split
- [pre_add_review.sh](../../pre_add_review.sh) : revue interne pre-add (format, clippy, tests)
- [pre_push_check.sh](../../pre_push_check.sh) : validation pre-push (checks, tests, conflits)
- [release_prepare.sh](../../release_prepare.sh) : preparation release (version/changelog/tag)
- [setup_hooks.sh](../../setup_hooks.sh) : installation des hooks git
- [sync_docs.sh](../../sync_docs.sh) : synchronisation de documentation
- [test_coverage.sh](../../test_coverage.sh) : generation rapports de couverture de tests
