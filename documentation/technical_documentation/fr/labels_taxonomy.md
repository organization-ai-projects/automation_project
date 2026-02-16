# Taxonomie des labels (Issues et Pull Requests)

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

Ce document definit comment les labels sont utilises sur les Issues et Pull Requests.

## Source de verite

- Les definitions de labels vivent dans `.github/labels.json`.
- Script de synchronisation :
  - `bash scripts/versioning/file_versioning/orchestrators/execute/labels_sync.sh`

## Regles de nommage

- Minuscules uniquement.
- Utiliser `kebab-case` ou des chemins scopes existants (`projects/...`, `tools/...`).
- Un label doit avoir une signification claire.
- Pas de synonymes avec le meme scope et la meme intention.

## Familles de labels

- Type : `bug`, `fix`, `enhancement`, `feature`, `refactor`, `chore`, `documentation`, `testing`, `security`.
- Workflow : `automation`, `automation-failed`, `review`, `sync_branch`.
- Scope : `projects/...`, `tools/...`, `workspace`, `scripts`.
- Triage : `high priority`, `question`, `duplicate`, `invalid`, `wontfix`, `help wanted`, `good first issue`.
- Cible artefact : `issue`, `pull-request`.

## Recommandations d'usage

- Issues :
  - Utiliser un label de type et les labels de scope/workflow pertinents.
  - Utiliser les labels de triage seulement si necessaire.
  - Optionnel : utiliser `issue` quand on suit explicitement du travail process issue.
- Pull Requests :
  - Reprendre les labels type et scope dominants du travail resolu.
  - Utiliser `pull-request` pour les changements process/template/automation de PR.
  - Eviter l'inflation de labels ; garder seulement ceux qui ameliorent routage/revue.

## Regles de clarification (intention non ambigue)

- `bug` : issue de signalement d'un probleme/defect.
- `fix` : changements code/config qui implementent une correction.
- `enhancement` : amelioration incrementale.
- `feature` : nouvelle capacite.

Cette separation evite de melanger classification d'issue et intention d'implementation.
