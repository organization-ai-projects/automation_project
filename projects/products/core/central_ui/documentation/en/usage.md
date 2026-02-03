# Central UI Usage

- [Back to Documentation Index](TOC.md)

## Purpose

Central UI is the main desktop interface. It loads product UI bundles at runtime and connects to the Engine over WebSocket.

## Key Behaviors

- Displays product list and system status
- Routes user actions to the Engine
- Shows real-time events and logs

## Startup

Central UI is usually started by the Launcher. For local development:

```bash
cargo run -p central_ui
```

To point at a custom Engine address:

```bash
cargo run -p central_ui -- --engine ws://localhost:9000
```
