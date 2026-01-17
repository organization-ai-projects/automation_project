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

### WebSocket Communication

- **Single Hub**: The engine acts as the sole WebSocket connection point for UIs and backend processes of products.
- **Authorized Clients**:
  - **Users**: via login/session.
  - **Agents**: via tokens or certificates.
  - **System Client (launcher)**: bootstrap authorized for limited commands (start engine, healthcheck, open UI).
- **Stable Contract**: Messages structured into commands (request/response) and events (stream).
  - **Examples of Commands**:
    - `ListProjects`
    - `ActivateProduct(product_id)`
    - `RunWorkflow(project_id, workflow_id)`
  - **Examples of Events**:
    - `LogLine(project_id, level, message)`
    - `Progress(workflow_id, pct)`
    - `WorkflowFinished(workflow_id, result)`
