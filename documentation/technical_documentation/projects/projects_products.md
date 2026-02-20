# Products and Workspace Components

- [Back to Projects Index](../TOC.md)

## Product Definition

A product = `metadata.ron` + `backend` + `ui bundle`.

For details on `metadata.ron`, see [Metadata](../metadata.md). `metadata.ron` uses hex `ProtocolId` values for `id`.

## Backend Definition

- Separate process (backend service)
- Machine authenticated with `engine`
- Exposes no public ports
- Communicates only with `engine`

## 1. Core Products

The **core products** are foundational components that ensure the proper functioning of the entire system. They are located in `products/core`.

### 1.1 Engine (`engine`)

Core of the system.

Responsibilities:

- Management of the `automation_project` workspace
- Global registry of projects (see [Registry](../registry.md))
- Project loading and lifecycle management
- Creation of `ProjectContext`
- Workflow execution

> `engine` is the logical core of the product.

---

### 1.2 Launcher (`launcher`)

Initial startup component.

Responsibilities:

- Initialization of critical components (engine, central_ui, watcher).
- Management of bootstrap commands.
- Supervision delegated to the `watcher` after startup.

> `launcher` is the main entry point of the system.

---

### 1.3 Central UI (`central_ui`)

Central user interface.

Responsibilities:

- Product administration.
- Aggregation of UIs from different products.
- Navigation between products.

> `central_ui` provides a unified view for end users.

---

### 1.4 Watcher (`watcher`)

Global supervisor.

#### Responsibilities

- **Monitoring of critical components** :
  - Monitors the `launcher`, `engine`, and `central_ui`.
  - Regularly pings components to check their status.
- **Automatic restart** :
  - Restarts failing components in case of crash or non-response.
  - Implements exponential backoff logic to avoid restart loops.
- **Log management** :
  - Logs critical events (crashes, restarts) to a dedicated log file.
- **Flexible configuration** :
  - Allows defining monitored components, ping intervals, and restart policies via a configuration file (`watcher.toml`).

The `watcher` never communicates with projects or workflows. It is strictly limited to supervising the core executables.

#### Example configuration (`watcher.toml`)

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

## 2. User Interfaces (UI)

### Dedicated UIs for Products

- Each product (e.g., `app`) includes a **dedicated UI** for its specific features.
- A **common library** in `projects/libraries/layers/domain/ui` provides reusable components for all UIs.
- A **registry mechanism** centralizes the list of products and their available UIs (see [Registry](../registry.md)).
- A **central UI** (global dashboard) allows:
  - Product administration.
  - Aggregation of UIs from different products.
  - Navigation between products.

---

## 3. `products/core`

- **Namespace** : Contains the main executables (launcher, engine, central_ui).
- **Rules** :
  - Each sub-folder is a distinct binary crate.
  - Shared dependencies via `projects/libraries/core/foundation/common` and `projects/libraries/core/contracts/protocol`.

This architecture ensures robust supervision and reduces single points of failure (SPOF) by isolating responsibilities between components.

Execution hierarchy:
launcher -> (starts engine, central_ui, watcher) -> watcher supervises core -> engine orchestrates products and UIs

## 4. AI Consumption Rule

- Products that need AI capabilities should depend on `projects/libraries/layers/orchestration/ai`.
- Direct product dependencies on `projects/libraries/layers/domain/neural` or `projects/libraries/layers/domain/symbolic` are discouraged by architecture and should be avoided in favor of the orchestrator path.
