# simulation_compiler

Compiles a high-level simulation DSL (components / systems / events / reports)
into a deterministic SimKernel pack module scaffold.

## Binaries

| Binary | Description |
|--------|-------------|
| `simulation-compiler-backend` | Core DSL compiler: parse → validate → emit pack |
| `simulation-compiler-ui` | Terminal UI that drives the backend over IPC |
| `simulation-compiler-tooling` | Offline pack validator and golden checker |

## Invariants

- Same DSL input ⇒ identical emitted pack bytes + manifest hash (SHA-256).
- Forbidden patterns (wall-clock, nondeterministic iteration) rejected by ruleset.
- 1-file = 1-type policy in emitted scaffold.
- Does **not** execute other binaries at runtime.
