# Documentation file_versioning

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les scripts de workflow de controle de version, de gestion de branches et d'operations GitHub.

## Role dans le projet

Ce repertoire orchestre les workflows Git/GitHub: gestion de branches, automation PR et synchronisation du repository.
Il interagit principalement avec:

- Git (branches, commits, push)
- GitHub (`gh` CLI pour PR/issues/labels)
- Les workflows CI/CD
- Les developpeurs via les orchestrateurs interactifs

## Structure du repertoire

```plaintext
file_versioning/
├── README.md (ce fichier, version EN canonique)
├── TOC.md
├── orchestrators/              # Orchestration des workflows
│   ├── execute/                # Points d'entree interactifs (user-facing)
│   │   ├── start_work.sh       # Workflow principal: sync, issues, branche
│   │   ├── ci_watch_pr.sh      # Suivi CI d'une PR
│   │   └── labels_sync.sh      # Synchronisation des labels
│   └── read/                   # Composants non interactifs (API layer)
│       ├── synch_main_dev_ci.sh      # Sync main->dev par bot
│       ├── check_priority_issues.sh  # Liste des issues prioritaires
│       └── create_pr.sh              # Creation de PR
├── git/                        # Operations Git pures
└── github/                     # Operations GitHub-only
    └── generate_pr_description.sh
```

## Fichiers

- `README.md`: Ce document.
- `TOC.md`: Index des scripts file_versioning.
- `orchestrators/`: Orchestration des workflows.
- `git/`: Scripts Git purs.
- `github/`: Scripts GitHub-only.

## Architecture: Execute vs Read

### `orchestrators/execute/` - points d'entree executables

Workflows complets lances directement par les humains:

- `start_work.sh` - point d'entree principal
- `ci_watch_pr.sh` - suivi CI d'une PR
- `labels_sync.sh` - synchronisation des labels

### `orchestrators/read/` - composants non interactifs

Scripts appeles par les executeurs ou par l'automation bot/CI:

- `synch_main_dev_ci.sh`
- `create_pr.sh`
- `check_priority_issues.sh`

### `git/` - utilitaires Git bas niveau

Scripts utilisant uniquement `git` (creation/suppression/push/nettoyage de branches, etc.).

## Pourquoi cette architecture?

1. Separation claire entre orchestration interactive et logique reutilisable
2. Workflow guidant l'utilisateur vers les bonnes etapes
3. Reduction des erreurs (sync/checks centralises)
4. Navigation plus simple dans le repertoire

## Workflow principal: start_work.sh

```bash
./scripts/versioning/file_versioning/orchestrators/execute/start_work.sh
```

Ce workflow orchestre:

1. Fetch des branches `dev` et `main`
2. Affichage des issues prioritaires
3. Creation d'une branche de travail conforme

## Apres merge PR: cleanup_after_pr.sh

```bash
./scripts/versioning/file_versioning/git/cleanup_after_pr.sh
```

Attention: en cas d'echec du safe delete, ce script peut forcer la suppression locale (`git branch -D`).

## Composants actuels

### Composants Git-only (`git/`)

- `create_branch.sh`
- `delete_branch.sh`
- `push_branch.sh`
- `clean_branches.sh`
- `clean_local_gone.sh`
- `create_work_branch.sh`
- `finish_branch.sh`
- `add_commit_push.sh`
- `create_after_delete.sh`
- `cleanup_after_pr.sh`

### Composants GitHub (`github/`)

- `generate_pr_description.sh`

### Composants hybrides (`orchestrators/read`)

- `check_priority_issues.sh`
- `synch_main_dev_ci.sh`
- `create_pr.sh`

## Conventions de nommage de branches

Validees par `create_branch.sh`:

- `feature/` ou `feat/`
- `fix/` ou `fixture/`
- `doc/` ou `docs/`
- `refactor/`
- `test/` ou `tests/`
- `chore/`

Exemples: `feature/user-authentication`, `fix/null-pointer-bug`.

## Ajouter un script

1. Workflow interactif complet? -> `orchestrators/execute/`
2. Composant non interactif reutilisable? -> `orchestrators/read/`
3. Git pur? -> `git/`
4. GitHub pur? -> `github/`
5. Mix Git/GitHub? -> racine `file_versioning/` ou `orchestrators/read/` selon usage
