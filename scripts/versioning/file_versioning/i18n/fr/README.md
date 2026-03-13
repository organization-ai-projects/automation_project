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
├── git/                        # Documentation/contrats des workflows Git
└── github/                     # Operations GitHub-only
    └── ...                     # Entrees canoniques via `versioning_automation`
```

## Fichiers

- `README.md`: Ce document.
- `TOC.md`: Index des scripts file_versioning.
- `git/`: Documentation des workflows Git pour le CLI Rust.
- `github/`: Scripts GitHub-only.

## Architecture runtime

Les entrypoints runtime shell ont ete supprimes.
Les workflows sont lances via `versioning_automation ...` (CLI Rust).

## Pourquoi cette architecture?

1. Separation claire entre orchestration interactive et logique reutilisable
2. Workflow guidant l'utilisateur vers les bonnes etapes
3. Reduction des erreurs (sync/checks centralises)
4. Navigation plus simple dans le repertoire

## Entry points runtime

Les entrypoints runtime sont desormais dans le CLI Rust :

- `versioning_automation automation ...`
- `versioning_automation git ...`
- `versioning_automation pr ...`
- `versioning_automation issue ...`

## Apres merge PR: cleanup-after-pr

```bash
versioning_automation git cleanup-after-pr
```

Attention: en cas d'echec du safe delete, ce script peut forcer la suppression locale (`git branch -D`).

## Composants actuels

### Composants Git (Rust CLI)

- `versioning_automation git create-branch ...`
- `versioning_automation git delete-branch ...`
- `versioning_automation git push-branch ...`
- `versioning_automation git clean-branches ...`
- `versioning_automation git clean-local-gone ...`
- `versioning_automation git create-work-branch ...`
- `versioning_automation git finish-branch ...`
- `versioning_automation git add-commit-push ...`
- `versioning_automation git create-after-delete ...`
- `versioning_automation git cleanup-after-pr ...`

### Composants GitHub (`github/`)

- `versioning_automation pr generate-description ...`

### Composants automation (Rust CLI)

- `versioning_automation automation check-priority-issues ...`
- `versioning_automation automation labels-sync ...`
- `versioning_automation automation ci-watch-pr ...`
- `versioning_automation automation sync-main-dev-ci ...`

## Conventions de nommage de branches

Validees par `versioning_automation git create-branch ...`:

- `feature/` ou `feat/`
- `fix/` ou `fixture/`
- `doc/` ou `docs/`
- `refactor/`
- `test/` ou `tests/`
- `chore/`

Exemples: `feature/user-authentication`, `fix/null-pointer-bug`.

## Ajouter une automation

1. Workflow runtime/logique? -> implementer dans `tools/versioning_automation`.
2. Comportement Git (docs/contrats)? -> `git/`.
3. Comportement GitHub (docs/tests)? -> `github/`.
