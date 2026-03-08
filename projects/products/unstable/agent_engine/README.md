# Agent Engine

Incremental product for a deterministic, issue-driven coding agent runtime.

Current scope: V0 core engine foundation.

## Backend CLI

Run a task from JSON:

```bash
cargo run -p agent_engine_backend -- run \
  projects/products/unstable/agent_engine/backend/tests/fixtures/task_minimal.json
```

The command prints a structured `AgentOutcome` JSON containing:

- task id
- success flag
- step results and artifacts
- output key/value map
- execution logs
