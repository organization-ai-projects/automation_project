#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/github/parent_issue_guard.sh"

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
printf "%s\n" "$*" >> "${MOCK_GH_ARGS_LOG}"

# repo view
if [[ "${1:-}" == "repo" && "${2:-}" == "view" ]]; then
  echo "org/repo"
  exit 0
fi

# issue view
if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  issue_number="${3:-}"
  if [[ "$issue_number" == "${MOCK_PARENT_NUMBER:-10}" ]]; then
    state="${MOCK_PARENT_STATE:-OPEN}"
    # body contains tasklist refs for children 20 and 21
    printf '{"number":%s,"title":"parent issue","state":"%s","url":"https://example.test/%s","body":"- [ ] #20\\n- [ ] #21"}\n' \
      "$issue_number" "$state" "$issue_number"
    exit 0
  fi
  if [[ "$issue_number" == "20" ]]; then
    printf '{"number":20,"title":"child one","state":"%s","url":"https://example.test/20"}\n' "${MOCK_CHILD_20_STATE:-CLOSED}"
    exit 0
  fi
  if [[ "$issue_number" == "21" ]]; then
    printf '{"number":21,"title":"child two","state":"%s","url":"https://example.test/21"}\n' "${MOCK_CHILD_21_STATE:-CLOSED}"
    exit 0
  fi
  exit 1
fi

# GraphQL queries - return pre-filtered output (as gh would with --jq)
if [[ "${1:-}" == "api" && "${2:-}" == "graphql" ]]; then
  joined="$*"
  # subIssues query - return empty (no native sub-issues; fall back to tasklist)
  if [[ "$joined" == *"subIssues"* ]]; then
    exit 0
  fi
  # parent ref query used by extract_parent_ref_from_github (--child mode)
  # returns "#<parent_number>" as jq would with: .data...parent.number // empty | "#"+tostring
  if [[ "$joined" == *"parent{number}"* ]]; then
    printf '#%s\n' "${MOCK_PARENT_NUMBER:-10}"
    exit 0
  fi
  exit 0
fi

# issue close
if [[ "${1:-}" == "issue" && "${2:-}" == "close" ]]; then
  printf "issue close %s\n" "$3" >> "${MOCK_GH_ARGS_LOG}"
  exit 0
fi

# issue reopen
if [[ "${1:-}" == "issue" && "${2:-}" == "reopen" ]]; then
  printf "issue reopen %s\n" "$3" >> "${MOCK_GH_ARGS_LOG}"
  exit 0
fi

# comments list (--paginate) - return empty array so no existing comment found
if [[ "${1:-}" == "api" && "$*" == *"--paginate"* ]]; then
  printf '[]\n'
  exit 0
fi

# all other api calls (comment create, label ops)
if [[ "${1:-}" == "api" ]]; then
  printf '{}\n'
  exit 0
fi

printf '{}\n'
MOCKEOF
  chmod +x "${mock_dir}/gh"
}

run_case() {
  local name="$1"
  local expected_exit="$2"
  local expected_pattern="$3"
  local command="$4"
  shift 4

  TESTS_RUN=$((TESTS_RUN + 1))

  local tmp
  tmp="$(shell_test_mktemp_dir "parent_issue_guard_tests")"
  local out_file="${tmp}/out.txt"
  local err_file="${tmp}/err.txt"
  local merged="${tmp}/merged.txt"
  local args_log="${tmp}/gh_args.log"
  touch "${args_log}"
  local status=0

  build_mock_bin "${tmp}/bin"

  (
    cd "${ROOT_DIR}"
    MOCK_GH_ARGS_LOG="${args_log}" \
    GH_REPO="org/repo" \
    PATH="${tmp}/bin:${PATH}" \
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
    cat "${args_log}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp}"
    return
  fi

  echo "PASS [${name}]"
  rm -rf "${tmp}"
}

run_close_check() {
  local name="$1"
  local should_close="$2"
  local command="$3"
  shift 3

  TESTS_RUN=$((TESTS_RUN + 1))

  local tmp
  tmp="$(shell_test_mktemp_dir "parent_issue_guard_tests")"
  local out_file="${tmp}/out.txt"
  local err_file="${tmp}/err.txt"
  local merged="${tmp}/merged.txt"
  local args_log="${tmp}/gh_args.log"
  touch "${args_log}"
  local status=0

  build_mock_bin "${tmp}/bin"

  (
    cd "${ROOT_DIR}"
    MOCK_GH_ARGS_LOG="${args_log}" \
    GH_REPO="org/repo" \
    PATH="${tmp}/bin:${PATH}" \
    "$@" \
    /bin/bash "${TARGET_SCRIPT}" ${command}
  ) >"${out_file}" 2>"${err_file}" || status=$?

  cat "${out_file}" "${err_file}" > "${merged}"

  local closed
  closed="$(grep -c "^issue close " "${args_log}" || true)"

  if [[ "${should_close}" == "yes" && "${closed}" -eq 0 ]]; then
    echo "FAIL [${name}] expected parent to be closed but it was not"
    sed -n '1,80p' "${merged}"
    cat "${args_log}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  elif [[ "${should_close}" == "no" && "${closed}" -gt 0 ]]; then
    echo "FAIL [${name}] expected parent NOT to be closed but close was called"
    sed -n '1,80p' "${merged}"
    cat "${args_log}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  else
    echo "PASS [${name}]"
  fi

  rm -rf "${tmp}"
}

main() {
  echo "Running regression tests for parent_issue_guard.sh"

  # All children closed => parent should be auto-closed
  run_case \
    "all-children-closed-closes-parent" \
    0 \
    "Closed parent issue #10 because all required children are closed" \
    "--issue 10" \
    env MOCK_PARENT_NUMBER=10 MOCK_PARENT_STATE=OPEN MOCK_CHILD_20_STATE=CLOSED MOCK_CHILD_21_STATE=CLOSED

  # One child still open => parent should NOT be closed
  run_close_check \
    "open-child-does-not-close-parent" \
    "no" \
    "--issue 10" \
    env MOCK_PARENT_NUMBER=10 MOCK_PARENT_STATE=OPEN MOCK_CHILD_20_STATE=OPEN MOCK_CHILD_21_STATE=CLOSED

  # Parent already closed, open children => parent should be reopened (strict-guard)
  run_case \
    "open-child-reopens-closed-parent" \
    0 \
    "Reopened parent issue #10 due to open required children" \
    "--issue 10 --strict-guard true" \
    env MOCK_PARENT_NUMBER=10 MOCK_PARENT_STATE=CLOSED MOCK_CHILD_20_STATE=OPEN MOCK_CHILD_21_STATE=CLOSED

  # --child mode: child closed, all siblings closed => parent auto-closed
  run_case \
    "child-mode-all-closed-closes-parent" \
    0 \
    "Closed parent issue #10 because all required children are closed" \
    "--child 20" \
    env MOCK_PARENT_NUMBER=10 MOCK_PARENT_STATE=OPEN MOCK_CHILD_20_STATE=CLOSED MOCK_CHILD_21_STATE=CLOSED

  echo ""
  echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
