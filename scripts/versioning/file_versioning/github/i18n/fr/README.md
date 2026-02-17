# Documentation des scripts GitHub

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les scripts axes workflows GitHub et generation de metadonnees PR.

## Role dans le projet

Ce repertoire est reserve aux operations cote GitHub.
Il interagit principalement avec:

- L'API GitHub via `gh`
- L'historique Git local (fallback/dry-run)
- La configuration du repository
- Les workflows GitHub Actions

## Structure du repertoire

```plaintext
github/
├── README.md (ce fichier, version EN canonique)
├── TOC.md
├── generate_pr_description.sh
├── parent_issue_guard.sh
├── lib/
│   ├── classification.sh
│   └── rendering.sh
└── tests/
    └── generate_pr_description_regression.sh
```

## Fichiers

- `README.md`: Ce document.
- `TOC.md`: Index des scripts GitHub-only.
- `generate_pr_description.sh`: Genere une description PR structuree.
- `parent_issue_guard.sh`: Verifie les regles parent/enfant avant fermeture.
- `lib/classification.sh`: Aides de classification PR/issues.
- `lib/rendering.sh`: Aides de rendu de sortie.
- `tests/generate_pr_description_regression.sh`: Matrice de regression CLI.

## Portee

Les scripts de ce repertoire doivent:

- Etre centres sur les workflows PR/issues GitHub
- Preferer les donnees `gh` quand disponibles
- Garder des fallbacks robustes en indisponibilite API

## Script principal: generate_pr_description.sh

Genere une description PR prete a coller (par ex. `dev -> main`) a partir des PR enfants et issues reliees.
Supporte:

- Mode PR number (enrichi GitHub)
- Mode dry-run local
- Mode auto (generation + creation PR)

Sections generees:

- `Description`
- `Scope`
- `Compatibility`
- `Issues Resolved`
- `Key Changes`
- `Testing`
- `Additional Notes`

### Utilisation

```bash
bash generate_pr_description.sh [--keep-artifacts] [--debug] [--duplicate-mode MODE] [--auto-edit PR_NUMBER] MAIN_PR_NUMBER [OUTPUT_FILE]
bash generate_pr_description.sh --dry-run [--base BRANCH] [--head BRANCH] [--create-pr] [--allow-partial-create] [--duplicate-mode MODE] [--debug] [--auto-edit PR_NUMBER] [--yes] [OUTPUT_FILE]
bash generate_pr_description.sh --auto [--base BRANCH] [--head BRANCH] [--debug] [--yes]
```

### Options clefs

- `--dry-run`: extraction locale (base `dev` par defaut, head = branche courante)
- `--base`, `--head`: plage explicite de comparaison
- `--create-pr`: creation PR avec le body genere
- `--allow-partial-create`: autorise la creation meme si enrichissement partiel
- `--auto-edit PR_NUMBER`: met a jour directement une PR existante
- `--duplicate-mode safe|auto-close`: gestion des duplicats
- `--yes`: mode non interactif
- `--debug`: traces de classification/extraction
- `--auto`: flux memoire (`--dry-run` + `--create-pr`)
- `--keep-artifacts`: garde les fichiers intermediaires

### Codes de sortie

- `0`: succes
- `2`: erreur d'usage/arguments
- `3`: dependance manquante (`gh`/`jq`)
- `4`: contexte Git invalide
- `5`: aucune donnee extraite en dry-run
- `6`: enrichissement partiel bloquant la creation PR

## Tests de regression

```bash
bash tests/generate_pr_description_regression.sh
```
