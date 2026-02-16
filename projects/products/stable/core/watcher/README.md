# Watcher Documentation

This directory contains the external supervisor process for core services in the automation project.

## Role in the Project

This product is responsible for being an independent supervisor that runs outside the main system. It monitors core processes (Engine, Central UI, backends) and ensures high availability by automatically restarting failed services with exponential backoff.

It interacts mainly with:

- Engine - Monitors and restarts if needed
- Central UI - Monitors and restarts if needed
- Product backends - Monitors and restarts if needed
- Launcher - Started by launcher after Engine

## Directory Structure

```
watcher/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   ├── TOC.md
│   └── supervision.md
└── src/               # Source code
    ├── main.rs
    └── ...
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `documentation/`: Additional documentation.
- `src/`: Source code.
- `tests/`: Tests.


## Overview

Watcher is an **independent supervisor** that runs outside the main system. It monitors core processes and ensures high availability by automatically restarting failed services with exponential backoff.

## Responsibilities

### Process Monitoring

- Watches Engine, Central UI, and product backends
- Detects crashes, hangs, and unresponsive processes
- Performs periodic health checks

### Automatic Recovery

- Restarts failed processes automatically
- Implements exponential backoff to prevent restart loops
- Logs all restart events for debugging

### Health Checks

- Periodic ping to Engine WebSocket
- Process liveness verification
- Resource usage monitoring (optional)

## Supervision Strategy

```text
┌─────────────────────────────────────────────────┐
│                   WATCHER                       │
│  (runs independently, survives child crashes)   │
└───────┬─────────────────┬─────────────────┬─────┘
        │                 │                 │
        ▼                 ▼                 ▼
   ┌─────────┐      ┌─────────┐      ┌─────────────┐
   │ Engine  │      │Central  │      │  Backends   │
   │         │      │   UI    │      │ (optional)  │
   └─────────┘      └─────────┘      └─────────────┘
        │                 │                 │
        └─────────────────┴─────────────────┘
                          │
                    Restart on failure
                    with backoff
```

## Backoff Strategy

When a process fails repeatedly:

1. First restart: immediate
2. Second restart: 1 second delay
3. Third restart: 2 seconds delay
4. Fourth restart: 4 seconds delay
5. ... (exponential up to max 60 seconds)

After successful uptime (e.g., 5 minutes), backoff resets.

## Running

```bash
# Usually started by launcher, but can run standalone
cargo run -p watcher

# With specific processes to watch
cargo run -p watcher -- --watch engine --watch central_ui
```

## Logging

Watcher writes operational logs to:

- stdout (default)
- Configured log file (if specified)
- System journal (on Linux with systemd)

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/products/stable/core/watcher/documentation/TOC.md)
- [Architecture](https://github.com/organization-ai-projects/automation_project/blob/main/documentation/technical_documentation/en/ARCHITECTURE.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
