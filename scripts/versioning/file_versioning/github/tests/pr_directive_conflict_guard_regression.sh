#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/github/pr_directive_conflict_guard/run.sh"

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

if [[ "$args" == pr\ view* && "$args" == *"--json body,url,number"* ]]; then
  body="${MOCK_PR_BODY:-Closes #42}"
  printf '{"body":%s,"url":"https://github.com/owner/repo/pull/1","number":1}\n' \
    "$(printf '%s' "$body" | jq -Rs '.')"
  exit 0
fi

if [[ "$args" == api\ repos/*/pulls/*/commits* ]]; then
  printf "%s\n" "${MOCK_PR_COMMITS:-}"
  exit 0
fi

if [[ "$args" == pr\ edit* ]]; then
  if [[ -n "${MOCK_PR_EDIT_LOG:-}" ]]; then
    printf "%s\n" "$args" >> "${MOCK_PR_EDIT_LOG}"
  fi
  exit 0
fi

if [[ "$args" == api\ repos/*/issues/*/comments* ]]; then
  if [[ "$args" == *"--paginate"* ]]; then
    echo "[]"
  fi
  exit 0
fi

if [[ "$args" == api\ -X\ PATCH\ repos/*/issues/comments/* ]]; then
  exit 0
fi

exit 0
EOF
  chmod +x "${mock_dir}/gh"

  if command -v jq >/dev/null 2>&1; then
    ln -sf "$(command -v jq)" "${mock_dir}/jq"
  else
    shell_test_write_passthrough_jq_mock "${mock_dir}"
  fi

  if command -v perl >/dev/null 2>&1; then
    ln -sf "$(command -v perl)" "${mock_dir}/perl"
  fi
}

run_case() {
  local name="$1"
  local expected_exit="$2"
  local expected_pattern="$3"
  local command="$4"
  local expected_edit_pattern="${5:-}"
  shift 5

  TESTS_RUN=$((TESTS_RUN + 1))
  local tmp out_file err_file merged edit_log status
  tmp="$(shell_test_mktemp_dir "directive_conflict_guard_tests")"
  out_file="${tmp}/out.txt"
  err_file="${tmp}/err.txt"
  merged="${tmp}/merged.txt"
  edit_log="${tmp}/edits.log"
  status=0

  build_mock_bin "${tmp}/bin"

  (
    cd "${ROOT_DIR}"
    PATH="${tmp}/bin:${PATH}" \
    GH_REPO="owner/repo" \
    MOCK_PR_EDIT_LOG="${edit_log}" \
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

  if [[ -n "${expected_edit_pattern}" ]]; then
    if [[ ! -f "${edit_log}" ]] || ! grep -qE -- "${expected_edit_pattern}" "${edit_log}"; then
      echo "FAIL [${name}] edit log missing pattern: ${expected_edit_pattern}"
      if [[ -f "${edit_log}" ]]; then
        sed -n '1,20p' "${edit_log}"
      fi
      TESTS_FAILED=$((TESTS_FAILED + 1))
      rm -rf "${tmp}"
      return
    fi
  fi

  echo "PASS [${name}]"
  rm -rf "${tmp}"
}

main() {
  echo "Running regression tests for pr_directive_conflict_guard/run.sh"

  run_case \
    "missing-pr-arg" \
    2 \
    "--pr is required" \
    "" \
    ""

  run_case \
    "no-conflict-no-op" \
    0 \
    "Directive conflict guard evaluated" \
    "--pr 1" \
    "" \
    env MOCK_PR_BODY="Closes #42"

  run_case \
    "conflict-without-explicit-decision-resolves-from-history" \
    0 \
    "Directive conflict guard evaluated" \
    "--pr 1" \
    "pr edit" \
    env MOCK_PR_BODY=$'Closes #42\nReopen #42'

  run_case \
    "resolved-conflict-reopen-accepted" \
    0 \
    "Directive conflict guard evaluated" \
    "--pr 1" \
    "pr edit" \
    env MOCK_PR_BODY=$'Closes #42\nReopen #42\nDirective Decision: #42 => reopen'

  run_case \
    "resolved-conflict-close-neutralizes-reopen" \
    0 \
    "Directive conflict guard evaluated" \
    "--pr 1" \
    "[Rr]eopen rejected #42" \
    env MOCK_PR_BODY=$'Closes #42\nReopen #42\nDirective Decision: #42 => close'

  run_case \
    "multi-source-branches-require-explicit-decision" \
    8 \
    "Unresolved directive conflicts detected" \
    "--pr 1" \
    "pr edit" \
    env MOCK_PR_BODY=$'Closes #42\nReopen #42' \
        MOCK_PR_COMMITS=$'Merge pull request #100 from org/feature-a\nMerge pull request #101 from org/feature-b'

  echo ""
  echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
