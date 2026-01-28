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

### `pre-push`

Exécute les vérifications de qualité avant chaque push :

1. **Formatage** : `cargo fmt --all --check`
2. **Linting** : `cargo clippy --all-targets --all-features -- -D warnings`
3. **Tests** : `cargo test --workspace`

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
