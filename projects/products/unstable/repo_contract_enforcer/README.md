# repo_contract_enforcer

Deterministic repository contract enforcer with strict `backend/ui` separation.

## Crates

- `repo_contract_enforcer_backend` (bin-only): reads JSONL requests from stdin and writes JSONL responses to stdout.
- `repo_contract_enforcer_ui` (bin-only): CLI client that spawns backend and prints human or JSON output.

## Commands

- Backend: `repo_contract_enforcer_backend serve`
- UI:
  - `repo_contract_enforcer_ui check --root <path> [--mode auto|strict|relaxed] [--json]`
  - `repo_contract_enforcer_ui check-product --path <product_path> [--mode auto|strict|relaxed] [--json]`

## Exit Codes

- `0`: success / non-blocking result
- `2`: invalid CLI usage
- `3`: blocking violations in strict mode (UI)
- `5`: internal error

## Tests

- Backend fixtures and golden reports live under:
  - `backend/tests/fixtures/repos`
  - `backend/tests/fixtures/golden`
- Run:
  - `cargo test -p repo_contract_enforcer_backend`
  - `cargo test -p repo_contract_enforcer_ui`
