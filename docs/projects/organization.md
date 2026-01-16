# Organisation multi-projets

## Introduction

Ce document détaille l'organisation des projets dans le workspace `automation_project`. Pour une vue d'ensemble, consultez [Vue d'ensemble](../overview.md).

---

## 1. Gestion de l'état interne des projets

Chaque projet est responsable de la gestion de son propre état interne. Cela inclut des éléments tels que les fichiers de configuration, les caches, les journaux, et les données temporaires.

### Principes clés

- **Indépendance** : Chaque projet doit gérer son état de manière isolée, sans dépendre d'autres projets.
- **Flexibilité** : La structure interne d'un projet est laissée à sa discrétion, afin qu'il puisse s'adapter à ses besoins spécifiques.
- **Isolation stricte** : Aucun fichier ou donnée interne ne doit "fuiter" vers d'autres projets.

### Recommandations

- Utiliser des fichiers ou des dossiers dédiés pour organiser l'état interne (par exemple, `state/`, `cache/`, `logs/`).
- Inclure un champ `schema_version` dans les fichiers persistés (comme `project.toml`) pour garantir la compatibilité future.
- Documenter la structure interne choisie pour faciliter la maintenance et la collaboration.

### Interdictions

- Aucun projet ne doit écrire dans l’état interne d’un autre projet.
- Aucun état partagé global hors des mécanismes explicitement prévus par l’Engine.
- Aucun accès direct à un autre projet via chemins relatifs ou absolus.

### Portée de l'état interne

L’état interne d’un projet inclut toute donnée persistée ou semi-persistée nécessaire à son fonctionnement, mais n’inclut pas le code source lui-même.

> Ces principes garantissent que chaque projet reste autonome et maintenable, tout en permettant une grande flexibilité dans son organisation interne.
