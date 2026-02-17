# Documentation des utilitaires file_versioning

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les utilitaires reutilisables pour les workflows de controle de version au niveau fichier.

## Role dans le projet

Ce repertoire abstrait les operations Git utilisees par les workflows de versioning: validation du repository, gestion de branches, etat du working tree, commits et synchronisation.
Il interagit principalement avec:

- L'interface Git en ligne de commande
- Les orchestrateurs de versioning
- L'etat et l'historique du repository
- La zone de staging et le working tree

## Structure du repertoire

```plaintext
file_versioning/
└── git/                   # Utilitaires d'operations Git pures
    ├── branch.sh          # Gestion des branches
    ├── commit.sh          # Operations de commit
    ├── repo.sh            # Validation du repository
    ├── staging.sh         # Operations de staging/index
    ├── synch.sh           # Utilitaires de synchronisation
    └── working_tree.sh    # Etat du working tree
```

## Fichiers

- `README.md`: Ce document (version EN canonique).
- `git/`: Utilitaires Git purs.

## Portee

Ces utilitaires couvrent:

- Operations Git pures (branches, commits, working tree)
- Support des workflows repository/versioning
- Verification de l'etat local/distant

## Structure actuelle

- **`git/`** - utilitaires Git purs
  - Validation du repository
  - Gestion des branches
  - Operations working tree
  - Operations de commit
  - Synchronisation

Pour le detail, voir `git/README.md`.

## Ajouter un utilitaire file_versioning

1. **Operation Git pure?** -> placer dans `git/`
2. **Specifique GitHub?** -> envisager un dossier `github/`
3. **Logique generique de versioning?** -> evaluer un placement au niveau parent `versioning/`

Documenter la nouvelle entree dans le README adapte.
