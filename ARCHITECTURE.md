# ARCHITECTURE

## 1. Vue d’ensemble

Ce document fournit une vue détaillée de l’architecture du projet `automatic_project`, incluant les dépendances entre les composants et des exemples de workflows types.

---

## 2. Diagramme des dépendances

### Description

Le diagramme ci-dessous illustre les dépendances compile-time entre les crates principales du projet. Chaque crate a un rôle spécifique et les dépendances sont strictement contrôlées pour éviter les cycles.

```plaintext
Dépendances compile-time (flèche = dépend de)

symbolic  ---->  core
neural    ---->  core

ai        ---->  core + symbolic (+ neural feature)

engine    ---->  core + ai
ui        ---->  core + engine

apps/app      ----> ui
apps/cli      ----> engine
apps/server   ----> engine
```

### Règles de dépendance

- `core` est indépendant et ne dépend d’aucune autre crate.
- `symbolic` et `neural` dépendent de `core` mais ne connaissent pas le workspace.
- `ai` dépend de `core`, `symbolic` et (optionnellement) de `neural` via un feature flag.
- `engine` dépend de `core` et `ai`.
- `ui` dépend de `core` et `engine`.
- Les applications (`apps/*`) dépendent de `ui` (pour `app`) ou de `engine` (pour `cli` et `server`).

---

## 3. Workflows types

### Workflow 1 : Analyse → Génération → Validation → Documentation

1. **Chargement du projet** :
   - `engine` initialise le contexte du projet (`ProjectContext`).
2. **Orchestration** :
   - `engine` lance un workflow et appelle `ai` pour planifier et router les tâches.
3. **Analyse** :
   - `ai` demande à `symbolic` d’analyser le code source.
4. **Génération** (optionnelle) :
   - Si nécessaire, `ai` délègue à `neural` la génération de code.
5. **Validation** :
   - `engine` exécute les validations, tests, et génère la documentation via des runners dédiés.
6. **Résultats** :
   - `engine` renvoie les résultats à `ui` sous forme d’événements.

```plaintext
ui/app  -> engine(workflow.run)
engine  -> ai(plan + routing)
ai      -> symbolic(analyse)
ai      -> neural(generate) [optionnel, feature]
engine  -> validation/tests/doc runners
engine  -> ui(events + results)
```

### Workflow 2 : Orchestration des workflows

1. **Chargement du projet** :
   - `engine` initialise le contexte du projet.
2. **Coordination IA** :
   - `engine` appelle `ai` pour coordonner les tâches symboliques et neuronales.
3. **Exécution** :
   - Les workflows sont exécutés et les résultats sont renvoyés à l’utilisateur via `ui`.

```plaintext
ui/app  -> engine(commands/events)
engine  -> ai(coordination)
ai      -> symbolic + neural
engine  -> ui(events + results)
```

---

## 4. Notes supplémentaires

- Les diagrammes peuvent être enrichis avec des outils comme PlantUML ou Mermaid pour une visualisation plus détaillée.
- Ce fichier est complémentaire au document principal de planification et peut évoluer indépendamment.

---

**Ce fichier est une référence visuelle et technique pour l’architecture du projet.**
