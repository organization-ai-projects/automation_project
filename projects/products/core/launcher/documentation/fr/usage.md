# Launcher Utilisation

- [Retour à Index de Documentation](TOC.md)

## Purpose

Launcher bootstraps the core services (`engine`, `watcher`, `central_ui`) in the correct order.

## Startup

```bash
cargo run -p launcher
```

## Notes

- Launcher is intended for workspace operators and development environments.
- In production, services may be started by a system supervisor instead.
