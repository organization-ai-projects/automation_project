#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
GITHUB_SCRIPTS_DIR="${ROOT_DIR}/scripts/versioning/file_versioning/github"

if ! command -v shellcheck >/dev/null 2>&1; then
  echo "Error: shellcheck is required." >&2
  exit 1
fi

run_shellcheck_group() {
  local label="$1"
  shift
  if [[ "$#" -eq 0 ]]; then
    return 0
  fi
  echo "ShellCheck: ${label}"
  shellcheck "$@"
}

mapfile -t standalone_scripts < <(
  find "${GITHUB_SCRIPTS_DIR}" -maxdepth 1 -type f -name '*.sh' | sort
)
mapfile -t nested_entrypoints < <(
  find "${GITHUB_SCRIPTS_DIR}" -mindepth 2 -maxdepth 2 -type f -name 'run.sh' | sort
)
mapfile -t test_scripts < <(
  find "${GITHUB_SCRIPTS_DIR}/tests" -maxdepth 1 -type f -name '*.sh' | sort
)
mapfile -t modular_lib_scripts < <(
  if [[ -d "${GITHUB_SCRIPTS_DIR}/lib" ]]; then
    find "${GITHUB_SCRIPTS_DIR}/lib" -maxdepth 1 -type f -name '*.sh' | sort
  fi
)
mapfile -t modular_pr_scripts < <(
  if [[ -d "${GITHUB_SCRIPTS_DIR}/pr" ]]; then
    find "${GITHUB_SCRIPTS_DIR}/pr" -type f -name '*.sh' | sort
  fi
)

# Standalone scripts are linted strictly, except SC2016 which is noisy for
# GraphQL payload literals containing "$var" placeholders.
standalone_targets=("${standalone_scripts[@]}" "${nested_entrypoints[@]}")
if [[ ${#standalone_targets[@]} -gt 0 ]]; then
  run_shellcheck_group "standalone scripts" -x -e SC2016 "${standalone_targets[@]}"
fi
# Test harnesses intentionally pass tokenized command snippets via ${command}.
run_shellcheck_group "test scripts" -x -e SC2086 "${test_scripts[@]}"

# Modular PR pipeline + shared lib use globals across sourced modules.
# We intentionally suppress:
# - SC2154: globals referenced across modules
# - SC2034: globals assigned in one module and consumed in another
# - SC1091: dynamic/indirect source graph in this modular shell app
modular_targets=("${modular_pr_scripts[@]}" "${modular_lib_scripts[@]}")
if [[ ${#modular_targets[@]} -gt 0 ]]; then
  run_shellcheck_group "modular pr generator" \
    -x -e SC2154,SC2034,SC1091 \
    "${modular_targets[@]}"
fi

echo "ShellCheck regression: PASS"
