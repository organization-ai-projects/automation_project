# Politique de structure et migration documentation EN/FR

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

## Objectif

Definir des regles deterministes et maintenables pour la documentation bilingue dans le repository.

## Source canonique

- L'anglais (`en`) est la source de verite canonique.
- Le contenu francais (`fr`) est une traduction du contenu canonique anglais.
- La traduction doit preserver le sens technique ; aucun refactor d'architecture/contenu n'est autorise pendant la traduction.

## Regles de structure dossiers/fichiers

### Documentation de niveau racine

- Conserver les fichiers canoniques anglais a la racine :
  - `README.md`
  - `CONTRIBUTING.md`
- Ajouter les equivalents francais avec suffixe `.fr.md` :
  - `README.fr.md`
  - `CONTRIBUTING.fr.md`

### Arbres de documentation

Pour chaque arbre documentaire (`documentation/`, `projects/**/documentation/`, `scripts/**`, `tools/**`) :

- Utiliser des arbres paralleles de langue :
  - `en/` pour la documentation canonique anglaise
  - `fr/` pour les traductions francaises
- Conserver une navigation symetrique :
  - les TOC anglais pointent vers les fichiers anglais
  - les TOC francais pointent vers les fichiers francais

## Regles de surete des liens

- Conserver tous les liens relatifs valides apres migration.
- Preferer des liens coherents par langue (`en -> en`, `fr -> fr`).
- Les assets partages ne doivent pas etre dupliques par langue.

## Regles d'assets partages

- Conserver une source unique des assets par zone documentaire.
- Exemple pour docs projet :
  - `documentation/assets/**`
- Exemple pour docs niveau crate :
  - `projects/**/documentation/assets/**`

## Regle d'en-tete de traduction

Chaque fichier francais doit inclure cette ligne en tete :

`This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.`

## Non-objectifs

- Aucun refactor code dans cette policy.
- Aucune traduction des commentaires source ni du rustdoc dans les fichiers `.rs`.
- Aucune duplication d'assets pour chaque langue.

## Phasage de migration

Ordre recommande :

1. Definir et approuver la structure/policy (ce document).
2. Migrer docs racine et entrypoints documentation.
3. Etendre aux docs par crate et docs scripts/tools.
4. Appliquer la validation de liens en CI.
