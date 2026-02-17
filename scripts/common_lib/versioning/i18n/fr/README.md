# Documentation des utilitaires versioning

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les fonctions reutilisables specifiques aux workflows de controle de version.

## Role dans le projet

Ce repertoire fournit des abstractions reutilisables pour les operations de versioning, principalement autour de la gestion de repository Git, des branches et de l'etat du working tree.
Il interagit principalement avec:

- Git
- Les scripts d'automatisation et de versioning
- L'etat/historique du repository
- La gestion des branches et commits

## Structure du repertoire

```plaintext
versioning/
└── file_versioning/     # Utilitaires de versioning au niveau fichier
    └── git/             # Operations specifiques Git
```

## Fichiers

- `README.md`: Ce document (version EN canonique).
- `file_versioning/`: Utilitaires de versioning fichier.

## Portee

Ces utilitaires couvrent:

- Gestion du repository et des branches Git
- Validation de l'etat du working tree
- Operations de commit et staging
- Synchronisation locale/distante

## Structure actuelle

- **`file_versioning/`** - utilitaires de versioning fichier
  - `git/` - operations Git pures

Pour le detail des utilitaires Git, voir `file_versioning/git/README.md`.

## Ajouter un utilitaire versioning

1. **Specifique Git?** -> `file_versioning/git/`
2. **Autre outil de versioning?** -> nouveau sous-repertoire dedie
3. **Logique vraiment generique?** -> evaluer un placement dans `core/`

Documenter le changement dans le README du niveau concerne.
