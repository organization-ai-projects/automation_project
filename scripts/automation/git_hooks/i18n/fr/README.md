# Documentation des hooks Git

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les hooks Git personnalises pour garantir la qualite du code et des commits.

## Role dans le projet

Ce repertoire applique automatiquement des regles de qualite a des points critiques du workflow Git (commit et push).
Il interagit principalement avec:

- Le workflow Git local
- Les outils Rust (`cargo fmt`, `cargo clippy`, `cargo test`)
- Les conventions de message de commit
- La detection des fichiers/scopes modifies

## Structure du repertoire

```plaintext
git_hooks/
├── commit-msg          # Valide le format du message de commit
├── pre-commit          # Lance le formatage avant commit
├── prepare-commit-msg  # Genere un sujet de commit automatiquement
├── pre-push            # Lance les checks qualite avant push
└── install_hooks.sh    # Installe les hooks dans .git/hooks/
```

## Fichiers

- `README.md`: Ce document (version EN canonique).
- `commit-msg`: Validation format commit.
- `pre-commit`: Formatage avant commit.
- `prepare-commit-msg`: Generation automatique du sujet de commit.
- `pre-push`: Quality checks avant push.
- `install_hooks.sh`: Installation des hooks dans `.git/hooks/`.

## Hooks disponibles

### `commit-msg`

Valide le message de commit selon la convention du projet.

**Format attendu:**

```plaintext
<type>(<scope>): <message>
```

ou

```plaintext
<type>: <message>
```

**Types autorises:**

- `feature`, `feat`
- `fix`
- `fixture`
- `doc`, `docs`
- `refactor`
- `test`, `tests`
- `chore`

**Exemples valides:**

```bash
feat(auth): add user authentication
fix: resolve null pointer exception
docs(readme): update installation instructions
refactor(api): simplify error handling
docs(.github): add default PR template
```

**Bypass (urgence uniquement):**

```bash
SKIP_COMMIT_VALIDATION=1 git commit -m "emergency fix"
```

### `pre-commit`

Checks executes avant chaque commit:

1. Blocage des commits directs sur `dev` et `main`
2. Formatage Rust: `cargo fmt --all`

Les fichiers reformates sont automatiquement restages.

**Bypass (urgence uniquement):**

```bash
SKIP_PRE_COMMIT=1 git commit -m "message"
ALLOW_PROTECTED_BRANCH_COMMIT=1 git commit -m "message"
```

### `prepare-commit-msg`

Genere automatiquement un sujet de commit conventionnel quand le message est vide.

Sources utilisees:

1. Prefixe de branche (`feat/`, `fix/`, `docs/`, etc.) pour deduire le type
2. Fichiers stages pour deduire les scopes requis et le fallback de type
3. Slug de branche pour produire une description lisible

Le hook ne remplace pas:

- Les messages explicites via `git commit -m`
- Les commits merge/squash/amend
- Les templates deja non vides

**Bypass (urgence uniquement):**

```bash
SKIP_PREPARE_COMMIT_MSG=1 git commit
```

### `pre-push`

Checks executes avant chaque push (selectifs):

1. `cargo fmt --all --check`
2. `cargo clippy` (crates impactees)
3. `cargo test` (crates impactees)
4. Mode docs/scripts-only: skip des checks Rust + verification shell legere

#### Logique de selection

- Si seuls docs/scripts/workflows sont modifies: checks Rust skips.
- Sinon: utilisation des scopes de commit pour cibler les crates.
- Si scopes invalides/manquants: fallback sur checks workspace complets.

**Bypass (urgence uniquement):**

```bash
SKIP_PRE_PUSH=1 git push
```

## Installation

```bash
./scripts/automation/git_hooks/install_hooks.sh
```

Le script copie les hooks dans `.git/hooks/` et les rend executables.

## Architecture

Les hooks sont:

- En bash (coherent avec l'infrastructure existante)
- Autonomes (pas de dependance externe type npm/cargo-husky)
- Bypassables en urgence
- Explicites sur les checks et corrections
- Optimises pour eviter les checks inutiles (scope detection)

## Maintenance

Pour mettre a jour les hooks apres modification:

```bash
./scripts/automation/git_hooks/install_hooks.sh
```

Desactivation temporaire:

```bash
mv .git/hooks/pre-push .git/hooks/pre-push.disabled
```

Reactivation:

```bash
mv .git/hooks/pre-push.disabled .git/hooks/pre-push
```
