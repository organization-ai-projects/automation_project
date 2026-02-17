# Documentation des scripts

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient l'ensemble des scripts d'automatisation du projet, organises par domaine de responsabilite.

## Role dans le projet

Ce repertoire fournit l'infrastructure d'automatisation, les workflows de versioning et des bibliotheques utilitaires reutilisables pour tout le workspace.
Il interagit principalement avec:

- Le systeme de build du projet et la structure du workspace
- Git pour le versioning et GitHub pour les operations distantes
- Les pipelines CI/CD et les quality gates
- Les modules du projet qui ont besoin d'automatisation

## Structure du repertoire

```plaintext
scripts/
├── automation/       # Automatisation du workspace (build, checks, CI, securite, releases)
│   └── git_hooks/    # Hooks Git pour valider les commits et les pushs
├── common_lib/       # Bibliotheques utilitaires reutilisables sourcees par d'autres scripts
├── versioning/       # Workflows de controle de version (branches, PR, releases)
└── README.md         # Ce fichier (version EN canonique)
```

## Contenu

- `README.md`: Documentation principale des scripts.
- `TOC.md`: Index de documentation des scripts.
- `automation/`: Scripts d'automatisation transverses au projet.
- `common_lib/`: Bibliotheques shell reutilisables.
- `versioning/`: Workflows de versioning et synchronisation.

## Principe d'organisation

Les scripts sont organises par **responsabilite metier**, pas par outil:

- **`automation/`** - Taches automatiques transverses (build, tests, audits, releases, quality checks)
  - **`automation/git_hooks/`** - Validations automatiques via hooks Git (format de commit, pre-push)
- **`common_lib/`** - Fonctions reutilisables sourcees par les autres scripts
- **`versioning/`** - Workflows Git/GitHub de gestion des versions
  - **`versioning/file_versioning/orchestrators/execute/`** - Scripts interactifs executes par un humain
  - **`versioning/file_versioning/orchestrators/read/`** - Scripts non interactifs composables par d'autres scripts

## Ajouter un nouveau script

1. **Comprendre le besoin** - Quelle tache est automatisee?
2. **Choisir le bon domaine** - `automation`, `versioning` ou `common_lib`?
3. **Verifier l'existant** - Aligner la nouvelle entree avec la structure deja en place
4. **Documenter** - Ajouter le script dans le README de son domaine et dans l'index pertinent

## Documentation

Pour les details complets:

- Lire le `README` de chaque domaine
- Lire l'index: `scripts/TOC.md`

## Reference rapide

| Objectif                            | Repertoire                                  | Exemple                                    |
| ----------------------------------- | ------------------------------------------- | ------------------------------------------ |
| Valider commits et qualite          | `automation/git_hooks/`                     | `commit-msg`, `pre-push`                   |
| Automatiser build/tests/checks      | `automation/`                               | `build_ui_bundles.sh`, `pre_add_review.sh` |
| Gerer branches, PR et synchroniser  | `versioning/file_versioning/orchestrators/` | `execute/start_work.sh`, bot automation    |
| Reutiliser des fonctions communes   | `common_lib/`                               | `logging.sh`, `command.sh`                 |
