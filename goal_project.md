# Planification du projet d’automatisation

## 1. Objectif

L’objectif de ce projet est de construire un **workspace d’automatisation de niveau industriel** (type Google / Microsoft) capable d’orchestrer **plusieurs projets simultanément**, avec une automatisation avancée du cycle de développement logiciel.

Le système vise à automatiser :

- La génération de code
- Le linting et la validation structurelle
- La documentation
- L’application et l’évolution des bonnes pratiques
- L’orchestration de workflows complets (analyse → génération → validation → itération)

Le projet est **100 % Rust**, avec :

- une base **symbolique forte** (règles, structures, invariants)
- une composante **neuronale optionnelle et activable** (Burn)

---

## 2. Concept fondamental : `automatic_project`

### Définition

**`automatic_project` est le workspace racine.**

Ce n’est pas un simple projet, mais un **environnement global outillé**, capable de gérer **N projets hétérogènes**.

> `automatic_project` = conteneur racine + registry + tooling + orchestration

Il peut contenir :

- des **produits finaux** (applications, services, outils)
- des **libraries / SDK** (réutilisables, versionnées)

---

## 3. Organisation multi-projets (first-class)

### Séparation claire des projets

```plaintext
automatic_project/
├── projects/
│   ├── products/        # Produits finaux (vendables, exécutables)
│   └── libraries/       # Libraries, SDK, modules réutilisables
```

Chaque projet est **isolé fonctionnellement et statiquement**.

### State par projet (obligatoire)

Chaque projet possède son propre état interne :

```plaintext
<project_root>/.dev-forge/
├── project.toml
├── state/
├── cache/
├── runs/
└── logs/
```

Aucun cache, index, règle ou mémoire IA ne doit fuiter entre projets.

> Tous les fichiers persistés (ex. `project.toml`, `state`) doivent inclure une version de schéma (`schema_version`) pour garantir la compatibilité future.

---

## 4. Architecture globale des crates

### Vue d’ensemble

```plaintext
automatic_project/
├── crates/
│   ├── core/        # Types purs, IDs, erreurs, événements
│   ├── symbolic/    # Logique symbolique (agrégateur)
│   ├── neural/      # Logique neuronale (Burn) – optionnelle
│   ├── ai/          # Orchestrateur symbolic + neural
│   ├── engine/      # Workspace, projets, workflows
│   └── ui/          # Composants UI (Dioxus)
│
├── apps/
│   ├── app/         # Application desktop
│   ├── cli/         # CLI (phase 2)
│   └── server/      # Serveur (phase 3)
│
├── projects/
│   ├── products/
│   └── libraries/
│
└── .automatic_project/
    ├── registry.json
    ├── settings.json
    ├── cache/
    └── logs/
```

---

## 5. Rôle détaillé des composantes

### A. `core`

- Types fondamentaux (IDs, enums, états)
- Erreurs communes
- Événements et contrats
- Aucune dépendance runtime (tokio, dioxus, etc.)

---

### B. Composante Symbolique (`symbolic`)

- Règles de linting et bonnes pratiques
- Analyse statique (structure, conventions, patterns)
- Moteur de règles et de décisions
- Orchestration symbolique des workflows

> `symbolic` est un **agrégateur** de sous-modules symboliques spécialisés.

---

### C. Composante Neuronale (`neural`)

- Compréhension d’intentions (langage naturel → structure)
- Génération de code Rust
- Ajustement par feedback
- Entraînement et inférence via **Burn**

Activation par feature flag uniquement.

---

### D. Orchestrateur IA (`ai`)

- Coordonne symbolic et neural
- Décide quand déléguer au neuronal
- Ne stocke **aucun état global**
- Travaille exclusivement via un `ProjectContext`

---

### E. Engine (`engine`)

Cœur du système.

Responsabilités :

- Gestion du workspace `automatic_project`
- Registry global des projets
- Chargement et lifecycle des projets
- Création des `ProjectContext`
- Exécution des workflows

> `engine` est le noyau logique du produit.

---

### F. Interface Utilisateur (`ui`)

- Développée avec **Dioxus**
- Affichage multi-projets
- Suivi des workflows
- Visualisation des logs et états

L’UI **ne contient aucune logique métier**.

---

### G. Applications (`apps`)

- `app` : desktop (UI principale)
- `cli` : automatisation / scripting (phase 2)
- `server` : accès distant, API, WebSocket (phase 3)

Le serveur dépend de `engine`, jamais de `ui`.

---

## 6. Technologies et formats

- **Rust** : langage unique
- **Burn** : IA neuronale
- **Dioxus** : UI
- **WebSocket** : canal d’événements et de commandes, sans logique métier. Peut être remplacé par gRPC ou autre sans casser l’architecture.
- **TOML / YAML** : configuration
- **.bin** : format prioritaire pour les données volumineuses
- **.jsonl / .ron** : formats lisibles accompagnés d’un `.bin`

Logs structurés avec niveaux : info / warning / error.

---

## 7. Phases de développement

### Phase 1 – Fondation (v0)

- Workspace `automatic_project`
- Crates `core`, `symbolic`, `ai`, `engine`, `ui`
- Multi-projets natif (products / libraries)
- UI minimale

---

### Phase 2 – Intelligence et automatisation

- Activation `neural`
- CLI
- Workflows complexes
- Feedback loop

---

### Phase 3 – Distribution

- Serveur
- WebSocket
- Sécurité (TLS, auth)
- Scalabilité

---

## 8. Principes non négociables

- Multi-projets **dès le design**
- Isolation stricte des états
- Symbolique prioritaire
- Neuronal optionnel
- APIs claires et stables
- Architecture pensée long terme
- Aucune dépendance circulaire entre les crates :
  - `engine` ne dépend jamais de `ui`
  - `ai` ne dépend jamais de `engine`
  - `symbolic` et `neural` ne connaissent pas le workspace

---

## 9. Documentation automatisée

### Objectifs

La documentation est une composante essentielle du projet et doit être générée automatiquement pour garantir sa mise à jour et sa cohérence avec le code.

#### Fonctionnalités proposées

- Génération automatique de documentation via `cargo doc`.
- Documentation enrichie avec des exemples de code, diagrammes, et explications détaillées.
- Exportation dans plusieurs formats :
  - **HTML** : pour la consultation en ligne.
  - **Markdown** : pour l’intégration dans des dépôts Git.
  - **PDF** : pour les livrables ou la distribution hors ligne.
- Intégration avec les workflows symboliques et neuronaux pour enrichir la documentation avec des exemples générés automatiquement.

#### Discussion

- Faut-il inclure un outil externe comme `mdBook` ou `Docusaurus` ?
- Quels formats privilégier pour la documentation ?
- Faut-il intégrer des outils de visualisation (diagrammes, graphiques) ?

---

## 10. Automatisation des bonnes pratiques

### Objectifs

L’automatisation des bonnes pratiques garantit la qualité et la maintenabilité du code.

#### Fonctionnalités proposées

- Définir des règles de linting spécifiques au projet.
- Automatiser la vérification des conventions de code via :
  - **Clippy** : pour les règles Rust standards.
  - Règles personnalisées adaptées au projet.
- Générer des rapports détaillés sur les violations des bonnes pratiques.
- Proposer des corrections automatiques lorsque possible.

---

## 11. Orchestration des workflows

### Objectifs

L’orchestration des workflows est au cœur du système et permet d’automatiser les étapes clés du développement logiciel.

#### Étapes typiques d’un workflow

1. **Analyse** : Vérification du code source et des dépendances.
2. **Génération** : Création de nouveaux fichiers ou modules.
3. **Validation** : Linting, tests, et vérifications structurelles.
4. **Itération** : Ajustements basés sur les résultats de la validation.

#### Exemple concret

Un workflow typique pourrait inclure :

- Analyse des fichiers Rust pour détecter les modules manquants.
- Génération automatique des modules nécessaires.
- Validation des modules générés avec des tests unitaires.
- Documentation automatique des modules ajoutés.

---

**Ce document constitue la référence architecturale du projet.**
Toute implémentation doit s’y conformer.
