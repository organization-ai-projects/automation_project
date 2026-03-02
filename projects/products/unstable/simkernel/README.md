# SimKernel

A deterministic meta simulation engine with strict backend/UI separation.

## Crates

- **simkernel_backend** – headless simulation server (IPC server, ECS, deterministic scheduler, packs)
- **simkernel_ui** – client UI, zero business logic (IPC client only)
- **simkernel_tooling** – pack generator + contract validators

## Architecture

Communication between UI and backend is strictly via local IPC (stdin/stdout JSON lines).
