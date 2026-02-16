# Convention de nommage des branches

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

Ce document est la source de verite pour le nommage des branches dans ce repository.

## Objectif

Conserver des noms de branches predictibles, lisibles et faciles a classer dans l'automatisation et les revues.

## Format requis

```text
<type>/<short-kebab-description>
```

- `type` est obligatoire et doit etre l'une des valeurs autorisees ci-dessous.
- `short-kebab-description` doit etre en minuscules et utiliser `-` comme separateur.

## Types autorises

- `feat` ou `feature`
- `fix`
- `refactor`
- `docs` ou `doc`
- `test` ou `tests`
- `chore`
- `fixture`
- `sync`
- `enhancement`

## Pattern sous-PR

Quand une branche de sous-PR est necessaire, utiliser :

```text
<owner>/sub-pr-<parent-pr-number>
```

Exemple :

```text
remi-bezot/sub-pr-378
```

## Patterns interdits

- Noms avec majuscules (exemple `Fix/Parser`).
- Espaces ou underscores (exemple `fix/json_parser`).
- Prefixe de type absent (exemple `parser-fix`).
- Noms generiques (exemple `tmp`, `test`, `update`).

## Exemples

Valides :

- `fix/scripts-breaking-detection`
- `docs/template-conventions`
- `enhancement/pr-description-hardening`
- `remi-bezot/sub-pr-378`

Invalides :

- `Fix/scripts-breaking-detection`
- `fix/scripts_breaking_detection`
- `scripts-breaking-detection`
- `tmp`

## References

- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
