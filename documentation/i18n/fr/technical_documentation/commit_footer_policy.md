# Politique des footers de commit

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

Ce document definit un usage strict des mots-cles de footer de commit et des references d'issues.

## Objectif

Conserver un suivi d'issues deterministe et eviter les comportements de fermeture ambigus.

## Mots-cles de footer

- `Closes #<issue>` : utiliser quand le changement ferme completement une issue.
- `Fixes #<issue>` : utiliser pour des corrections de bug qui resolvent un comportement incorrect.
- `Resolves #<issue>` : utiliser quand ce n'est pas strictement un bug fix mais que l'issue est entierement traitee.
- `Related to #<issue>` : utiliser pour un contexte lie sans fermeture.
- `Part of #<issue>` : utiliser quand un commit contribue a une issue plus large sans la fermer.

## Regles

- Utiliser au maximum un mot-cle de fermeture par issue referencee dans le meme commit.
- Ne pas melanger mot-cle de fermeture et non-fermeture pour la meme issue dans un commit.
- Preferer `Closes` pour les issues documentation/gouvernance/process.
- Preferer `Fixes` pour les defects confirmes.
- Utiliser `Related to` ou `Part of` quand le travail est partiel.

## Exemples

```text
docs(governance): define branch naming convention

Closes #417
Related to #410
```

```text
fix(scripts/versioning/file_versioning/github): avoid false positive breaking detection

Fixes #389
Part of #403
```

## Source de verite

- Ce fichier est la politique de footer de reference.
- `CONTRIBUTING.md` doit referencer cette politique au lieu de dupliquer un wording contradictoire.

## References

- [CONTRIBUTING.md](../../../../CONTRIBUTING.md)
