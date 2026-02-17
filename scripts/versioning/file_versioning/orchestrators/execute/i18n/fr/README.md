# Documentation des orchestrateurs executables

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les points d'entree principaux lances directement en ligne de commande.

## Role dans le projet

Ce repertoire fournit des workflows interactifs orientant les developpeurs dans les taches de versioning frequentes.
Il interagit principalement avec:

- Les developpeurs (prompts + guidance)
- Les orchestrateurs `../read/`
- Les utilitaires Git `../../git/`
- L'API GitHub via `gh`

## Structure du repertoire

```plaintext
execute/
├── README.md (ce fichier, version EN canonique)
├── TOC.md
├── start_work.sh
├── ci_watch_pr.sh
└── labels_sync.sh
```

## Fichiers

- `README.md`: Ce document.
- `TOC.md`: Index des orchestrateurs execute.
- `start_work.sh`: Workflow principal pour demarrer le travail.
- `ci_watch_pr.sh`: Suivi du statut CI d'une PR.
- `labels_sync.sh`: Synchronisation des labels repository.

## Scripts

### `start_work.sh`

Point d'entree principal pour demarrer un nouveau travail.

Orchestre:

1. Fetch des changements `dev` et `main`
2. Verification des issues prioritaires (`../read/check_priority_issues.sh`)
3. Creation de branche (`../read/git/create_branch.sh`)

```bash
./start_work.sh
```

### `ci_watch_pr.sh`

Surveille l'etat CI d'une PR jusqu'au succes ou echec.

```bash
./ci_watch_pr.sh [pr-number]
```

Variables:

- `POLL_INTERVAL` (defaut: 10s)
- `MAX_WAIT` (defaut: 3600s)

### `labels_sync.sh`

Synchronise les labels GitHub depuis `.github/labels.json`.

```bash
./labels_sync.sh
./labels_sync.sh --prune
./labels_sync.sh --prune path/to/labels.json
```

## Pre-requis

Tous ces scripts requierent:

- `git`
- `gh`
- `jq`
