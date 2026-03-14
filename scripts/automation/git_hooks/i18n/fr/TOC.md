# Table des matieres

Langue : [English](../../TOC.md) | **Francais**

Ce document fournit une vue d'ensemble des fichiers de documentation de ce dossier.

## Documentation

- [README.md](../../README.md) : documentation principale des hooks Git

## Hooks

- [commit-msg](../../commit-msg) : valide le format des messages de commit
- `pre-commit` : genere par `versioning_automation automation install-hooks`, lance les checks pre-commit via la CLI Rust
- [prepare-commit-msg](../../prepare-commit-msg) : genere automatiquement le sujet de commit
- `pre-push` : genere par `versioning_automation automation install-hooks`, lance les checks pre-push via la CLI Rust
- `post-checkout` : genere par `versioning_automation automation install-hooks`, lance les checks post-checkout via la CLI Rust
- `versioning_automation automation install-hooks` : installe les hooks dans `.git/hooks/`
