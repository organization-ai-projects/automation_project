# Documentation des orchestrateurs

Langue : [English](../../README.md) | **Francais**

Ce repertoire contient les scripts d'orchestration, organises par mode d'execution et niveau d'interactivite.

## Role dans le projet

Ce repertoire orchestre des workflows complets en coordonnant operations Git, appels GitHub et interactions utilisateur.
Il interagit principalement avec:

- Les scripts utilitaires Git (`../git/`)
- L'API GitHub via `gh`
- Les developpeurs (prompts, guidance)
- La CI/CD (sync bot automatisee)

## Structure du repertoire

```plaintext
orchestrators/
├── README.md (ce fichier, version EN canonique)
├── TOC.md
├── execute/                    # Orchestrateurs interactifs (couche UI)
│   ├── start_work.sh
│   ├── ci_watch_pr.sh
│   └── labels_sync.sh
└── read/                       # Orchestrateurs non interactifs (couche API)
    ├── synch_main_dev_ci.sh
    ├── check_priority_issues.sh
    └── create_pr.sh
```

## Fichiers

- `README.md`: Ce document.
- `TOC.md`: Index des orchestrateurs.
- `execute/`: Couches interactives lancees par les humains.
- `read/`: Couches non interactives composees par scripts/CI.

## Architecture: execute vs read

### `execute/` - Orchestrateurs interactifs

Pour usage humain direct:

- prompts (`read -rp`) autorises
- sortie orientee utilisateur
- guidance pas-a-pas
- orchestration de flux complets

### `read/` - Composants composables

Pour usage script/bot:

- aucun prompt
- codes de sortie stables
- sortie parsable
- arguments/variables d'environnement en entree

## Contrat technique

### Scripts dans `read/`

- Pas de prompt utilisateur
- Valeurs par defaut robustes
- Codes de sortie explicites (`0` succes, non-zero echec)

### Scripts dans `execute/`

- Peuvent guider l'utilisateur
- Peuvent afficher un rendu lisible/humain
- Doivent appeler la couche `read/` pour la logique reutilisable

## Quand ajouter un script ici?

1. Script interactif? -> `execute/`
2. Script non interactif reutilisable? -> `read/`
3. Git pur sans orchestration? -> `../git/`
4. Utilitaire sourceable? -> `scripts/common_lib/`
