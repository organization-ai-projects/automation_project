# Scripts d'Automatisation

## Vue d'ensemble

Dans la volonté de faciliter et structurer le travail de l'équipe, des scripts d'automatisation ont été créés pour gérer les tâches répétitives et les workflows critiques du projet.

Ces scripts couvrent plusieurs domaines :

- **Automatisation générale** : Build UI, synchronisation documentation, validation de bundles
- **Gestion des versions** : Gestion Git (branches, commits, synchronisation), workflows GitHub (pull requests, issues)
- **Bibliothèques communes** : Utilitaires réutilisables (logging, opérations fichiers, commandes, réseau)

## Documentation des Scripts

**La documentation complète et pratique sur l'utilisation des scripts se trouve dans le répertoire `scripts/` à la racine du projet.**

Pour consulter :

- **[README Principal des Scripts](../../scripts/README.md)** : Vue d'ensemble de l'organisation et philosophie
- **[Scripts d'Automatisation](../../scripts/automation/README.md)** : Documentation des scripts automation (build UI, sync docs, etc.)
- **[Scripts de Versioning](../../scripts/versioning/README.md)** : Documentation des scripts Git et GitHub
- **[Bibliothèques Communes](../../scripts/common_lib/README.md)** : Documentation des utilitaires réutilisables

## Principes et Standards

Les scripts respectent les principes suivants :

1. **Robustesse** : Tous les scripts utilisent `set -euo pipefail` pour une gestion stricte des erreurs
2. **Modularité** : Les fonctions communes sont centralisées dans `scripts/common_lib/`
3. **Documentation** : Chaque script est documenté avec son usage, ses paramètres et des exemples
4. **Maintenabilité** : Code clair avec logging cohérent et messages d'erreur explicites
5. **Sécurité** : Validation des entrées, gestion des credentials, opérations atomiques

## Workflows et Conventions

Ce répertoire (`technical_documentation/`) contient les guides conceptuels sur les workflows d'équipe et les conventions :

- **[Workflows Git](versioning/file_versioning/git/)** : Conventions et processus pour le versioning
- **[Workflows GitHub](versioning/file_versioning/github/)** : Conventions pour les pull requests, issues, etc.
- **[Automatisation de Documentation](automation/)** : Processus de génération et synchronisation

Ces documents décrivent **pourquoi** et **comment** travailler en équipe, tandis que `scripts/` décrit **comment utiliser** les outils pratiques.

## Séparation des Responsabilités

| Emplacement                | Contenu                                                   | Objectif                             |
| -------------------------- | --------------------------------------------------------- | ------------------------------------ |
| `technical_documentation/` | Workflows, conventions, philosophie, guides conceptuels   | Comprendre les processus d'équipe    |
| `scripts/`                 | Documentation pratique des scripts, utilisation, exemples | Utiliser les outils d'automatisation |

## Note Importante

**Source Unique de Vérité** : Pour toute information sur l'utilisation pratique d'un script (paramètres, options, exemples), consultez toujours la documentation dans `scripts/`. C'est la seule source maintenue pour l'usage pratique des scripts.

Les documents dans `technical_documentation/` peuvent référencer les scripts mais ne doivent jamais dupliquer leur documentation d'utilisation.
