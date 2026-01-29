# Git Hooks

Git hooks personnalisés pour assurer la qualité du code et des commits.

## Hooks disponibles

### `commit-msg`

Valide le format des messages de commit selon les conventions du projet.

**Format attendu:**

```plaintext
<type>(<scope>): <message>
```

ou

```plaintext
<type>: <message>
```

**Types autorisés:**

- `feature`, `feat` - Nouvelle fonctionnalité
- `fix` - Correction de bug
- `fixture` - Données de test ou fixtures
- `doc`, `docs` - Documentation
- `refactor` - Refactorisation
- `test`, `tests` - Tests
- `chore` - Tâches de maintenance

**Exemples valides:**

```bash
feat(auth): add user authentication
fix: resolve null pointer exception
docs(readme): update installation instructions
refactor(api): simplify error handling
```

**Bypass (urgence uniquement):**

```bash
SKIP_COMMIT_VALIDATION=1 git commit -m "emergency fix"
```

### `pre-commit`

Exécute le formatage du code avant chaque commit :

1. **Formatage** : `cargo fmt --all`

Ajoute automatiquement les fichiers formatés au staging.

**Bypass (urgence uniquement):**

```bash
SKIP_PRE_COMMIT=1 git commit -m "message"
```

### `pre-push`

Exécute les vérifications de qualité avant chaque push, **avec détection de scope intelligent** :

1. **Formatage** : `cargo fmt --all --check`
2. **Linting** : `cargo clippy` (seulement sur les crates affectés)
3. **Tests** : `cargo test` (seulement sur les crates affectés)

#### Détection de scope

Le hook analyse les fichiers modifiés et ne teste que les crates concernés :

```plaintext
projects/products/accounts/backend/src/...  → teste accounts-backend
projects/libraries/security/src/...         → teste security
projects/products/core/engine/src/...       → teste engine
```

Si aucune modification n'est détectée, un test complet du workspace est lancé.

**Bypass (urgence uniquement):**

```bash
SKIP_PRE_PUSH=1 git push
```

## Installation

Exécutez le script d'installation :

```bash
./scripts/git_hooks/install_hooks.sh
```

Ce script copie les hooks dans `.git/hooks/` et les rend exécutables.

## Architecture

Les hooks sont :

- **Custom bash scripts** - Cohérents avec l'infrastructure existante
- **Autonomes** - Pas de dépendances externes (npm, cargo-husky, etc.)
- **Bypassables** - Variables d'environnement pour les urgences
- **Informatifs** - Messages clairs sur ce qui est vérifié et comment corriger
- **Intelligents** - Détection de scope pour éviter les tests inutiles

## Maintenance

Pour mettre à jour les hooks après modification :

```bash
./scripts/git_hooks/install_hooks.sh
```

Pour désactiver temporairement un hook :

```bash
# Renommer le hook dans .git/hooks/
mv .git/hooks/pre-push .git/hooks/pre-push.disabled
```

Pour le réactiver :

```bash
mv .git/hooks/pre-push.disabled .git/hooks/pre-push
```
