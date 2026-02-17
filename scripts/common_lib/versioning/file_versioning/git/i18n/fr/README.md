# Documentation des utilitaires Git

Langue : [English](../../README.md) | **Francais**

Fonctions utilitaires reutilisables pour les operations Git.

## Role dans le projet

Ce repertoire fournit des abstractions bas niveau pour les operations Git communes: validation repository, gestion de branches, validation working tree, commits, staging et synchronisation.
Il interagit principalement avec:

- L'interface Git en ligne de commande
- La configuration et l'etat du repository
- Le working tree et la zone de staging
- Les branches locales/distantes
- L'historique de commits

## Structure du repertoire

```plaintext
git/
├── branch.sh           # Utilitaires de gestion des branches
├── commit.sh           # Operations de commit
├── repo.sh             # Validation du repository
├── staging.sh          # Operations de staging/index
├── synch.sh            # Utilitaires de synchronisation
└── working_tree.sh     # Validation de l'etat du working tree
```

## Fichiers

- `README.md`: Ce document (version EN canonique).
- `branch.sh`: Gestion des branches.
- `commit.sh`: Operations de commit.
- `repo.sh`: Validation du repository.
- `staging.sh`: Operations staging/index.
- `synch.sh`: Synchronisation.
- `working_tree.sh`: Validation working tree.

## Portee

Ce repertoire contient les fonctions Git de base reutilisees par d'autres scripts:

- Operations repository et branches
- Validation de l'etat du working tree
- Operations de commit
- Operations de staging/index
- Synchronisation locale/distante

## Modules actuels

### repo.sh

Validation du repository:

- `require_git_repo()` - verifier qu'on est dans un repository Git

### branch.sh

Gestion des branches:

- `branch_exists_local()` - verifier l'existence d'une branche locale
- `branch_exists_remote()` - verifier l'existence d'une branche distante
- `is_protected_branch()` - verifier si une branche est protegee
- `get_current_branch()` - recuperer la branche courante
- `require_non_protected_branch()` - imposer une branche non protegee
- `save_last_deleted_branch()` - sauvegarder le nom de la derniere branche supprimee
- `get_last_deleted_branch()` - recuperer ce nom

### working_tree.sh

Validation de l'etat local:

- `require_clean_tree()` - imposer un working tree propre
- `has_untracked_files()` - verifier la presence de fichiers non suivis
- `is_working_tree_dirty()` - verifier la presence de modifications locales

### staging.sh

Operations sur l'index:

- `git_add_all()` - ajouter tous les changements
- `git_add_files()` - ajouter des fichiers precis
- `git_reset_all()` - desindexer tous les changements
- `git_reset_files()` - desindexer des fichiers precis
- `git_status()` - afficher le statut Git complet
- `git_status_short()` - afficher le statut court

### commit.sh

Operations de commit:

- `git_commit()` - creer un commit
- `git_commit_amend()` - amender le commit precedent
- `git_commit_amend_message()` - amender uniquement le message
- `has_staged_changes()` - verifier la presence de changements indexes
- `has_unstaged_changes()` - verifier la presence de changements non indexes

### synch.sh

Synchronisation:

- `git_fetch_prune()` - fetch distant + prune des branches supprimees

## Ajouter un utilitaire Git

1. Identifier la categorie fonctionnelle (branch/repo/staging/commit/...)
2. Garder un fichier focalise sur un seul domaine
3. Ecrire une fonction reutilisable et robuste
4. Documenter ici et dans le fichier shell concerne
