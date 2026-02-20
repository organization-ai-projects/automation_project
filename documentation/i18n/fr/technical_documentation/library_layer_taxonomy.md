# Taxonomie des couches de bibliotheques du workspace

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

## Objectif

Definir le modele strict et deterministe de couches pour les bibliotheques du workspace.

## Modele de couches

- `L1 Technical Building Blocks` : briques techniques pour composer les bibliotheques de domaine (`L2`), sans logique metier.
- `L2 Domain` : bibliotheques metier et APIs/contrats orientes domaine.
- `L3 Orchestration` : seule couche autorisee a composer/croiser plusieurs domaines.

## Regles strictes de dependance (adjacent-only)

- `L1` ne doit dependre d'aucune autre crate de couche.
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

- `protocol` est fixe comme crate `core/contracts` (pas un niveau de couche).
- `ui-lib` (crate sous `projects/libraries/layers/domain/ui`) est fixe en `L2`.
- Les crates techniques partagees sont fixees comme suit:
  - Elles sont maintenant traitees via `core/foundation` (hors couches numeriques).
- `ai` reste en `L3` et doit consommer uniquement des contrats/facades `L2`.
  - Politique produit: les produits doivent passer par `ai` pour les workflows IA; dependre directement de `neural` ou `symbolic` n'est pas la cible d'architecture.
  Cible de migration: supprimer les aretes directes `L3 -> L1` (notamment vers `common_json`) via des frontieres `L2`.

## Impact de migration (vague courante)

- Les anomalies directes `L3 -> L1` sont de la dette de migration, pas une ambiguite de politique.
- Les refactors de suivi doivent aligner le code sur ce placement final sans redefinir les couches.

## Gouvernance des exceptions

- Les exceptions doivent etre explicites, minimales et temporaires.
- Chaque entree de whitelist doit inclure:
  - une raison,
  - un owner,
  - une date de revue/expiration.
