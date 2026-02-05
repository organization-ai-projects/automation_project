# Launcher Documentation

This directory contains the bootstrap binary for the automation project.

## Role in the Project

This product is responsible for being the entry point for starting the automation platform. It initializes the environment, starts core services in the correct order (Engine → Watcher → Central UI), and then exits once the system is running.

It interacts mainly with:

- Engine - Starts and waits for ready
- Watcher - Starts for supervision
- Central UI - Starts the desktop interface
- Registry - Reads configuration

## Directory Structure

```
launcher/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   ├── TOC.md
│   └── usage.md
└── src/               # Source code
    ├── main.rs
    └── ...
```

## Overview

Launcher is the **entry point** for starting the automation platform. It initializes the environment, starts core services in the correct order, and then exits once the system is running.

## Responsibilities

### Bootstrap Sequence

1. Validate environment and configuration
2. Start **Engine** (WebSocket hub)
3. Wait for Engine to be ready
4. Start **Watcher** (external supervisor)
5. Start **Central UI** (desktop cockpit)
6. Exit (supervision delegated to Watcher)

### Environment Setup

- Reads and validates the central registry
- Sets up required directories and files
- Configures logging and environment variables

### Process Wiring

- Ensures core services start in dependency order
- Passes configuration to child processes
- Handles startup failures gracefully

## Startup Flow

```text
┌──────────┐
│ Launcher │
└────┬─────┘
     │
     ├──► Start Engine ──► Wait for ready
     │
     ├──► Start Watcher
     │
     ├──► Start Central UI
     │
     └──► Exit (success)
           │
           ▼
    ┌─────────────┐
    │   Watcher   │  ◄── Takes over supervision
    └─────────────┘
```

## Running

```bash
# Start the entire platform
cargo run -p launcher

# Or use the built binary
./target/release/launcher
```

## Exit Codes

| Code | Meaning                                  |
| ---- | ---------------------------------------- |
| 0    | Successful startup, all services running |
| 1    | Configuration error                      |
| 2    | Engine failed to start                   |
| 3    | Watcher failed to start                  |
| 4    | Central UI failed to start               |

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/products/core/launcher/documentation/TOC.md)
- [Architecture](https://github.com/organization-ai-projects/automation_project/blob/main/documentation/technical_documentation/ARCHITECTURE.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
