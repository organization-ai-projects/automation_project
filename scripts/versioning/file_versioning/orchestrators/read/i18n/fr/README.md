# Documentation des composants orchestrateurs read

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les scripts internes appeles par les orchestrateurs executables.

## Role dans le projet

Ce repertoire fournit des composants non interactifs et composables qui implementent la logique metier des workflows de versioning.
Il interagit principalement avec:

- Les orchestrateurs `../execute/`
- Les utilitaires Git `../../git/`
- L'API GitHub via `gh`
- La CI/CD (automation bot)

## Structure du repertoire

```plaintext
read/
├── README.md (ce fichier, version EN canonique)
├── TOC.md
├── check_priority_issues.sh
├── create_pr.sh
└── synch_main_dev_ci.sh
```

## Fichiers

- `README.md`: Ce document.
- `TOC.md`: Index des composants read.
- `check_priority_issues.sh`: Liste des issues prioritaires/securite.
- `create_pr.sh`: Creation de PR.
- `synch_main_dev_ci.sh`: Sync automatisee main->dev (bot-only).

## Scripts

### `synch_main_dev_ci.sh`

Synchronise `dev` avec `main` via PR automatisee (CI/bot uniquement).

Etapes:

1. Creation d'une branche temporaire depuis `main`
2. Creation PR vers `dev`
3. Activation auto-merge
4. Attente du merge
5. Nettoyage de branche temporaire

Variables notables: `MAIN`, `DEV`, `REMOTE`, `MAX_RETRIES`, `STRICT_MAIN_SHA`, `MAX_WAIT_SECONDS`.

### `check_priority_issues.sh`

Liste les issues GitHub avec labels `high priority` ou `security`.
Utilise notamment par `start_work.sh`.

### `create_pr.sh`

Cree une PR depuis la branche courante avec validation de tests par defaut.

Usage direct:

```bash
bash create_pr.sh [--base <branch>] [--title <title>] [--body <body>] [--draft] [--skip-tests]
```

Comportement:

1. Verifie que la branche courante n'est pas protegee
2. Lance `cargo test --workspace` par defaut
3. Cree la PR vers la base cible (defaut: `dev`)
4. Genere titre/body automatiquement si absents
5. Ajoute des labels selon le prefixe de branche

## Quand les executer directement?

En general: non.
Utiliser plutot les scripts `execute/` qui orchestrent l'ordre complet.

Exception: debug ou cas special ou vous savez exactement quelle etape isoler.

## Note d'architecture

Ces scripts doivent rester:

- Non interactifs (pas de prompt)
- Stables en code de sortie
- Composables et script-friendly
