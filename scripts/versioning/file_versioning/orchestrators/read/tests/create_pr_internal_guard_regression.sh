#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/orchestrators/read/create_pr.sh"

# shellcheck source=scripts/common_lib/testing/shell_test_helpers.sh
source "${ROOT_DIR}/scripts/common_lib/testing/shell_test_helpers.sh"

TESTS_RUN=0
TESTS_FAILED=0

run_case() {
  local name="$1"
  local expected_exit="$2"
  local expected_pattern="$3"
  local command="$4"

  TESTS_RUN=$((TESTS_RUN + 1))

  local tmp out_file err_file merged_file status
  tmp="$(shell_test_mktemp_dir "create_pr_internal_guard_tests")"
  out_file="${tmp}/out.txt"
  err_file="${tmp}/err.txt"
  merged_file="${tmp}/merged.txt"
  status=0

  (
    cd "${ROOT_DIR}"
    /bin/bash -c "${command}"
  ) >"${out_file}" 2>"${err_file}" || status=$?

  cat "${out_file}" "${err_file}" > "${merged_file}"

  if [[ "${status}" -ne "${expected_exit}" ]]; then
    echo "FAIL [${name}] expected exit ${expected_exit}, got ${status}"
    sed -n '1,80p' "${merged_file}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp}"
    return
  fi

  if [[ -n "${expected_pattern}" ]] && ! grep -qE -- "${expected_pattern}" "${merged_file}"; then
    echo "FAIL [${name}] missing pattern: ${expected_pattern}"
    sed -n '1,80p' "${merged_file}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp}"
    return
  fi

  echo "PASS [${name}]"
  rm -rf "${tmp}"
}

main() {
  echo "Running create_pr internal-guard regression tests"

  run_case \
    "direct-call-rejected" \
    2 \
    "internal-only and cannot be run directly" \
    "/bin/bash '${TARGET_SCRIPT}' --base dev --skip-tests"

  echo ""
  echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
