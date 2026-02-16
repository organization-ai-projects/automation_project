# Taxonomie des couches de bibliotheques du workspace

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

## Objectif

Definir un modele de couches unique pour toutes les crates du workspace et imposer des regles de direction de dependance.

## Modele de couches

- `L0 Foundation` : primitives partagees, helpers de format, macros, parsing/tokenizing/time/calendar, et utilitaires d'infra coeur.
- `L1 Domain` : bibliotheques metier construites sur les crates foundation.
- `L2 Interface` : bibliotheques d'interface et de frontiere (contrats/abstractions niveau UI).
- `L3 Applications` : produits/composants executables (produits stables et instables).

## Regles de direction des dependances

- `L0` peut dependre uniquement de `L0`.
- `L1` peut dependre de `L0` et `L1`.
- `L2` peut dependre de `L0`, `L1` et `L2`.
- `L3` peut dependre de `L0`, `L1` et `L2`.
- Les dependances montantes sont interdites (par exemple : `L0 -> L1/L2/L3`, `L1 -> L2/L3`, `L2 -> L3`).

## Exemples de dependances autorisees

1. `projects/libraries/symbolic` (`L1`) -> `projects/libraries/common` (`L0`)
2. `projects/libraries/ui` (`L2`) -> `projects/libraries/protocol` (`L2`)
3. `projects/products/stable/accounts/backend` (`L3`) -> `projects/libraries/security` (`L1`)

## Exemples de dependances interdites

1. `projects/libraries/common` (`L0`) -> `projects/libraries/symbolic` (`L1`)
2. `projects/libraries/protocol` (`L2`) -> `projects/products/stable/core/engine` (`L3`)
3. `projects/libraries/identity` (`L1`) -> `projects/libraries/ui` (`L2`)

## Mapping crates -> couches du workspace

Chaque membre du workspace est assigne a une seule couche.

| Membre du workspace | Couche |
|---|---|
| `projects/libraries/ai` | `L1 Domain` |
| `projects/libraries/ast_core` | `L0 Foundation` |
| `projects/libraries/ast_macros` | `L0 Foundation` |
| `projects/libraries/command_runner` | `L0 Foundation` |
| `projects/libraries/common` | `L0 Foundation` |
| `projects/libraries/common_binary` | `L0 Foundation` |
| `projects/libraries/common_calendar` | `L0 Foundation` |
| `projects/libraries/common_json` | `L0 Foundation` |
| `projects/libraries/common_parsing` | `L0 Foundation` |
| `projects/libraries/common_ron` | `L0 Foundation` |
| `projects/libraries/common_time` | `L0 Foundation` |
| `projects/libraries/common_tokenize` | `L0 Foundation` |
| `projects/libraries/hybrid_arena` | `L0 Foundation` |
| `projects/libraries/identity` | `L1 Domain` |
| `projects/libraries/neural` | `L1 Domain` |
| `projects/libraries/pjson_proc_macros` | `L0 Foundation` |
| `projects/libraries/protocol` | `L2 Interface` |
| `projects/libraries/protocol_macros` | `L0 Foundation` |
| `projects/libraries/security` | `L1 Domain` |
| `projects/libraries/symbolic` | `L1 Domain` |
| `projects/libraries/ui` | `L2 Interface` |
| `projects/libraries/versioning` | `L1 Domain` |
| `projects/products/stable/accounts/backend` | `L3 Applications` |
| `projects/products/stable/accounts/ui` | `L3 Applications` |
| `projects/products/stable/code_agent_sandbox` | `L3 Applications` |
| `projects/products/stable/core/central_ui` | `L3 Applications` |
| `projects/products/stable/core/engine` | `L3 Applications` |
| `projects/products/stable/core/launcher` | `L3 Applications` |
| `projects/products/stable/core/watcher` | `L3 Applications` |
| `projects/products/stable/varina/backend` | `L3 Applications` |
| `projects/products/stable/varina/ui` | `L3 Applications` |
| `projects/products/unstable/auto_manager_ai` | `L3 Applications` |
| `projects/products/unstable/autonomous_dev_ai` | `L3 Applications` |

## Suivi enforcement

Ce document definit la baseline de policy. Les controles automatiques de frontieres en CI sont geres dans l'issue de suivi d'enforcement dediee.
