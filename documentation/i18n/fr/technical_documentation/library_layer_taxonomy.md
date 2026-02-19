# Taxonomie des couches de bibliotheques du workspace

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

## Objectif

Definir le modele strict et deterministe de couches pour les bibliotheques du workspace.

## Modele de couches

- `L0 Foundation` : primitives et utilitaires techniques ultra-generiques.
- `L1 Technical Specialization` : adaptateurs/specialisations techniques construits sur `L0` (pas encore metier).
- `L2 Domain` : bibliotheques metier et APIs/contrats orientes domaine.
- `L3 Orchestration` : seule couche autorisee a composer/croiser plusieurs domaines.

## Regles strictes de dependance (adjacent-only)

- `L0` ne doit dependre d'aucune crate du workspace.
- `L1` peut dependre uniquement de `L0`.
- `L2` peut dependre uniquement de `L1`.
- `L3` peut dependre uniquement de `L2`.
- Les dependances montantes sont interdites.
- Les dependances laterales sont interdites par defaut (`L1 -> L1`, `L2 -> L2`, `L3 -> L3`), sauf whitelist explicite.

## Contrat de comportement du checker

- Les controles de couches evaluent uniquement les aretes entre crates du workspace.
- Les crates externes sont ignorees pour les regles de direction.
- Les dependances `path`/workspace sont traitees comme les dependances workspace nommees.
- Le scope cible `dependencies` et `build-dependencies` par defaut.
- Les `dev-dependencies` sont exclues par defaut.

## Guide de placement des couches

- Les contrats purement techniques/partages vont en `L1`.
- Les contrats orientes domaine vont en `L2`.
- `L3` reste orchestration uniquement et doit consommer les contrats `L2`, pas les details internes `L1`.

## Decisions de placement finalisees

Les decisions suivantes sont finalisees et doivent etre traitees comme politique d'architecture:

- `protocol` est fixe en `L1` (couche de contrats techniques).
- `ui-lib` (crate sous `projects/libraries/ui`) est fixe en `L2`.
- Les crates techniques partagees sont fixees comme suit:
  - `L0`: `common_time`, `common_calendar`, `common_binary`, `common_parsing`, `common_tokenize`, `hybrid_arena`, `ast_core`, `ast_macros`, `pjson_proc_macros`, `protocol_macros`.
  - `L1`: `common`, `common_json`, `common_ron`, `command_runner`, `protocol`.
- `ai` reste en `L3` et doit consommer uniquement des contrats/facades `L2`.
  Cible de migration: supprimer les aretes directes `L3 -> L1` (notamment vers `common_json` et `protocol`) via des frontieres `L2`.

## Impact de migration (vague courante)

- Les anomalies directes `L3 -> L1` et `L2 -> L0` sont de la dette de migration, pas une ambiguite de politique.
- Les refactors de suivi doivent aligner le code sur ce placement final sans redefinir les couches.

## Gouvernance des exceptions

- Les exceptions doivent etre explicites, minimales et temporaires.
- Chaque entree de whitelist doit inclure:
  - une raison,
  - un owner,
  - une date de revue/expiration.
