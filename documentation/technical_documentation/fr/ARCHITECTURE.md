# ARCHITECTURE (version finale)

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

- [Retour au TOC technique](TOC.md)

## 1. Principes d'architecture

### 1.1 Separation stricte Frontend / Backend

- **Frontend** : UIs (incluant l'UI centrale) ecrites en Rust (Dioxus) et distribuees en **UI bundles** (WASM + assets).
- **Backend** : executables services/backends (processus separes) qui realisent les taches (workflows, generation, linting, etc.).

Aucune UI ne contient de logique metier. Une UI :

- affiche un etat,
- envoie des commandes,
- ecoute des evenements.

### 1.2 Hub unique et autorite unique

- **Engine** est le **hub unique** (WebSocket) et l'**autorite d'execution** :
  - authentifie,
  - autorise,
  - journalise (audit),
  - orchestre,
  - supervise l'execution,
  - route commandes et evenements.

`central_ui` est la **passerelle UI** : il agrege les UI bundles et proxifie les actions UI vers `engine`.

### 1.3 Agregation reelle des UIs (sans dependances compile-time)

L'UI centrale **agrege** les UIs produits en chargeant leurs bundles **au runtime**.
Il est interdit de "dependre" des crates UI produit dans `central_ui`.

Consequence :

- pas de recompilation globale requise pour ajouter/mettre a jour une UI produit,
- l'UI centrale reste stable, extensible et "future proof".

---

## 2. Topologie des composants

Pour des descriptions detaillees des composants coeur (`engine`, `central_ui`, `watcher`, `launcher`) et des produits, se referer a [Produits et composants du workspace](projects/projects_products.md).

---

## 3. Communication et protocole (Command/Event)

### 3.1 Hub WebSocket

Toutes les connexions passent par `engine` :

- `central_ui` -> `engine` (client UI)
- `product backend` -> `engine` (client backend)
- `launcher` -> `engine` (client systeme)

Les UI bundles ne se connectent pas directement a `engine` ; ils tournent dans `central_ui`, qui agit comme client unique vers `engine`.

Interdictions :

- UI bundle <-> engine direct
- UI bundle <-> backend direct
- backend <-> backend direct
- UI <-> UI direct

### 3.2 Modele Command / Event

- **Command** (request/response) : declenche une action ou demande un etat.
- **Event** (stream) : notifications push temps reel.

Exemples de commandes :

- `ListProjects`
- `ListProducts`
- `ActivateProduct(product_id)`
- `RunWorkflow(project_id, workflow_id)`
- `SpawnBackend(product_id)` (si backend non demarre)
- `GetStatus(project_id)`

Exemples d'evenements :

- `LogLine(project_id, level, message)`
- `Progress(workflow_id, pct)`
- `ProductStateChanged(product_id, state)`
- `WorkflowFinished(workflow_id, result)`

### 3.3 Exemple : flux Command/Event

Le diagramme ci-dessous reflète les regles strictes de communication. Les UIs produits envoient des commandes a `central_ui`, qui les transfere a `engine`. `engine` appelle les backends produits et renvoie les evenements vers les UIs via `central_ui`.

Chemin commande : UI -> central_ui -> engine -> backend. Chemin evenement : backend -> engine -> central_ui -> UI.

Note bootstrap (phase de demarrage) : `launcher` demarre les services coeur (`engine`, `central_ui`, `watcher`). Apres bootstrap, le flux normal commande/evenement commence.

![Bootstrap (startup)](assets/architecture_bootstrap.png)

```plaintext
                             +---------+
                             | Watcher |
                             +---------+

   +-----------+    +------------+    +---------+    +---------------+
   | Product   | <->| Central UI | <->| Engine  | <->| Product Backend|
   | UI (WASM) |    |  (Gateway) |    |  (Hub)  |    |   (Service)    |
   +-----------+    +------------+    +---------+    +---------------+
```

#### Notes mises a jour

- **Product UI** : envoie des commandes a `central_ui`; ne parle jamais directement aux backends.
- **Central UI** : passerelle entre UIs et `engine`; transfere les commandes et stream les evenements.
- **Engine** : hub central qui authentifie, autorise, orchestre et route vers les backends.
- **Product Backend** : execute les commandes et emet des evenements via `engine`.
- **Watcher** : supervise les composants coeur (`engine`, `central_ui`, etc.) et les redemarre en cas d'echec. Il ne fait pas partie du chemin commande/evenement.

---

## 4. UI Bundles (regle centrale)

### 4.1 Definition

Un **UI bundle** est un artefact distribuable :

- `ui.wasm` (Dioxus WASM)
- `assets/` (icons, css, etc.)
- `ui_manifest.ron` (metadonnees du bundle)

Le UI bundle :

- depend de `protocol` + `common`,
- communique avec `engine` **via `central_ui`** (jamais directement).

### 4.2 Regles UI

- Chaque UI produit est un bundle WASM charge par `central_ui`.
- Une UI ne depend jamais d'un backend produit.
- Toutes les actions passent par `engine` via `protocol`.

### 4.3 Chargement des bundles

`central_ui` charge les UI bundles au runtime :

- depuis le disque local (installation),
- ou depuis `engine` (distribution distante).

`central_ui` ne compile pas les UIs produites, il les **charge**.

### 4.4 Contrat UI minimal

- S'execute dans `central_ui`
- Authentification session utilisateur (via `central_ui`)
- Envoi de Command / reception d'Events via `central_ui`

---

## 5. Gestion des backends

### 5.1 Regle

Les backends sont des processus separes, demarres et supervises par `engine`.

### 5.2 Cycle de vie

Quand un utilisateur ouvre une UI produit ou declenche une action :

1. Le UI bundle envoie une `Command` a `central_ui`.
2. `central_ui` transfere la `Command` a `engine`.
3. `engine` verifie auth/permissions.
4. si le backend n'est pas demarre : `engine` le demarre.
5. `engine` route la commande vers le backend.
6. le backend publie des `Event` (logs, progression, resultat).
7. `engine` transfere les evenements a `central_ui`.
8. `central_ui` affiche l'etat en temps reel.

---

## 6. Registre (source de verite)

### 6.1 Role

Le registre central (`.automation_project/registry.json`) est la source de verite pour (voir [Registry](registry.md)) :

- liste des produits,
- chemins des bundles UI,
- identites backend,
- versions + compatibilite de schema.

### 6.2 Regle

Aucun composant ne doit deduire l'architecture via scan implicite sans registre.
Le registre est **explicite** et versionne.

---

## 7. Securite

- `engine` est l'unique autorite :
  - authentification utilisateurs (UI),
  - authentification machines (backends),
  - permissions,
  - audit log.

`central_ui` affiche les permissions renvoyees par `engine` mais ne decide jamais.

---

## 8. Structure du workspace

```plaintext
automation_project/
|-- projects/
|   |-- products/
|   |   |-- core/
|   |   |   |-- launcher/         # bootstrap
|   |   |   |-- engine/           # WS hub + orchestration + spawn backends
|   |   |   |-- central_ui/       # desktop cockpit, loads UI bundles
|   |   |   `-- watcher/          # external supervision
|   |   |-- <product_x>/
|   |   |   |-- backend/          # backend (binary)
|   |   |   `-- ui/               # UI source (compiles to ui_dist/)
|   |   |       `-- ui_dist/      # packaged artifacts (wasm + assets + manifest)
|   `-- libraries/
|       |-- common/
|       |-- protocol/
|       |-- security/
|       |-- symbolic/
|       |-- neural/
|       |-- ai/
|       `-- ui/                   # reusable UI components (lib)
```

---

## 9. Produits initiaux

Pour les details sur les produits initiaux comme `varina`, se referer a [Produits et composants du workspace](projects/projects_products.md).
