#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/github/closure_hygiene_on_main_merge/run.sh"

# shellcheck source=scripts/common_lib/testing/shell_test_helpers.sh
source "${ROOT_DIR}/scripts/common_lib/testing/shell_test_helpers.sh"

TESTS_RUN=0
TESTS_FAILED=0

build_mock_bin() {
  local mock_dir="$1"
  mkdir -p "$mock_dir"

  cat > "${mock_dir}/gh" <<'MOCKEOF'
#!/usr/bin/env bash
set -euo pipefail

args="$*"
if [[ -n "${MOCK_GH_ARGS_LOG:-}" ]]; then
  printf "%s\n" "$args" >> "${MOCK_GH_ARGS_LOG}"
fi

if [[ "$args" == repo\ view* ]]; then
  echo "owner/repo"
  exit 0
fi

if [[ "$args" == issue\ list* && "$args" == *"--json number"* ]]; then
  printf "%s\n" "${MOCK_OPEN_ISSUES:-10}"
  exit 0
fi

if [[ "$args" == issue\ view\ 10* && "$args" == *"--json title,body,state,labels"* ]]; then
  state="${MOCK_PARENT_STATE:-OPEN}"
  body='- [ ] #20\n- [ ] #21'
  printf '{"title":"parent issue","body":"%s","state":"%s","labels":[]}\n' "$body" "$state"
  exit 0
fi

if [[ "$args" == issue\ view\ 20* && "$args" == *"--json title,body,state,labels"* ]]; then
  printf '{"title":"child 20","body":"","state":"%s","labels":[]}\n' "${MOCK_CHILD_20_STATE:-CLOSED}"
  exit 0
fi

if [[ "$args" == issue\ view\ 21* && "$args" == *"--json title,body,state,labels"* ]]; then
  printf '{"title":"child 21","body":"","state":"%s","labels":[]}\n' "${MOCK_CHILD_21_STATE:-CLOSED}"
  exit 0
fi

if [[ "$args" == api\ repos/*/issues/*/comments* && "$args" == *"--paginate"* ]]; then
  echo "[]"
  exit 0
fi

if [[ "$args" == issue\ close* || "$args" == issue\ reopen* || "$args" == issue\ edit* ]]; then
  exit 0
fi

if [[ "$args" == api\ repos/*/milestones?state=open* && "$args" == *"--jq"* ]]; then
  printf "%s\n" "${MOCK_MILESTONES_TSV:-1\tM1\t0}"
  exit 0
fi

if [[ "$args" == api\ -X\ PATCH\ repos/*/milestones/* ]]; then
  exit 0
fi

if [[ "$args" == api* ]]; then
  echo "{}"
  exit 0
fi

exit 0
MOCKEOF
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
  tmp="$(shell_test_mktemp_dir "closure_hygiene_tests")"
  local out_file="${tmp}/out.txt"
  local err_file="${tmp}/err.txt"
  local merged_file="${tmp}/merged.txt"
  local status=0

  build_mock_bin "${tmp}/bin"

  (
    cd "${ROOT_DIR}"
    PATH="${tmp}/bin:${PATH}" \
    MOCK_GH_ARGS_LOG="${tmp}/gh_args.log" \
    GH_REPO="owner/repo" \
    "$@" \
    /bin/bash "${TARGET_SCRIPT}" ${command}
  ) >"${out_file}" 2>"${err_file}" || status=$?

  cat "${out_file}" "${err_file}" >"${merged_file}"

  if [[ "${status}" -ne "${expected_exit}" ]]; then
    echo "FAIL [${name}] expected exit ${expected_exit}, got ${status}"
    sed -n '1,120p' "${merged_file}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp}"
    return
  fi

  if [[ -n "${expected_pattern}" ]] && ! grep -qE -- "${expected_pattern}" "${merged_file}"; then
    echo "FAIL [${name}] missing pattern: ${expected_pattern}"
    sed -n '1,120p' "${merged_file}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp}"
    return
  fi

  echo "PASS [${name}]"
  rm -rf "${tmp}"
}

main() {
  echo "Running regression tests for closure_hygiene_on_main_merge/run.sh"

  run_case \
    "all-children-closed-closes-parent-and-milestone" \
    0 \
    "Closed parent issue #10\\.|Closed milestone #1 \(M1\)\\.|Closure hygiene completed\\." \
    "" \
    env MOCK_PARENT_STATE=OPEN MOCK_CHILD_20_STATE=CLOSED MOCK_CHILD_21_STATE=CLOSED MOCK_MILESTONES_TSV=$'1\tM1\t0'

  run_case \
    "open-child-does-not-close-parent" \
    0 \
    "Closure hygiene completed\\." \
    "" \
    env MOCK_PARENT_STATE=OPEN MOCK_CHILD_20_STATE=OPEN MOCK_CHILD_21_STATE=CLOSED MOCK_MILESTONES_TSV=$'1\tM1\t1'

  run_case \
    "already-closed-parent-no-close-call" \
    0 \
    "Closure hygiene completed\\." \
    "" \
    env MOCK_PARENT_STATE=CLOSED MOCK_CHILD_20_STATE=CLOSED MOCK_CHILD_21_STATE=CLOSED MOCK_MILESTONES_TSV=""

  echo ""
  echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
