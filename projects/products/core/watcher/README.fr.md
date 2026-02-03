# Watcher

External supervisor process for `automation_project` core services.

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

- [Index de Documentation](https://github.com/organization-ai-projects/automation_project/blob/main/projects/products/core/watcher/documentation/en/TOC.md)
- [Architecture](https://github.com/organization-ai-projects/automation_project/blob/main/documentation/technical_documentation/ARCHITECTURE.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
