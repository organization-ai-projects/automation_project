# Registre

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

- [Retour au TOC technique](TOC.md)

## Objectif

Le registre (`.automation_project/registry.json`) est la **source de verite** pour :

- la liste des produits
- les emplacements de bundles UI
- les identites backend
- les versions et la compatibilite de schema

Il est **explicite**, versionne, et mis a jour par l'Engine lors de la lecture des `metadata.ron` de chaque produit.
Le registre n'est pas une configuration ecrite a la main. C'est une vue compilee et normalisee des metadonnees produit, produite par l'Engine.

## Relation avec `metadata.ron`

- Chaque produit embarque un fichier `metadata.ron`.
- L'Engine charge `metadata.ron`, le valide, puis ecrit/met a jour le registre.
- L'UI centrale lit le registre pour afficher produits et UIs.

Cela signifie que la decouverte est toujours pilotee par les metadonnees, pas par scan du filesystem.

## Format des IDs (ProtocolId)

Tous les identifiants stockes dans le registre sont des chaines hex **ProtocolId** (32 caracteres hex) :

- IDs produit
- IDs des entrypoints UI
- IDs de domaines
- IDs backend (si applicable)

Cela ne change pas la structure du registre, seulement le format des valeurs.

## Champs du registre (niveau haut)

Le registre inclut au minimum :

- liste des produits
- chemins des bundles UI
- identites backend
- metadonnees de schema/version

La structure exacte peut evoluer, mais les IDs restent des chaines hex ProtocolId.

## Regles

- Ne pas deduire les produits par scan du workspace.
- Mettre a jour le registre uniquement via le chargement de metadonnees par l'Engine.
- Le registre est autoritaire pour l'UI centrale.

## Documents lies

- [Metadata](metadata.md)
- [Architecture](../../technical_documentation/ARCHITECTURE.md)
- [Products and Workspace Components](../../technical_documentation/projects/projects_products.md)
