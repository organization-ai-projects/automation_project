# E2E Matrix Scenario

This scenario documents deterministic local reproduction for the autonomy orchestration end-to-end matrix.

## Covered Scenarios

The binary matrix test file (`tests/binary_e2e_matrix_tests.rs`) covers:

- happy path
- policy denial
- missing CI signal
- review rejection
- timeout
- crash-resume

Each scenario asserts:

- terminal state
- blocking reason codes (when blocked)
- expected output artifacts (`orchestrator_run_report.json`, `orchestrator_checkpoint.json`)

## Local Reproduction

Run only the matrix:

```bash
cargo test -p autonomy_orchestrator_ai --test binary_e2e_matrix_tests
```

Run full orchestrator test suite:

```bash
cargo test -p autonomy_orchestrator_ai
```

Use helper script:

```bash
projects/products/unstable/autonomy_orchestrator_ai/scripts/run_e2e_matrix.sh
```
