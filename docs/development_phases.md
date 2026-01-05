# Development Phases

## Phase 1 – Foundation (v0)

- Creation of the `automation_project` workspace.
- Setup of the main crates:
  - `core`: Includes `launcher`, `engine`, `central_ui`, and `watcher`.
  - `symbolic`: Management of symbolic rules.
  - `ai`: AI orchestrator.
  - `ui`: Common UI components.
- Native support for multi-projects (products / libraries).
- Introduction of `metadata.ron` files for each project.
- Initial documentation automatically generated via `cargo doc`.

## Phase 2 – Intelligence and Automation

- Activation of the `neural` component for Rust code generation and feedback-based adjustments.
- Implementation of complex workflows with symbolic and neural orchestration.
- Addition of centralized supervision via the `watcher`.
- Centralization of metadata and UIs through the global registry.

## Phase 3 – Distribution

- Addition of remote engine mode with secure communication (WebSocket, TLS, auth).
- Setup of the central UI for administration and aggregation of product UIs.
- Scalability to handle a large number of simultaneous projects.
- Optimization of performance and workflows for production environments.

### Communication WebSocket

- **Hub unique** : Engine agit comme le seul point de connexion WebSocket pour les UIs et les processus backend des produits.
- **Clients autorisés** :
  - **Utilisateurs** : via login/session.
  - **Agents** : via tokens ou certificats.
  - **System Client (launcher)** : bootstrap autorisé pour des commandes limitées (start engine, healthcheck, open UI).
- **Contrat stable** : Messages structurés en commandes (request/response) et événements (stream).
  - **Exemples de commandes** :
    - `ListProjects`
    - `ActivateProduct(product_id)`
    - `RunWorkflow(project_id, workflow_id)`
  - **Exemples d’événements** :
    - `LogLine(project_id, level, message)`
    - `Progress(workflow_id, pct)`
    - `WorkflowFinished(workflow_id, result)`
