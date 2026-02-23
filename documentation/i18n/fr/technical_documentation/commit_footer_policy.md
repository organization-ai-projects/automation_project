# Politique des footers de commit

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

Ce document definit un usage strict des mots-cles de footer de commit et des references d'issues.

## Objectif

Conserver un suivi d'issues deterministe et eviter les comportements de fermeture ambigus.

## Mots-cles de footer

- `Closes #<issue>` : utiliser quand le changement ferme completement une issue.
- `Part of #<issue>` : utiliser quand un commit contribue a une issue plus large sans la fermer.
- `Reopen #<issue>` : utiliser pour bloquer explicitement la fermeture quand une issue a ete fermee trop tot.

## Regles

- Utiliser au maximum un mot-cle de fermeture par issue referencee dans le meme commit.
- Ne pas melanger mot-cle de fermeture et non-fermeture pour la meme issue dans un commit.
- Utiliser `Part of` quand le travail est partiel.
- Utiliser `Reopen` avec la meme issue pour neutraliser explicitement une fermeture.

## Exemples

```text
docs(governance): define branch naming convention

Closes #417
Part of #410
```

```text
fix(scripts/versioning/file_versioning/github): avoid premature close on out-of-sync issue state

Part of #389
Reopen #389
```

## Source de verite

- Ce fichier est la politique de footer de reference.
- `CONTRIBUTING.md` doit referencer cette politique au lieu de dupliquer un wording contradictoire.

## References

- [CONTRIBUTING.md](../../../../CONTRIBUTING.md)
