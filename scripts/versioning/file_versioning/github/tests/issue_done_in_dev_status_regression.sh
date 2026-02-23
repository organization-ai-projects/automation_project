#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/github/issue_done_in_dev_status.sh"

# shellcheck source=scripts/common_lib/testing/shell_test_helpers.sh
source "${ROOT_DIR}/scripts/common_lib/testing/shell_test_helpers.sh"

TESTS_RUN=0
TESTS_FAILED=0

build_mock_bin() {
  local mock_dir="$1"
  mkdir -p "$mock_dir"

  cat > "${mock_dir}/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

args="$*"

if [[ "$args" == repo\ view* ]]; then
  echo "owner/repo"
  exit 0
fi

if [[ "$args" == label\ list* ]]; then
  if [[ "${MOCK_LABEL_EXISTS:-1}" == "1" ]]; then
    echo "done-in-dev"
  fi
  exit 0
fi

if [[ "$args" == pr\ view* && "$args" == *"--json state"* ]]; then
  echo "${MOCK_PR_STATE:-MERGED}"
  exit 0
fi

if [[ "$args" == pr\ view* && "$args" == *"--json title"* ]]; then
  echo "${MOCK_PR_TITLE:-Feature branch}"
  exit 0
fi

if [[ "$args" == pr\ view* && "$args" == *"--json body"* ]]; then
  echo "${MOCK_PR_BODY:-Closes #101}"
  exit 0
fi

if [[ "$args" == api\ repos/*/pulls/*/commits* ]]; then
  printf "%s\n" "${MOCK_PR_COMMITS:-feat: update\n\nCloses #102}"
  exit 0
fi

if [[ "$args" == issue\ view\ 101* && "$args" == *"--json state"* ]]; then
  echo "${MOCK_ISSUE_101_STATE:-OPEN}"
  exit 0
fi

if [[ "$args" == issue\ view\ 102* && "$args" == *"--json state"* ]]; then
  echo "${MOCK_ISSUE_102_STATE:-CLOSED}"
  exit 0
fi

if [[ "$args" == issue\ view\ 101* && "$args" == *"--json labels"* ]]; then
  if [[ "${MOCK_ISSUE_101_HAS_LABEL:-0}" == "1" ]]; then
    echo "done-in-dev"
  fi
  exit 0
fi

if [[ "$args" == issue\ view\ 202* && "$args" == *"--json labels"* ]]; then
  if [[ "${MOCK_ISSUE_202_HAS_LABEL:-1}" == "1" ]]; then
    echo "done-in-dev"
  fi
  exit 0
fi

if [[ "$args" == issue\ edit* ]]; then
  if [[ -n "${MOCK_GH_EDITS_LOG:-}" ]]; then
    printf "%s\n" "$args" >> "${MOCK_GH_EDITS_LOG}"
  fi
  exit 0
fi

exit 0
EOF
  chmod +x "${mock_dir}/gh"

  shell_test_write_passthrough_jq_mock "${mock_dir}"
}

run_case() {
  local name="$1"
  local expected_exit="$2"
  local expected_pattern="$3"
  local command="$4"
  shift 4

  TESTS_RUN=$((TESTS_RUN + 1))

  local tmp
  tmp="$(shell_test_mktemp_dir "done_in_dev_tests")"
  local out_file="${tmp}/out.txt"
  local err_file="${tmp}/err.txt"
  local merged="${tmp}/merged.txt"
  local status=0

  build_mock_bin "${tmp}/bin"

  (
    cd "${ROOT_DIR}"
    PATH="${tmp}/bin:${PATH}" \
    GH_REPO="owner/repo" \
    MOCK_GH_EDITS_LOG="${tmp}/edits.log" \
    "$@" \
    /bin/bash "${TARGET_SCRIPT}" ${command}
  ) >"${out_file}" 2>"${err_file}" || status=$?

  cat "${out_file}" "${err_file}" > "${merged}"

  if [[ "${status}" -ne "${expected_exit}" ]]; then
    echo "FAIL [${name}] expected exit ${expected_exit}, got ${status}"
    sed -n '1,80p' "${merged}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp}"
    return
  fi

  if [[ -n "${expected_pattern}" ]] && ! grep -qE -- "${expected_pattern}" "${merged}"; then
    echo "FAIL [${name}] missing pattern: ${expected_pattern}"
    sed -n '1,80p' "${merged}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp}"
    return
  fi

  echo "PASS [${name}]"
  rm -rf "${tmp}"
}

main() {
  echo "Running regression tests for issue_done_in_dev_status.sh"

  run_case \
    "dev-merge-adds-label-for-open-closing-issues" \
    0 \
    "Issue #101: added label 'done-in-dev'." \
    "--on-dev-merge --pr 55"

  run_case \
    "issue-closed-removes-label" \
    0 \
    "Issue #202: removed label 'done-in-dev'." \
    "--on-issue-closed --issue 202"

  run_case \
    "missing-label-definition-skips" \
    0 \
    "label 'done-in-dev' does not exist" \
    "--on-dev-merge --pr 55" \
    env MOCK_LABEL_EXISTS=0

  echo ""
  echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
