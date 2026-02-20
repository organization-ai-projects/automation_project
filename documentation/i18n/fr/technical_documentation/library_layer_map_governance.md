# Gouvernance de la carte des couches de bibliotheques

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

## Objectif

Definir les regles de gouvernance de l'artefact canonique `crate -> layer` du workspace, utilise par les checks stricts de couches.

## Artefact canonique

- Artefact principal: `scripts/checks/layer_map.txt`
- Format: `crate_name=L0|L1|L2|L3|UNMAPPED`
- Scope: crates du workspace sous `projects/libraries/`
- Overlay core gere par le checker (pas de fichier artefact separe).
- Politique de l'overlay core:
  - `layer -> core` autorise
  - `core -> layer` interdit
  - `core -> core` autorise

## Ownership

- Owners principaux: maintainers responsables des checks d'architecture/layering.
- Les contributeurs peuvent proposer des updates via PR, mais une review d'ownership est requise.

## Politique de mise a jour

Toute modification de `layer_map.txt` doit etre explicite et reviewable:

1. Expliquer la raison de chaque mapping de crate modifie.
2. Referencer l'issue/la decision qui justifie le changement.
3. Garder une decision logique par commit quand possible.
4. Eviter les remappings massifs sans plan de migration.

## Exigences de validation

Lors d'une mise a jour de la carte:

1. Lancer l'analyse:

```bash
./scripts/checks/analyze_layer_anomalies.sh --map-file scripts/checks/layer_map.txt
```

1. Confirmer la completude de la carte (aucune bibliotheque workspace manquante).
2. Confirmer l'absence d'entrees malformees (seulement `L0|L1|L2|L3|UNMAPPED`).
3. Capturer les anomalies cles impactees par le changement de mapping dans la description de PR.

## Politique UNMAPPED

- `UNMAPPED` est autorise uniquement comme etat de decision temporaire.
- Les nouvelles crates doivent etre mappees avant activation d'un enforcement strict pour leur chemin.
- Les entrees `UNMAPPED` exigent une issue de suivi avec owner et fenetre de resolution attendue.
- `protocol` et `ui-lib` sont des mappings finalises et ne doivent pas revenir a `UNMAPPED`.

## Alignement exceptions et whitelist

- Les decisions de map ne remplacent pas la gouvernance de whitelist.
- Si une crate mappee a encore besoin temporairement d'une arete interdite, utiliser une entree de whitelist gouvernee.
- Chaque exception de whitelist doit inclure raison, owner, et date de revue/expiration.

## Checklist de review

Avant de merger une mise a jour de map, les reviewers doivent confirmer:

1. Les changements de mapping sont justifies et scopes.
2. Le modele de couches reste coherent avec `library_layer_taxonomy.md`.
3. Les hypotheses de comportement checker dans `library_layer_boundaries.md` restent valides.
4. Les issues de migration liees sont reliees et actionnables.
