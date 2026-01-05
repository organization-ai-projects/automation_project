# ARCHITECTURE (version finale)

## 1. Principes d’architecture

### 1.1 Séparation stricte Frontend / Backend

- **Frontend** : UIs (y compris l’UI centrale) écrites en Rust (Dioxus) et distribuées sous forme de **bundles UI** (WASM + assets).
- **Backend** : services/backends exécutables (process séparés) qui réalisent le travail (workflows, génération, lint, etc.).

Aucune UI ne contient de logique métier. Une UI ne fait que :

- afficher un état,
- envoyer des commandes,
- écouter des événements.

### 1.2 Hub unique et autorité unique

- **Engine** est le **hub unique** (WebSocket) et l’**autorité d’exécution** :

  - authentifie,
  - autorise,
  - journalise (audit),
  - orchestre,
  - supervise l’exécution,
  - route commandes et événements.

### 1.3 Agrégation réelle des UIs (sans dépendances compile-time)

L’UI centrale **agrège** les UIs produits en chargeant leurs bundles **au runtime**.
Il est interdit de “dépendre” des crates UI produits dans `central_ui`.

Conséquence :

- aucune recompilation globale requise pour ajouter/mettre à jour une UI produit,
- l’UI centrale reste stable, extensible, et “future proof”.

---

## 2. Topologie des composants

### 2.1 Composants essentiels (core)

- `engine` (binaire) : hub WS + autorité + orchestration + gestion process.
- `central_ui` (binaire) : cockpit desktop, charge des bundles UI.
- `watcher` (binaire) : superviseur externe (resilience: relance, backoff, healthchecks).
- `launcher` (binaire) : bootstrap initial (démarre engine/central_ui/watcher si nécessaire).

### 2.2 Produits

Un **produit** peut contenir :

- un **backend** (exécutable) : optionnel mais recommandé,
- une ou plusieurs **UIs** (bundles) : optionnel (mais attendu si le produit a une surface utilisateur).

Le backend et les UIs d’un produit ne communiquent **jamais directement**.

---

## 3. Communication et protocole (Command/Event)

### 3.1 WebSocket Hub

Toutes les connexions passent par `engine` :

- `central_ui` → `engine` (UI client)
- `product backend` → `engine` (backend client)
- `launcher` → `engine` (system client)

Interdictions :

- UI ↔ backend direct
- backend ↔ backend direct
- UI ↔ UI direct

### 3.2 Modèle Command / Event

- **Command** (request/response) : déclenche une action ou demande un état.
- **Event** (stream) : notifications push temps réel.

Exemples de commandes :

- `ListProjects`
- `ListProducts`
- `ActivateProduct(product_id)`
- `RunWorkflow(project_id, workflow_id)`
- `SpawnBackend(product_id)` (si backend non lancé)
- `GetStatus(project_id)`

Exemples d’événements :

- `LogLine(project_id, level, message)`
- `Progress(workflow_id, pct)`
- `ProductStateChanged(product_id, state)`
- `WorkflowFinished(workflow_id, result)`

---

## 4. Bundles UI (règle centrale)

### 4.1 Définition

Un **bundle UI** est un artefact distribuable :

- `ui.wasm` (Dioxus WASM)
- `assets/` (icônes, css, etc.)
- `ui_manifest.ron` (métadonnées du bundle)

Le bundle UI :

- dépend de `protocol` + `common`,
- parle uniquement à `engine` via WS.

### 4.2 Chargement des bundles

`central_ui` charge les bundles UI au runtime :

- depuis le disque local (installation),
- ou depuis `engine` (distribution distante).

`central_ui` ne compile pas les UIs produits, il les **charge**.

---

## 5. Gestion des backends

### 5.1 Règle

Les backends sont des processus séparés, démarrés et supervisés par `engine`.

### 5.2 Cycle de vie

Quand un utilisateur ouvre une UI produit ou déclenche une action :

1. `central_ui` envoie une `Command` à `engine`.
2. `engine` vérifie auth/permissions.
3. si le backend n’est pas lancé : `engine` le démarre.
4. `engine` route la commande vers le backend.
5. le backend publie des `Event` (logs, progress, résultat).
6. `central_ui` affiche l’état temps réel.

---

## 6. Registry (source de vérité)

### 6.1 Rôle

Le registry central (`.automation_project/registry.json`) est la source de vérité pour :

- liste des produits,
- chemins bundles UI,
- identités backends,
- versions + compatibilité schema.

### 6.2 Règle

Aucun composant n’infère l’architecture par scan implicite sans registry.
Le registry est **explicite** et versionné.

---

## 7. Sécurité

- `engine` est l’unique autorité :

  - authentification utilisateur (UI),
  - authentification machine (backends),
  - permissions,
  - audit log.

`central_ui` affiche les permissions renvoyées par `engine` mais ne décide jamais.

---

## 8. Structure du workspace

```plaintext
automation_project/
├── projects/
│   ├── products/
│   │   ├── core/
│   │   │   ├── launcher/         # bootstrap
│   │   │   ├── engine/           # WS hub + orchestration + spawn backends
│   │   │   ├── central_ui/       # cockpit desktop, charge bundles UI
│   │   │   └── watcher/          # supervision externe
│   │   ├── <product_x>/
│   │   │   ├── backend/          # backend (binaire)
│   │   │   └── ui/               # source UI (compile en ui_dist/)
│   │   │       └── ui_dist/      # artefacts packagés (wasm + assets + manifest)
│   └── libraries/
│       ├── common/
│       ├── protocol/
│       ├── security/
│       ├── symbolic/
│       ├── neural/
│       ├── ai/
│       └── ui/                   # composants UI réutilisables (lib)
```

## 9. Produits initiaux

### 9.1 Varina

**Varina** est le premier produit du workspace. Il est dédié à la partie automatisation et génération. Ses responsabilités incluent :

- Automatisation des workflows de développement.
- Génération de code et de modules nécessaires.
- Orchestration des tâches symboliques et neuronales.
- Intégration avec le `engine` pour exécuter les commandes et publier les événements.

Structure :

```plaintext
projects/
├── products/
│   ├── varina/
│   │   ├── backend/          # backend (binaire pour l’automatisation)
│   │   └── ui/               # source UI (compile en ui_dist/)
│   │       └── ui_dist/      # artefacts packagés (wasm + assets + manifest)
```
