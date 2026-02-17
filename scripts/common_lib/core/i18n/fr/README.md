# Documentation des utilitaires core

Langue : [English](../../README.md) | **Francais**

Fonctions utilitaires essentielles utilisees par l'ensemble des scripts du projet.

## Role dans le projet

Ce repertoire fournit les fonctions de base transverses: logging, execution de commandes, operations fichiers, manipulation de chaines et utilitaires reseau.
Il interagit principalement avec:

- Tous les scripts d'automatisation et de versioning
- Les commandes systeme et utilitaires shell
- Le systeme de fichiers
- Les ressources reseau

## Structure du repertoire

```plaintext
core/
├── command.sh           # Execution et validation de commandes
├── file_operations.sh   # Operations sur fichiers et repertoires
├── logging.sh           # Fonctions de logs avec format coherent
├── network_utils.sh     # Utilitaires reseau
└── string_utils.sh      # Utilitaires de manipulation de chaines
```

## Fichiers

- `README.md`: Ce document (version EN canonique).
- `command.sh`: Utilitaires d'execution/validation de commandes.
- `file_operations.sh`: Utilitaires de manipulation fichiers/repertoires.
- `logging.sh`: Fonctions de logs de base.
- `network_utils.sh`: Utilitaires reseau.
- `string_utils.sh`: Utilitaires de manipulation de chaines.

## Portee

Ce repertoire contient des utilitaires fondamentaux qui:

- Sont generiques (pas specifiques a Git/versioning)
- Servent de base a tous les autres scripts
- Garantissent un comportement coherent dans le codebase shell

## Modules actuels

### logging.sh

Fonctions de logs standardisees:

- `info()` - message de niveau info
- `warn()` - message de niveau warning
- `die()` - message d'erreur puis sortie

**Usage**: `source "$ROOT_DIR/scripts/common_lib/core/logging.sh"`

### command.sh

Utilitaires de commandes:

- `require_cmd()` - verifier qu'une commande est disponible
- `command_exists()` - tester l'existence d'une commande
- `retry_command()` - relancer une commande avec backoff

**Usage**: `source "$ROOT_DIR/scripts/common_lib/core/command.sh"`

### file_operations.sh

Utilitaires fichiers/repertoires:

- `file_exists()` - verifier l'existence d'un fichier
- `dir_exists()` - verifier l'existence d'un repertoire
- `backup_file()` - sauvegarder un fichier
- `ensure_dir()` - garantir l'existence d'un repertoire

**Usage**: `source "$ROOT_DIR/scripts/common_lib/core/file_operations.sh"`

### string_utils.sh

Utilitaires de chaine:

- `string_to_upper()` - convertir en majuscules
- `string_to_lower()` - convertir en minuscules
- `string_trim()` - supprimer les espaces en debut/fin
- `string_contains()` - verifier la presence d'une sous-chaine

**Usage**: `source "$ROOT_DIR/scripts/common_lib/core/string_utils.sh"`

### network_utils.sh

Utilitaires reseau:

- `url_reachable()` - verifier qu'une URL est joignable
- `download_file()` - telecharger un fichier depuis une URL

**Usage**: `source "$ROOT_DIR/scripts/common_lib/core/network_utils.sh"`

## Ajouter un utilitaire core

1. Verifier qu'il est vraiment generique
2. Le placer dans le bon module (ou en creer un nouveau si necessaire)
3. Garder une responsabilite claire par fichier
4. Documenter a la fois ici et dans le script concerne
