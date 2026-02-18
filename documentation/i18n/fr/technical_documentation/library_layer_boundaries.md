# Frontieres des couches de bibliotheques

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

Ce document definit la frontiere de dependances enforcee automatiquement en CI.

## Regle actuellement enforcee

- Les crates sous `projects/libraries/` ne doivent pas dependre des crates sous `projects/products/`.

Direction autorisee :

- `projects/products/*` -> `projects/libraries/*`

Direction interdite :

- `projects/libraries/*` -> `projects/products/*`

## Regles strictes cibles (adjacent-only)

Pour les bibliotheques du workspace, la direction des dependances est:

- `L0` -> aucune dependance workspace
- `L1` -> `L0` uniquement
- `L2` -> `L1` uniquement
- `L3` -> `L2` uniquement

Contraintes additionnelles:

- aucune dependance montante
- aucune dependance laterale par defaut (sauf whitelist explicite)
- exceptions explicites, temporaires et gouvernees

## Scope du checker

- Evaluer uniquement les aretes entre crates du workspace.
- Ignorer les aretes vers crates externes pour les regles de couches.
- Traiter les dependances `path`/workspace comme les dependances workspace nommees.
- Inclure `dependencies` et `build-dependencies` par defaut.
- Exclure `dev-dependencies` par defaut.

## Validation

La CI execute :

```bash
./scripts/checks/check_layer_boundaries.sh
```

Le controle utilise `cargo metadata` pour inspecter les aretes de dependance du workspace et echoue lorsqu'une arete interdite est detectee.

## Guide de correction

Si la CI signale une arete interdite :

1. Deplacer la logique partagee dans une crate appropriee sous `projects/libraries/`.
2. Faire consommer cette crate partagee par les crates produits.
3. Supprimer le couplage direct aux produits dans les crates bibliotheques.
