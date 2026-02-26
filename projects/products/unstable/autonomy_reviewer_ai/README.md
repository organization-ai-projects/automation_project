# Autonomy Reviewer AI (Unstable)

`autonomy_reviewer_ai` is a lightweight review binary intended to be orchestrated by
`autonomy_orchestrator_ai` during the `validation` stage.

It verifies delegated AI artifacts and emits a machine-readable review report.

## Usage

```bash
cargo run -p autonomy_reviewer_ai -- \
  . \
  ./out/reviewer \
  --manager-action-plan ./out/manager/action_plan.json \
  --manager-run-report ./out/manager/run_report.json \
  --executor-run-report ./out/executor/agent_run_report.json \
  --executor-audit-log ./out/executor/audit.log
```

Optional strict mode:

```bash
cargo run -p autonomy_reviewer_ai -- ... --strict
```

Optional validation invocation execution inside `<repo_root>`:

```bash
cargo run -p autonomy_reviewer_ai -- \
  . \
  ./out/reviewer \
  --validation-bin cargo --validation-arg check --validation-arg -p --validation-arg autonomy_orchestrator_ai \
  --validation-bin cargo --validation-arg test --validation-arg -p --validation-arg autonomy_orchestrator_ai --validation-arg --test --validation-arg binary_e2e_matrix_tests
```

## Output

- `review_report.json` in the output directory
- includes `findings`, `warnings`, and prioritized `next_step_plan`

Exit codes:

- `0` review passed
- `1` review failed
- `2` invalid CLI usage
