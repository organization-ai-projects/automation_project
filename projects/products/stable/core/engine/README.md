# Engine Documentation

This directory contains the central hub and execution authority for the automation project.

## Role in the Project

This product is responsible for being the single point of communication in the system. All components (UIs, backends, launcher) connect to Engine via WebSocket. It handles authentication, authorization, audit logging, process management, and registry access.

It interacts mainly with:

- All UIs (central_ui, product UIs) - Via WebSocket
- All backends - Via WebSocket for command routing
- Launcher - For startup coordination
- Security library - For authentication and authorization

## Directory Structure

```
engine/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   ├── TOC.md
│   └── operations.md
└── src/               # Source code
    ├── main.rs
    └── ...
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `documentation/`: Additional documentation.
- `src/`: Source code.


## Overview

Engine is the **single point of communication** in the system. All components (UIs, backends, launcher) connect to Engine via WebSocket. No direct communication between components is allowed.

## Responsibilities

### WebSocket Hub

- Single entry point for all connections
- Routes commands from UIs to appropriate backends
- Broadcasts events from backends to subscribed UIs

### Execution Authority

- **Authentication**: Validates user sessions (UIs) and machine identities (backends)
- **Authorization**: Enforces permissions for all operations
- **Audit logging**: Records all actions for security and debugging

### Process Management

- Spawns backend processes on demand
- Monitors backend health and restarts on failure
- Graceful shutdown orchestration

### Registry Access

- Reads product and UI bundle metadata from the central registry
- Validates schema compatibility between components

## Architecture Position

```plaintext
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  central_ui │     │  product_ui │     │   launcher  │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │
       └───────────────────┼───────────────────┘
                           │
                    ┌──────▼──────┐
                    │   ENGINE    │  ◄── Single Hub
                    └──────┬──────┘
                           │
       ┌───────────────────┼───────────────────┐
       │                   │                   │
┌──────▼──────┐     ┌──────▼──────┐     ┌──────▼──────┐
│  backend_a  │     │  backend_b  │     │  backend_c  │
└─────────────┘     └─────────────┘     └─────────────┘
```

## Command/Event Model

### Commands (request/response)

```plaintext
ListProjects, ListProducts, ActivateProduct(id),
RunWorkflow(project_id, workflow_id), SpawnBackend(product_id)
```

### Events (stream)

```plaintext
LogLine(project_id, level, msg), Progress(workflow_id, pct),
ProductStateChanged(product_id, state), WorkflowFinished(workflow_id, result)
```

## Running

```bash
cargo run -p engine
```

## Configuration

Engine reads its configuration from the central registry at `.automation_project/registry.json`.

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/products/stable/core/engine/documentation/TOC.md)
- [Architecture](https://github.com/organization-ai-projects/automation_project/blob/main/documentation/technical_documentation/ARCHITECTURE.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
