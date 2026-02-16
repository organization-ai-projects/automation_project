# Standard de template documentation

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

Ce fichier est la source canonique des regles de structure des templates documentaires dans ce repository.

## Objectif

Definir des roles coherents pour `README.md` et `TOC.md`, et lister les sections obligatoires vs optionnelles afin d'ajouter de la documentation sans ambiguite.

## Roles des fichiers

### `README.md`

- But : expliquer un dossier/module comme point d'entree humain.
- Focus : contexte, usage et conventions.
- Ne doit pas dupliquer un index complet de fichiers deja maintenu dans `TOC.md`.

Sections obligatoires :

- `Purpose` ou `Role`
- `Scope`
- `Key Components` (ou resume equivalent)
- `Navigation` (ou liens vers le TOC pertinent)

Sections optionnelles :

- `Conventions`
- `Usage`
- `Troubleshooting`
- `References`

### `TOC.md`

- But : fournir un index navigable des fichiers de documentation d'un dossier.
- Focus : liens + descriptions courtes.
- Ne doit pas dupliquer le narratif explicatif deja present dans `README.md`.

Sections obligatoires :

- `Documentation` (ou `Documentation Files`) avec liens et descriptions courtes
- `Navigation` (lien de retour vers l'index parent)

Sections optionnelles :

- `Related Documentation`
- `Related Governance Docs`
- `Templates`
- `Workflows`

## Regle de localisation canonique

- Ce fichier (`documentation/technical_documentation/en/documentation_template_standard.md`) est la source unique de verite pour la structure des templates.
- Les autres documents peuvent resumer ou referencer ce standard, mais ne doivent pas redefinir de regles contradictoires.

## Regle d'adoption

- Les nouvelles zones de documentation doivent inclure les deux :
  - un `README.md` (contexte et usage),
  - un `TOC.md` (index et navigation),
  sauf si la zone ne contient qu'un seul fichier sans sous-structure.

- Les zones existantes doivent converger progressivement vers cette structure lors de leurs modifications.
