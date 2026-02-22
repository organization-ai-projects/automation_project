# Autonomy Orchestrator AI (V0 - Binary Orchestration)

`autonomy_orchestrator_ai` is an unstable product that orchestrates autonomous workflows through product binary invocation boundaries.

For bootstrap V0, this crate provides:

- explicit stage state machine (`planning`, `policy_ingestion`, `execution`, `validation`, `closure`)
- explicit terminal states (`done`, `blocked`, `failed`, `timeout`)
- deterministic transition recording with run identifier and timestamp
- JSON run report artifact for audit/replay style supervision

## Usage

```bash
cargo run -p autonomy_orchestrator_ai -- [output_dir]
```

Optional blocked simulation:

```bash
cargo run -p autonomy_orchestrator_ai -- [output_dir] --simulate-blocked
```

## Output

- `orchestrator_run_report.json`

This report includes machine-readable stage transitions and terminal outcome.
