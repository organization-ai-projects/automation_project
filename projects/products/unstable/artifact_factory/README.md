# Artifact Factory

A documentation bundle generator for automation artifacts.

## Crates

- **backend** — IPC backend; loads inputs, analyzes, renders docs, builds bundles
- **ui** — headless UI driver that communicates with the backend via stdin/stdout JSON IPC
- **tooling** — CLI for validating and hashing artifact bundles
