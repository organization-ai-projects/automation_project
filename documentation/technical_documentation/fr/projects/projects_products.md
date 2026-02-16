# Produits et composants du workspace

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

- [Retour a l'index projets](../TOC.md)

## Definition d'un produit

Un produit = `metadata.ron` + `backend` + `ui bundle`.

Pour les details sur `metadata.ron`, voir [Metadata](../metadata.md). `metadata.ron` utilise des valeurs hex `ProtocolId` pour `id`.

## Definition d'un backend

- Processus separe (service backend)
- Machine authentifiee avec `engine`
- N'expose pas de ports publics
- Communique uniquement avec `engine`

## 1. Produits coeur

Les **produits coeur** sont des composants fondamentaux qui garantissent le bon fonctionnement de tout le systeme. Ils sont situes dans `products/core`.

### 1.1 Engine (`engine`)

Coeur du systeme.

Responsabilites :

- Gestion du workspace `automation_project`
- Registre global des projets (voir [Registry](../registry.md))
- Chargement des projets et gestion du cycle de vie
- Creation du `ProjectContext`
- Execution des workflows

> `engine` est le coeur logique du produit.

---

### 1.2 Launcher (`launcher`)

Composant de demarrage initial.

Responsabilites :

- Initialisation des composants critiques (engine, central_ui, watcher).
- Gestion des commandes de bootstrap.
- Supervision deleguee au `watcher` apres demarrage.

> `launcher` est le point d'entree principal du systeme.

---

### 1.3 Central UI (`central_ui`)

Interface utilisateur centrale.

Responsabilites :

- Administration des produits.
- Agregation des UIs des differents produits.
- Navigation entre produits.

> `central_ui` fournit une vue unifiee pour les utilisateurs finaux.

---

### 1.4 Watcher (`watcher`)

Superviseur global.

#### Responsabilites

- **Monitoring des composants critiques** :
  - Surveille `launcher`, `engine` et `central_ui`.
  - Ping regulier des composants pour verifier leur statut.
- **Redemarrage automatique** :
  - Redemarre les composants en echec en cas de crash ou non-reponse.
  - Implemente une logique d'exponential backoff pour eviter les boucles de restart.
- **Gestion des logs** :
  - Journalise les evenements critiques (crashs, redemarrages) dans un fichier de logs dedie.
- **Configuration flexible** :
  - Permet de definir les composants surveilles, les intervalles de ping et les politiques de redemarrage via un fichier de configuration (`watcher.toml`).

Le `watcher` ne communique jamais avec les projets ou workflows. Il est strictement limite a la supervision des executables coeur.

#### Exemple de configuration (`watcher.toml`)

```toml
[components]
launcher = { ping_interval = 10, restart_policy = "always" }
engine = { ping_interval = 5, restart_policy = "on-failure" }
central_ui = { ping_interval = 15, restart_policy = "always" }

[logging]
log_file = "/var/log/watcher.log"
log_level = "info"
```

---

## 2. Interfaces utilisateur (UI)

### UIs dediees pour les produits

- Chaque produit (ex : `app`) inclut une **UI dediee** a ses fonctionnalites specifiques.
- Une **bibliotheque commune** dans `projects/libraries/ui` fournit des composants reutilisables pour toutes les UIs.
- Un **mecanisme de registre** centralise la liste des produits et leurs UIs disponibles (voir [Registry](../registry.md)).
- Une **UI centrale** (dashboard global) permet :
  - l'administration des produits,
  - l'agregation des UIs de differents produits,
  - la navigation entre produits.

---

## 3. `products/core`

- **Namespace** : contient les executables principaux (launcher, engine, central_ui).
- **Regles** :
  - Chaque sous-dossier est une crate binaire distincte.
  - Dependances partagees via `libraries/common` et `libraries/protocol`.

Cette architecture assure une supervision robuste et reduit les points uniques de defaillance (SPOF) en isolant les responsabilites entre composants.

Hierarchie d'execution :
launcher -> (demarre engine, central_ui, watcher) -> watcher supervise le coeur -> engine orchestre produits et UIs
