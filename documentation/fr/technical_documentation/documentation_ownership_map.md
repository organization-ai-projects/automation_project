# Carte de responsabilites documentation

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

Cette carte definit les responsabilites d'ownership des zones majeures de documentation.

## Objectif

Reduire les zones documentaires orphelines et expliciter les attentes de maintenance.

## Tableau d'ownership

| Zone de documentation | Role proprietaire principal | Role proprietaire de secours | Attente de maintenance |
| --- | --- | --- | --- |
| `README.md` (racine repo) | Repository Maintainer | Release Maintainer | Mis a jour sur changements de structure/process |
| `CONTRIBUTING.md` | Repository Maintainer | Code Review Maintainer | Mis a jour sur changements de workflow/policy |
| `documentation/` | Documentation Maintainer | Repository Maintainer | Mis a jour lors des deplacements/splits de docs techniques |
| `.github/documentation/` | Governance Maintainer | Repository Maintainer | Mis a jour lors des changements de process/template GitHub |
| `.github/workflows/documentation/` | CI Maintainer | Repository Maintainer | Mis a jour lors des changements de comportement workflow |
| `scripts/**/README.md` et `scripts/**/TOC.md` | Script Owner | Repository Maintainer | Mis a jour lors des changements CLI/contrat/comportement |
| `projects/**/README.md` et `projects/**/documentation/TOC.md` | Product/Library Owner | Repository Maintainer | Mis a jour lors des changements de comportement/interface module |

## Regles de maintenance

- L'auteur d'un changement de comportement met a jour la documentation impactee dans la meme PR.
- L'ownership manquant pour une nouvelle zone documentaire doit etre defini avant merge.
- Si l'ownership est incertain, le role par defaut est `Repository Maintainer` jusqu'a delegation explicite.

## Decouvrabilite

- Ce fichier est reference depuis `documentation/TOC.md`.
- Les conventions specifiques gouvernance restent sous `.github/documentation/`.
