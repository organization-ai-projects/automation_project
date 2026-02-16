# Processus systeme

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

- [Retour au TOC technique](TOC.md)

## Introduction

Ce document explique comment les processus coeur du systeme sont lances, supervises et coordonnes dans `automation_project`.

---

## 1. Flux des processus coeur

### 1.1 Objectifs

L'Engine est le hub unique des commandes/evenements et coordonne toute execution. Les services coeur sont demarres par le Launcher et supervises par le Watcher.

### 1.2 Demarrage et supervision (Launcher)

Pour les utilisateurs et operateurs du workspace, le Launcher est le point d'entree :

1. Demarrer le systeme avec le Launcher (`cargo run -p launcher` depuis la racine du repo).
2. Le Launcher demarre les services coeur : `engine`, `central_ui`, et `watcher`.
3. Le Watcher supervise les services coeur et les redemarre en cas de crash.
4. L'Engine devient le hub unique pour commandes et evenements.

### 1.3 Flux Command/Event

1. Un bundle UI envoie une Command a `central_ui`, qui la transfere a l'Engine.
2. L'Engine valide auth/permissions.
3. L'Engine route la commande vers le backend cible.
4. Le backend emet des Events (logs, progression, resultats).
5. L'Engine transfere les Events a `central_ui`, qui les affiche.

### 1.4 Checklist premier lancement

- Verifier que le registre est disponible (`.automation_project/registry.json`). Voir [Registry](registry.md).
- Bootstrap admin de type appliance (one-time, sans terminal) :
  - L'Engine genere `~/.automation_project/owner.claim` au premier demarrage (permissions 0600).
  - Sur plateformes non Unix, les permissions strictes 0600 peuvent ne pas etre enforceables ; traiter le fichier claim comme sensible et restreindre l'acces via ACLs OS quand possible.
  - L'Engine reste en mode setup tant que la claim n'est pas consommee.
  - Central UI lit `owner.claim` localement et appelle `POST /setup/owner/admin` avec :
    - `claim` (secret du fichier)
    - `user_id` (32 caracteres hex)
    - `password`
  - L'Engine verifie la claim, cree le premier admin, puis consomme la claim et ecrit `owner.claim.used`.
  - Le mode setup est desactive definitivement apres creation du premier admin.
  - Les claims expirent apres 24h ; les claims expirees sont regenerees au prochain demarrage.
- Le login valide les credentials contre l'identity store et rejette les credentials invalides.
- L'escalade de role via la requete login est ignoree (le role est derive de l'identity store).

### 1.5 Diagrammes

#### Flux des processus coeur

Le diagramme ci-dessous illustre le flux des processus coeur :

```plaintext
+-----------+       +---------+       +---------+       +---------+
|   UI      | ----> | Central | ----> | Engine  | ----> | Backend |
|  Bundle   |       |   UI    |       |         |       |         |
+-----------+       +---------+       +---------+       +---------+
     |                     |                 |                 |
     v                     v                 v                 v
+-----------+       +---------+       +---------+       +---------+
| Commands  |       | Validate |       | Route   |       | Emit    |
| & Events  |       | Auth     |       | Command |       | Events  |
+-----------+       +---------+       +---------+       +---------+
```

#### Flux de supervision

Le diagramme suivant montre comment le Watcher supervise les services coeur :

```plaintext
+---------+       +---------+       +---------+
| Launcher| ----> | Watcher | ----> | Services|
+---------+       +---------+       +---------+
     |                 |                 |
     v                 v                 v
+---------+       +---------+       +---------+
| Start   |       | Monitor |       | Restart |
| Services|       | Health  |       | on Crash|
+---------+       +---------+       +---------+
```
