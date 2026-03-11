# neurosymbolic_moe

Advanced modular Mixture-of-Experts platform with routing, retrieval, memory, policy guards, and traceability.

## Binaries

- `neurosymbolic_moe_backend`: MoE runtime and orchestration pipeline.
- `neurosymbolic_moe_ui`: UI entrypoint crate.

## UI Runtime

- `neurosymbolic_moe_ui` uses a Dioxus launch path for `wasm32`.
- Native execution remains a minimal entrypoint for local workflows.
