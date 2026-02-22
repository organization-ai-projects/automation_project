#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"
cd "$ROOT_DIR"

cargo test -p autonomy_orchestrator_ai --test binary_e2e_matrix_tests
