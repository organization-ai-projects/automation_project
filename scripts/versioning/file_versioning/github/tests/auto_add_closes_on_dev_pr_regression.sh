#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/github/auto_add_closes_on_dev_pr.sh"

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

if [[ "$args" == pr\ view* && "$args" == *"--json number,state,baseRefName,title,body,author"* ]]; then
  cat <<JSON
{"number":55,"state":"${MOCK_PR_STATE:-OPEN}","baseRefName":"${MOCK_PR_BASE:-dev}","title":"${MOCK_PR_TITLE:-feat: sample}","body":"${MOCK_PR_BODY:-Part of #101}","author":{"login":"${MOCK_PR_AUTHOR:-devuser}"}}
JSON
  exit 0
fi

if [[ "$args" == api\ repos/*/pulls/*/commits* ]]; then
  printf "%s\n" "${MOCK_PR_COMMITS:-feat: sample\n\nPart of #101}"
  exit 0
fi

if [[ "$args" == issue\ view\ 101* && "$args" == *"--json assignees"* ]]; then
  if [[ "${MOCK_ISSUE_101_MULTI:-0}" == "1" ]]; then
    echo "${MOCK_PR_AUTHOR:-devuser}"
    echo "pairdev"
  elif [[ "${MOCK_ISSUE_101_UNASSIGNED:-0}" == "1" ]]; then
    :
  else
    echo "${MOCK_ISSUE_101_ASSIGNEE:-${MOCK_PR_AUTHOR:-devuser}}"
  fi
  exit 0
fi

if [[ "$args" == pr\ edit* ]]; then
  if [[ -n "${MOCK_PR_EDIT_LOG:-}" ]]; then
    printf "%s\n" "$args" >> "${MOCK_PR_EDIT_LOG}"
  fi
  exit 0
fi

exit 0
EOF
  chmod +x "${mock_dir}/gh"

  cat > "${mock_dir}/jq" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
/usr/bin/jq "$@"
EOF
  chmod +x "${mock_dir}/jq"
}

run_case() {
  local name="$1"
  local expected_exit="$2"
  local expected_pattern="$3"
  local command="$4"
  shift 4

  TESTS_RUN=$((TESTS_RUN + 1))

  local tmp out_file err_file merged status
  tmp="$(shell_test_mktemp_dir "auto_add_closes_tests")"
  out_file="${tmp}/out.txt"
  err_file="${tmp}/err.txt"
  merged="${tmp}/merged.txt"
  status=0

  build_mock_bin "${tmp}/bin"

  (
    cd "${ROOT_DIR}"
    PATH="${tmp}/bin:${PATH}" \
    GH_REPO="owner/repo" \
    MOCK_PR_EDIT_LOG="${tmp}/pr_edit.log" \
    "$@" \
    /bin/bash "${TARGET_SCRIPT}" ${command}
  ) >"${out_file}" 2>"${err_file}" || status=$?

  cat "${out_file}" "${err_file}" > "${merged}"

  if [[ "${status}" -ne "${expected_exit}" ]]; then
    echo "FAIL [${name}] expected exit ${expected_exit}, got ${status}"
    sed -n '1,120p' "${merged}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp}"
    return
  fi

  if [[ -n "${expected_pattern}" ]] && ! grep -qE -- "${expected_pattern}" "${merged}"; then
    echo "FAIL [${name}] missing pattern: ${expected_pattern}"
    sed -n '1,120p' "${merged}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp}"
    return
  fi

  echo "PASS [${name}]"
  rm -rf "${tmp}"
}

main() {
  echo "Running regression tests for auto_add_closes_on_dev_pr.sh"

  run_case \
    "adds-managed-closes-for-single-assignee-pr-author" \
    0 \
    "updated body with auto-managed Closes refs" \
    "--pr 55"

  run_case \
    "skips-when-closes-already-present" \
    0 \
    "no qualifying single-assignee issue found; nothing to enrich" \
    "--pr 55" \
    env MOCK_PR_BODY="Part of #101 Closes #101" MOCK_PR_COMMITS=""

  run_case \
    "skips-for-multi-assignee-issue" \
    0 \
    "no qualifying single-assignee issue found; nothing to enrich" \
    "--pr 55" \
    env MOCK_ISSUE_101_MULTI=1

  run_case \
    "skips-when-pr-target-is-not-dev" \
    0 \
    "does not target dev; skipping" \
    "--pr 55" \
    env MOCK_PR_BASE="main"

  echo ""
  echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
