# spec_to_runtime_compiler

Compiles formal-ish state-transition specs (states, transitions, invariants)
into a deterministic runnable backend skeleton + tests.

## Binaries

- `spec_to_runtime_compiler_backend`: Core spec compiler: parse -> validate -> emit runtime.
- `spec_to_runtime_compiler_ui`: Terminal UI that drives the backend over IPC.

## DSL

```text
state Idle {}
state Running { tick: u64 }
state Done { result: u32 }

transition Idle -> Running on start {}
transition Running -> Done on complete {}

invariant no_self_loop "no state may transition to itself"
```

## Invariants

- Same spec input => identical emitted project bytes + manifest hash (SHA-256).
- Forbidden patterns (wall-clock, nondeterministic iteration) rejected by ruleset.
- Emitted runner forbids wall-clock usage; uses logical ticks.
- Emitted outputs include canonical manifests and hashes.

## IPC Protocol

Requests: LoadSpec, ValidateSpec, CompileDryRun, CompileWrite, GetCompileReport, Shutdown
Responses: Ok, Error, CompileReport { manifest_hash, report_json }
