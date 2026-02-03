# Contribuer

**English version**: [CONTRIBUTING.md](CONTRIBUTING.md)

## Vue d'ensemble

Les contributions sont les bienvenues. Gardez les modifications ciblées, suivez la structure existante et liez votre travail à une issue lorsque c'est possible.

Pour la documentation détaillée du flux de travail, voir le [TOC des scripts](scripts/TOC.fr.md).

---

## Commencer

1. Forkez le dépôt (contributeurs externes) ou clonez directement (membres de l'équipe).
2. Créez une branche à partir de `dev` en suivant la convention de nommage ci-dessous.
3. Effectuez vos modifications avec des commits clairs et ciblés.
4. Ouvrez une pull request vers `dev`.

---

## Nommage des branches

Utilisez des noms de branches descriptifs avec un préfixe de type :

```text
<type>/<description-courte>
```

**Types** :

- `feat/` – Nouvelle fonctionnalité
- `fix/` – Correction de bug
- `doc/` – Modifications de documentation
- `refactor/` – Refactorisation de code
- `test/` – Ajout ou mise à jour de tests
- `chore/` – Tâches de maintenance

**Exemples** :

- `feat/user-authentication`
- `fix/json-parser-panic`
- `doc/update-api-docs`

---

## Directives de commit

- Gardez les commits petits et centrés sur un seul changement.
- Utilisez des messages de commit clairs et descriptifs.
- Référencez les issues le cas échéant : `fix: Résoudre la panique dans le parseur (#42)`

Voir [TOC des scripts Git](scripts/versioning/file_versioning/git/fr/TOC.md) pour plus de détails.

---

## Directives de Pull Request

### Avant d'ouvrir une PR

1. Rebasez votre branche sur le dernier `dev` :

   ```bash
   git fetch origin
   git rebase origin/dev
   ```

2. Exécutez les tests localement :

   ```bash
   cargo test --workspace
   ```

3. Vérifiez le formatage et les lints :

   ```bash
   cargo fmt --check
   cargo clippy --workspace
   ```

### Exigences de PR

- **Titre** : Utilisez la même convention que pour les noms de branches (`feat:`, `fix:`, etc.)
- **Description** : Expliquez quoi et pourquoi, liez les issues associées
- **Taille** : Gardez les PRs ciblées ; divisez les grands changements en PRs plus petites
- **Tests** : Incluez des tests pour les nouvelles fonctionnalités

Voir [TOC de Versioning](scripts/versioning/file_versioning/fr/TOC.md) pour plus de détails.

---

## Processus de revue de code

1. Toutes les PRs nécessitent au moins une approbation avant la fusion.
2. Traitez les commentaires des reviewers rapidement.
3. Redemandez une revue après avoir apporté des modifications.
4. Résolvez toutes les conversations avant la fusion.

### Attentes de revue

- Les revues se font généralement dans un délai de 1 à 2 jours ouvrables.
- Soyez respectueux et constructif dans vos commentaires.
- Concentrez-vous sur la correction, la clarté et la maintenabilité.

---

## Exigences de test

- Toutes les nouvelles fonctionnalités doivent inclure des tests.
- Les corrections de bugs doivent inclure un test de régression lorsque c'est possible.
- Exécutez la suite de tests complète avant de soumettre :

  ```bash
  cargo test --workspace
  ```

- Assurez-vous que la CI passe sur votre PR.

---

## Qualité du code

- Préférez la gestion explicite des erreurs aux paniques.
- Maintenez la documentation à jour avec les modifications de code.
- Évitez de casser les APIs publiques sans un chemin de migration clair.
- Suivez le style de code et les modèles existants.
- Utilisez `cargo fmt` pour le formatage et `cargo clippy` pour les lints.

---

## Questions ?

Si vous avez des questions sur la contribution, ouvrez une issue ou contactez les mainteneurs.
