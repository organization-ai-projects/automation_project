# Produits et Composants du Workspace

## 1. Core

### 1.1 Engine (`engine`)

Cœur du système.

Responsabilités :

- Gestion du workspace `automation_project`
- Registry global des projets
- Chargement et lifecycle des projets
- Création des `ProjectContext`
- Exécution des workflows

> `engine` est le noyau logique du produit.

---

### 1.2 Launcher (`launcher`)

Composant de démarrage initial.

Responsabilités :

- Initialisation des composants critiques (engine, central_ui, watcher).
- Gestion des commandes de bootstrap.
- Supervision déléguée au `watcher` après démarrage.

> `launcher` est le point d'entrée principal du système.

---

### 1.3 Central UI (`central_ui`)

Interface utilisateur centrale.

Responsabilités :

- Administration des produits.
- Agrégation des UIs des différents produits.
- Navigation entre les produits.

> `central_ui` fournit une vue unifiée pour les utilisateurs finaux.

---

### 1.4 Watcher (`watcher`)

Superviseur global.

#### Responsabilités

- **Surveillance des composants critiques** :
  - Supervise le `launcher`, l'`engine`, et le `central_ui`.
  - Ping régulièrement les composants pour vérifier leur état.
- **Redémarrage automatique** :
  - Redémarre les composants défaillants en cas de crash ou de non-réponse.
  - Implémente une logique de backoff exponentiel pour éviter les boucles de redémarrage.
- **Gestion des logs** :
  - Consigne les événements critiques (crashs, redémarrages) dans un fichier de log dédié.
- **Configuration flexible** :
  - Permet de définir les composants à surveiller, les intervalles de ping, et les politiques de redémarrage via un fichier de configuration (`watcher.toml`).

Le `watcher` ne communique jamais avec les projets ou les workflows. Il se limite strictement à la supervision des exécutables du core.

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

## 2. Interfaces Utilisateur (UI)

### 2.1 UI pour chaque produit

- Chaque produit (ex. `app`) inclut une **UI dédiée** pour ses fonctionnalités spécifiques.
- Une **bibliothèque commune** dans `projects/libraries/ui` fournit des composants réutilisables pour toutes les UIs.
- Un **mécanisme de registry** centralise la liste des produits et leurs UIs disponibles.
- Une **UI centrale** (dashboard global) permet :
  - L’administration des produits.
  - L’agrégation des UIs des différents produits.
  - La navigation entre les produits.

> Ces informations ont été consolidées pour éviter les doublons.

---

## 3. `products/core`

- **Namespace** : Contient les exécutables principaux (launcher, engine, central_ui).
- **Règles** :
  - Chaque sous-dossier est un crate binaire distinct.
  - Dépendances partagées via `libraries/common` et `libraries/protocol`.

> Cette section a été consolidée pour éviter les doublons.

Cette architecture garantit une supervision robuste et réduit les points de défaillance uniques (SPOF) en isolant les responsabilités entre les composants.

Hiérarchie d’exécution :
launcher → watcher → engine → projets → UIs
