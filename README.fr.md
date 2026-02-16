# Projet Automation

Langue : [English](./README.md) | **Français**

Ce répertoire contient le workspace `automation_project`, conçu pour orchestrer plusieurs projets simultanément avec des fonctionnalités comme la génération de code, le linting, la documentation automatisée et l'orchestration de workflows.

## Rôle dans le projet

Ce dépôt sert de workspace principal pour le projet d'automatisation. Il coordonne l'ensemble des composants, produits et bibliothèques dans un système cohérent.

Il interagit principalement avec :

- `projects/products/stable/core/` : binaires cœur (engine, launcher, watcher, interface centrale)
- `projects/libraries/` : bibliothèques partagées (protocol, utilitaires communs, sécurité, IA)
- `documentation/` : documentation technique et guides d'architecture
- `.github/workflows/` : automatisation CI/CD et orchestration des workflows

## Architecture en bref

- L'Engine est l'autorité unique et le hub WebSocket.
- Les backends produits sont des processus séparés ; les UIs sont chargées dynamiquement à l'exécution.
- L'UI centrale agrège les UIs produits sans couplage à la compilation.

Pour les détails, voir le document d'architecture : `documentation/technical_documentation/ARCHITECTURE.md`.

## Structure du répertoire

```plaintext
./
├── .github/              # Configuration GitHub et workflows
├── documentation/        # Documentation technique et guides
├── projects/             # Produits et bibliothèques
│   ├── products/         # Backends produits et bundles UI
│   │   ├── stable/       # Produits prêts pour la production (core + produits stables)
│   │   └── unstable/     # Produits MVP pour itération rapide
│   └── libraries/        # Bibliothèques partagées (protocol, common, security, ai)
├── scripts/              # Scripts d'automatisation et de versioning
├── CONTRIBUTING.md       # Guide de contribution
└── README.md             # Ce fichier
```

## Organisation des produits

- `projects/products/stable/` : produits prêts pour la production et conformes aux principes d'architecture
  - `stable/core/` : binaires cœur (engine, launcher, watcher, interface centrale)
  - `stable/<product>/` : backends produits stables et bundles UI
- `projects/products/unstable/` : produits MVP pour expérimentation rapide (peuvent déroger à certains principes)

Voir [projects/products/README.md](projects/products/README.md) pour le détail stable vs unstable.

## Fichiers

- `README.md` : ce fichier.
- `CONTRIBUTING.md` : guide de contribution.
- `LICENSE` : licence du dépôt (si présente).
- `.github/` : configuration GitHub et workflows.
- `documentation/` : documentation technique et guides.
- `projects/` : produits et bibliothèques.
- `scripts/` : scripts d'automatisation et de versioning.

La documentation interne est disponible dans `documentation/`. Commencez par l'[index de documentation](documentation/TOC.md), ou allez directement au [TOC de la documentation technique](documentation/technical_documentation/TOC.md).

La documentation des scripts est indexée dans [scripts/TOC.md](./scripts/TOC.md).

## Contribuer

Les contributions sont les bienvenues. Ouvrez une issue ou une pull request sur le dépôt GitHub.

Pour les règles de contribution, voir [CONTRIBUTING.fr.md](./CONTRIBUTING.fr.md).

## Licences

Chaque crate de ce workspace peut avoir sa propre licence. Référez-vous au fichier `LICENSE` ou au `README.md` de chaque crate pour les détails de licence.
