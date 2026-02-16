# Contribuer

Langue : [English](./CONTRIBUTING.md) | **Français**

## Vue d'ensemble

Les contributions sont les bienvenues. Gardez des changements ciblés, respectez la structure existante, et liez votre travail à une issue quand c'est pertinent.

Pour la documentation détaillée du workflow, voir le [TOC scripts](./scripts/TOC.md).

---

## Prérequis

Installez et configurez ces outils avant de contribuer :

- `git` (dernière version stable)
- `rustup` + toolchain Rust `stable` (épinglée dans `rust-toolchain.toml`)
- Composants Rust : `rustfmt`, `clippy`
- `node` (LTS active recommandée)
- `pnpm` (dernière stable, via Corepack recommandé)
- GitHub CLI `gh` (requis pour les scripts d'automatisation issue/PR)

Vérification rapide :

```bash
git --version
rustup --version
cargo --version
rustfmt --version
cargo clippy --version
node --version
pnpm --version
gh --version
```

---

## Démarrage

1. Forkez le dépôt (contributeurs externes) ou clonez directement (membres de l'équipe).
2. Créez une branche depuis `dev` en suivant la convention ci-dessous.
3. Faites vos changements avec des commits clairs et ciblés.
4. Ouvrez une pull request vers `dev`.

---

## Nommage des branches

Utilisez des noms descriptifs avec un préfixe de type :

```text
<type>/<description-courte>
```

**Types** (préfixes acceptés, alias inclus) :

- `feature/` ou `feat/` - Nouvelle fonctionnalité
- `fix/` - Correction de bug
- `fixture/` - Jeux de données ou fixtures de test
- `doc/` ou `docs/` - Documentation
- `refactor/` - Refactorisation
- `test/` ou `tests/` - Ajout/mise à jour de tests
- `chore/` - Maintenance

**Exemples** :

- `feat/user-authentication`
- `feature/user-dashboard`
- `fix/json-parser-panic`
- `fixture/test-data`
- `doc/update-api-docs`
- `docs/add-examples`
- `refactor/simplify-error-handling`
- `test/add-integration-tests`
- `tests/unit-coverage`
- `chore/update-dependencies`

**Note** : le nommage des branches est contrôlé par `create_branch.sh`. Les noms invalides sont rejetés avec un message explicite.

**Source de vérité** : `documentation/technical_documentation/en/branch_naming_convention.md`

---

## Règles de commit

### Format de message de commit (obligatoire)

Tous les messages de commit **doivent** suivre le format conventional commit :

```text
<type>(<scope>): <summary>
```

ou

```text
<type>: <summary>
```

**Types requis** :

- `feature`, `feat` - Nouvelle fonctionnalité
- `fix` - Correction de bug
- `fixture` - Données de test
- `doc`, `docs` - Documentation
- `refactor` - Refactorisation
- `test`, `tests` - Tests
- `chore` - Maintenance

**Exemples** :

- `feat(auth): add user authentication`
- `feat(ci,scripts): add workflows and sync script`
- `fix: resolve null pointer exception`
- `docs(readme): update installation instructions`
- `refactor(api): simplify error handling`
- `test: add unit tests for validator`
- `chore: update dependencies`

**Scope** (optionnel mais requis pour les changements sous `projects/libraries` et `projects/products`) : module/composant impacté.

### Mapping des scopes à partir des fichiers touchés

Quand les changements touchent le code produit/librairie, le scope doit mapper les chemins touchés :

- `projects/libraries/<library_name>/...` -> `projects/libraries/<library_name>`
- `projects/products/.../<product_name>/ui/...` -> `projects/products/<product_name>/ui`
- `projects/products/.../<product_name>/backend/...` -> `projects/products/<product_name>/backend`
- `projects/products/.../<product_name>/...` (fichiers racine du produit) -> `projects/products/<product_name>`

Pour les changements transverses, utilisez plusieurs scopes (séparés par des virgules) ou `workspace` seulement si un scope unique n'est pas représentatif.

**Résumé** : description claire et concise du changement.

**Contrôles** :

- `add_commit_push.sh` valide les messages de commit
- Les hooks git valident aussi les messages (installés via `scripts/automation/git_hooks/install_hooks.sh`)
- Les messages non conformes sont rejetés avec des erreurs explicites
- Bypass uniquement en urgence :
  - `--no-verify` avec `add_commit_push.sh`
  - `SKIP_COMMIT_VALIDATION=1 git commit -m "message"` en git direct

### Règles complémentaires

- Gardez des commits petits et focalisés.
- Référencez les issues si applicable : `fix: resolve panic in parser (#42)`
- Utilisez des mots-clés de footer explicites (`Closes`, `Fixes`, `Resolves`, `Related to`, `Part of`) selon `documentation/technical_documentation/en/commit_footer_policy.md`.

Voir le [TOC des scripts Git](scripts/versioning/file_versioning/git/TOC.md) pour plus de détails.

---

## Règles de Pull Request

### Avant d'ouvrir une PR

1. Rebasez sur le dernier `dev` :

   ```bash
   git fetch origin
   git rebase origin/dev
   ```

2. Lancez les tests localement :

   ```bash
   cargo test --workspace
   ```

3. Vérifiez formatage et lints :

   ```bash
   cargo fmt --check
   cargo clippy --workspace
   pnpm run lint-md
   ```

### Créer une PR

Le script `create_pr.sh` automatise la création de PR et **lance les tests automatiquement** avant création.

```bash
bash scripts/versioning/file_versioning/orchestrators/read/create_pr.sh
```

**Contrôle des tests** :

- Par défaut, `create_pr.sh` lance `cargo test --workspace` avant de créer la PR
- Si les tests échouent, la PR n'est pas créée
- Pour ignorer les tests (non recommandé), utilisez `--skip-tests` :

  ```bash
  bash scripts/versioning/file_versioning/orchestrators/read/create_pr.sh --skip-tests
  ```

**Options complémentaires** :

- `--base <branch>` : branche cible (par défaut : `dev`)
- `--title <title>` : titre personnalisé
- `--body <body>` : description personnalisée
- `--draft` : crée une PR en brouillon

### Exemple de description de PR

Utilisez une structure concrète et révisable :

```md
## Why
- Fixes intermittent failure in account audit flush tests.

## What
- Stabilize test timing using deterministic flush trigger.
- Keep production behavior unchanged.

## Validation
- cargo test -p accounts-backend --bin accounts-backend
- cargo test --workspace

Closes #<issue-number>
```

### Exigences PR

- **Titre** : utilisez la convention des types (`feat:`, `fix:`, etc.)
- **Description** : expliquez le quoi et le pourquoi, liez les issues concernées
- **Taille** : gardez des PR focalisées ; découpez les trop grosses
- **Tests** : ajoutez des tests pour toute nouvelle fonctionnalité

Voir [Versioning TOC](scripts/versioning/file_versioning/TOC.md) pour plus de détails.

---

## Référence des scripts

Scripts les plus utilisés dans ce guide :

- `scripts/versioning/file_versioning/git/create_branch.sh` : crée une branche et valide sa convention de nommage.
- `scripts/versioning/file_versioning/git/add_commit_push.sh` : ajoute, valide le message, commit et push.
- `scripts/versioning/file_versioning/orchestrators/read/create_pr.sh` : crée une PR vers `dev` (avec tests par défaut).
- `scripts/automation/git_hooks/install_hooks.sh` : installe les hooks git du dépôt.

---

## Processus de revue de code

1. Toute PR nécessite au moins une approbation avant merge.
2. Traitez rapidement les retours de revue.
3. Redemandez une revue après modifications.
4. Résolvez toutes les conversations avant fusion.

### Attentes de revue

- Les revues prennent en général 1 à 2 jours ouvrés.
- Gardez un ton respectueux et constructif.
- Priorisez correction, clarté et maintenabilité.

---

## Exigences de test

- Toute nouvelle fonctionnalité doit inclure des tests.
- Une correction de bug doit ajouter un test de régression si possible.
- Exécutez la suite complète avant soumission :

  ```bash
  cargo test --workspace
  ```

- Vérifiez que la CI passe sur la PR.

### Style d'import dans les tests

**Utilisez des imports explicites dans les modules de test.** Évitez `use super::*`; préférez des `use crate::` ou `use super::` ciblés.

**Bon exemple** :

```rust
#[cfg(test)]
mod tests {
    use crate::some_module::MyStruct;
    use crate::some_module::MyEnum;

    #[test]
    fn test_something() {
        let s = MyStruct::new();
    }
}
```

**À éviter** :

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
    }
}
```

**Raison** : des imports explicites améliorent la lisibilité et rendent les dépendances plus claires en revue.

---

## Qualité du code

- Préférez la gestion d'erreurs explicite aux panics.
- Maintenez la documentation à jour avec le code.
- Évitez de casser les API publiques sans plan de migration clair.
- Respectez les patterns et styles existants.
- Utilisez `cargo fmt` pour le formatage et `cargo clippy` pour les lints.
- Utilisez `pnpm run lint-md` pour le lint Markdown et `pnpm run lint-md-fix` pour l'auto-fix.

---

## FAQ

### Pourquoi mon commit est-il rejeté ?

Le message de commit ne respecte probablement pas le format requis, ou un hook a détecté un échec de check. Utilisez `add_commit_push.sh` pour un flux guidé.

### Quand utiliser `Closes #...` vs `Related to #...` ?

Utilisez `Closes #...` uniquement si le travail de la branche résout entièrement l'issue. Utilisez `Related to #...` pour un lien contextuel non entièrement résolu ici.

### Pourquoi le push échoue alors que mes tests passent en local ?

Les règles de protection peuvent imposer PR obligatoire et CI verte avant merge.

### Je peux commit directement sur `main` ou `dev` ?

Non. Travaillez depuis une branche dédiée puis ouvrez une PR vers `dev`.

### Existe-t-il un guide en anglais ?

Oui : [CONTRIBUTING.md](./CONTRIBUTING.md).

### Quelle est la stratégie i18n pour la doc de contribution ?

L'approche actuelle est bilingue (EN/FR) pour les points d'entrée majeurs. Toute nouvelle section pertinente doit être répercutée dans les deux versions.

## Questions ?

Si vous avez des questions, ouvrez une issue ou contactez les maintainers.
