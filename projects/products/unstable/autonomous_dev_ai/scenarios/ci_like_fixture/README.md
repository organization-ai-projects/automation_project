# CI-like Fixture Scenario

This scenario exercises a non-interactive end-to-end lifecycle run on a fixture repository and persists replay/report artifacts.

## Scope

It is intended to cover the non-test part of issue `#647` acceptance by providing a reproducible fixture execution path:

- lifecycle progression (`explore -> plan -> execute -> verify -> objectives`)
- policy/authz/risk-gate mediation
- PR/review orchestration path in non-interactive mode
- run replay and run report artifact generation

## Run

From repository root:

```bash
projects/products/unstable/autonomous_dev_ai/scripts/run_ci_like_fixture.sh
```

Optional output directory:

```bash
projects/products/unstable/autonomous_dev_ai/scripts/run_ci_like_fixture.sh /tmp/autonomous_dev_ai_ci_fixture
```

## Expected artifacts

The script fails fast if required artifacts are missing or if the run does not finish in `Done` state.

Default artifact directory:

- `projects/products/unstable/autonomous_dev_ai/artifacts/ci_like_fixture/agent_run_report.json`
- `projects/products/unstable/autonomous_dev_ai/artifacts/ci_like_fixture/agent_run_replay.json`
- `projects/products/unstable/autonomous_dev_ai/artifacts/ci_like_fixture/agent_run_replay.txt`
- `projects/products/unstable/autonomous_dev_ai/artifacts/ci_like_fixture/agent_audit.log`
