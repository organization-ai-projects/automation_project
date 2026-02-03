# Projet d'Automatisation

**English version**: [README.md](README.md)

`automation_project` est un espace de travail d'automatisation avancé, conçu pour orchestrer plusieurs projets simultanément avec des fonctionnalités telles que la génération de code, le linting, la documentation automatisée et l'orchestration de flux de travail.

## Architecture en un coup d'œil

- Engine est l'autorité unique et le hub WebSocket.
- Les backends de produits sont des processus séparés ; les UIs sont des bundles chargés à l'exécution.
- L'interface centrale agrège les UIs de produits sans couplage au moment de la compilation.

Pour plus de détails, voir le document d'architecture : `documentation/technical_documentation/en/ARCHITECTURE.md`.

## Structure du dépôt

- `projects/products/core/` : binaires de base (engine, launcher, watcher, central UI).
- `projects/products/<product>/` : backends de produits et bundles UI.
- `projects/libraries/` : bibliothèques partagées (protocol, common, security, symbolic, neural, ai).
- `documentation/` : documentation technique et guides.

La documentation interne est disponible dans le dossier `documentation/`. Commencez par l'[Index de Documentation](documentation/TOC.fr.md), ou passez directement au [TOC de Documentation Technique](documentation/technical_documentation/fr/TOC.md).

La documentation des scripts est indexée dans [scripts/TOC.fr.md](scripts/TOC.fr.md).

## Contribuer

Les contributions sont les bienvenues ! Veuillez ouvrir une issue ou une pull request sur le dépôt GitHub.

Pour les directives de contribution, voir [CONTRIBUTING.fr.md](CONTRIBUTING.fr.md).

## Licences

Chaque crate dans cet espace de travail peut avoir sa propre licence. Veuillez vous référer au fichier `LICENSE` ou au `README.md` dans le répertoire de chaque crate pour des détails de licence spécifiques.
