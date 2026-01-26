# Central UI

Desktop cockpit for `automation_project`.

## Overview

Central UI is the **main user interface** for the automation platform. It aggregates product UIs by loading their bundles at runtime, providing a unified experience without compile-time dependencies on individual products.

## Responsibilities

### UI Bundle Loading

- Loads product UI bundles (WASM + assets) at runtime
- No compile-time dependencies on product UI crates
- Supports hot-loading of new/updated products

### Engine Communication

- Single WebSocket connection to Engine
- Sends commands on behalf of the user
- Receives and displays real-time events

### Navigation & Administration

- Product switcher and navigation
- User session management
- System status dashboard

## Architecture Position

```text
┌─────────────────────────────────────────────────────────────┐
│                      CENTRAL UI                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │ Product A   │  │ Product B   │  │ Product C   │  ...    │
│  │ UI Bundle   │  │ UI Bundle   │  │ UI Bundle   │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
│        ▲                ▲                ▲                  │
│        └────────────────┼────────────────┘                  │
│                         │ Runtime Loading                   │
└─────────────────────────┼───────────────────────────────────┘
                          │
                   ┌──────▼──────┐
                   │   ENGINE    │  ◄── WebSocket Hub
                   └─────────────┘
```

## UI Bundle Contract

Central UI loads bundles that conform to this contract:

| Artifact           | Description                        |
| ------------------ | ---------------------------------- |
| `ui.wasm`          | Compiled Dioxus WASM module        |
| `assets/`          | Icons, CSS, static resources       |
| `ui_manifest.ron`  | Bundle metadata and configuration  |

## Key Principles

### No Business Logic

Central UI only:

- **Displays** state received from Engine
- **Sends** commands to Engine
- **Listens** to events from Engine

All business logic lives in backends, orchestrated by Engine.

### True Aggregation

- Products can be added/updated without recompiling Central UI
- Bundle discovery via the central registry
- Schema compatibility validated at load time

## Running

```bash
# Usually started by Launcher, but can run standalone
cargo run -p central_ui

# With custom Engine address
cargo run -p central_ui -- --engine ws://localhost:9000
```

## Configuration

Central UI reads product information from the registry at `.automation_project/registry.json`.

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/products/core/central_ui/documentation/TOC.md)
- [Architecture](https://github.com/organization-ai-projects/automation_project/blob/main/projects/products/core/central_ui/../../../../documentation/technical_documentation/ARCHITECTURE.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
