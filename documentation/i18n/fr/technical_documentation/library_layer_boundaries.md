# Frontieres des couches de bibliotheques

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

Ce document definit la frontiere de dependances enforcee automatiquement en CI.

## Regle enforcee

- Les crates sous `projects/libraries/` ne doivent pas dependre des crates sous `projects/products/`.

Direction autorisee :

- `projects/products/*` -> `projects/libraries/*`

Direction interdite :

- `projects/libraries/*` -> `projects/products/*`

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
