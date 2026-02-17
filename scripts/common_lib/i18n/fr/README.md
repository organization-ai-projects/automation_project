# Documentation common_lib

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les bibliotheques de fonctions shell reutilisables sourcees par les autres scripts.

## Role dans le projet

Ce repertoire fournit des fonctions utilitaires et des abstractions communes pour standardiser les operations reutilisees dans les scripts.
Il interagit surtout avec:

- Les scripts `automation/` et `versioning/`
- Les utilitaires systeme de base (logs, fichiers, reseau)
- L'interface en ligne de commande Git
- La manipulation/validation de chaines

## Structure du repertoire

```plaintext
common_lib/
├── core/                               # Utilitaires de base pour tous les scripts
│   ├── command.sh                      # Execution et validation de commandes
│   ├── file_operations.sh              # Operations sur fichiers/repertoires
│   ├── logging.sh                      # Fonctions de logs coherentes
│   ├── network_utils.sh                # Utilitaires reseau
│   └── string_utils.sh                 # Utilitaires de manipulation de chaines
└── versioning/                         # Utilitaires de controle de version
    └── file_versioning/                # Versioning au niveau des fichiers
        └── git/                        # Operations specifiques Git
            ├── branch.sh               # Gestion des branches
            ├── commit.sh               # Operations de commit
            ├── repo.sh                 # Validation de repository
            ├── staging.sh              # Operations de staging/index
            ├── synch.sh                # Utilitaires de synchronisation
            └── working_tree.sh         # Etat du working tree
```

## Fichiers

- `README.md`: Ce document (version EN canonique).
- `core/`: Utilitaires de base partages.
- `versioning/`: Utilitaires de versioning.

## Principe d'organisation

Les utilitaires sont organises par **portee et outil**:

- **`core/`** - Utilitaires transverses (logging, commandes, fichiers, chaines, reseau)
- **`versioning/file_versioning/git/`** - Utilitaires Git specialises

## Portee

Les scripts de ce repertoire:

- Definissent des fonctions reutilisables
- Ne doivent pas etre executes directement (ils sont `source`d)
- Restent focalises sur un domaine unique
- Sont importes via `source "$ROOT_DIR/scripts/common_lib/core/logging.sh"`

## Utilitaires de base (`core/`)

- `logging.sh` - Logs uniformes (`info`, `warn`, `die`)
- `command.sh` - Outils de commande (`command_exists`, `require_cmd`, `retry_command`)
- `file_operations.sh` - Aides fichiers/repertoires (`file_exists`, `dir_exists`, `backup_file`)
- `string_utils.sh` - Manipulation de chaines (`to_upper`, `to_lower`, `trim`, `contains`)
- `network_utils.sh` - Helpers reseau (`url_reachable`, `download_file`)

## Utilitaires Git (`versioning/file_versioning/git/`)

- `repo.sh` - Validation repository (`require_git_repo`)
- `branch.sh` - Operations sur branches
- `working_tree.sh` - Etat du working tree
- `staging.sh` - Operations de staging/index
- `commit.sh` - Operations de commit
- `synch.sh` - Synchronisation (`fetch --prune`)

## Ajouter un nouvel utilitaire

1. Identifier le domaine fonctionnel
2. Choisir le bon fichier (ou en creer un nouveau si necessaire)
3. Garder un fichier focalise sur un seul domaine
4. Documenter dans ce `README` et dans le fichier shell

## Exemple d'utilisation

```bash
#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Source des utilitaires
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
source "$ROOT_DIR/scripts/common_lib/core/command.sh"

# Utilisation
require_cmd "git"
info "Demarrage du deploiement..."
die "Une erreur est survenue"
```
