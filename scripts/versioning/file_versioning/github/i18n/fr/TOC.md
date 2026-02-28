# Table des matieres

Langue : [English](../../TOC.md) | **Francais**

Ce document fournit une vue d'ensemble des fichiers de documentation de ce dossier.

## Documentation

- [README.md](../../README.md) : documentation principale des scripts GitHub CLI

## Scripts

- [auto_link_parent_issue.sh](../../auto_link_parent_issue.sh) : auto-liaison parent/enfant depuis le champ `Parent:` du corps d'issue
- [auto_add_closes_on_dev_pr.sh](../../auto_add_closes_on_dev_pr.sh) : enrichissement automatique des PR vers `dev` avec des lignes `Closes #...` gerees
- [generate_pr_description.sh](../../generate_pr_description.sh) : generation de descriptions de PR merge structurees
- [issue_done_in_dev_status.sh](../../issue_done_in_dev_status.sh) : gestion du label `done-in-dev` apres merge dev et fermeture d'issue
- [parent_issue_guard.sh](../../parent_issue_guard.sh) : garde de fermeture parent/enfant et resume de statut
- [lib/classification.sh](../../lib/classification.sh) : helpers de classification/actions issue
- [lib/rendering.sh](../../lib/rendering.sh) : helpers de rendu des sections et titres PR
- [tests/generate_pr_description_regression.sh](../../tests/generate_pr_description_regression.sh) : matrice de regression CLI
- [tests/auto_add_closes_on_dev_pr_regression.sh](../../tests/auto_add_closes_on_dev_pr_regression.sh) : tests de regression de l'enrichissement auto des `Closes #...` sur PR vers `dev`
- [tests/issue_done_in_dev_status_regression.sh](../../tests/issue_done_in_dev_status_regression.sh) : tests de regression du cycle de label `done-in-dev`

## Navigation

- [Retour au TOC file_versioning](../../TOC.md)
