# Documentation versioning

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les scripts de gestion de version a differents niveaux.

## Role dans le projet

Ce repertoire couvre la gestion de version pour les operations fichier et les operations systeme.
Il interagit principalement avec:

- Les repositories Git (local + distant)
- Les APIs GitHub via `gh`
- Les workflows CI/CD (synchronisation et gestion PR)

## Structure du repertoire

```plaintext
versioning/
├── README.md (ce fichier, version EN canonique)
├── TOC.md
└── file_versioning/          # Workflows de controle de version
    ├── git/                  # Operations Git pures
    ├── github/               # Operations GitHub CLI (reserve)
    ├── orchestrators/        # Orchestration de workflows
    │   ├── execute/          # Points d'entree interactifs
    │   └── read/             # Composants non interactifs
    └── scripts racine        # Operations hybrides Git + GitHub
```

## Fichiers

- `README.md`: Ce document.
- `TOC.md`: Index de la documentation versioning.
- `file_versioning/`: Workflows de versioning.

## Portee

Les scripts ici gerent:

- **Versioning au niveau fichiers** - Branches, commits, PR, synchronisation
- **Versioning systeme** - Releases (versioning semantique, changelog, tags)

## Structure interne

- **`file_versioning/`** - Workflows Git/GitHub de versioning
  - `git/` - Operations Git pures
  - `github/` - Operations GitHub CLI (reserve, actuellement vide)
  - Racine `file_versioning/` - Operations hybrides Git + GitHub

Voir `file_versioning/README.md` pour le detail.

## Scripts actuels

### File versioning (`file_versioning/`)

- **Gestion de branches** - Creation, suppression, nettoyage, protection
- **Automatisation PR** - Creation de PR, suivi CI, synchronisation labels
- **Synchronisation repository** - Sync `main <-> dev`

## Ajouter un script de versioning

1. **Workflow de controle de version?** -> `file_versioning/`
2. **Gestion de releases/version semantique?** -> niveau `versioning/`
3. **Git uniquement?** -> `file_versioning/git/`
4. **GitHub uniquement?** -> `file_versioning/github/`
5. **Mix Git + GitHub?** -> racine `file_versioning/`

Documenter dans:

- Le README concerne (ce fichier ou `file_versioning/README.md`)
- La documentation technique des scripts de versioning
