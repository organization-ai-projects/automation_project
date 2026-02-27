#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/github/manager_issues.sh"

# shellcheck source=scripts/common_lib/testing/shell_test_helpers.sh
source "${ROOT_DIR}/scripts/common_lib/testing/shell_test_helpers.sh"

TESTS_RUN=0
TESTS_FAILED=0

build_mock_bin() {
  local mock_dir="$1"
  mkdir -p "${mock_dir}"

  cat > "${mock_dir}/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if [[ -n "${MOCK_GH_ARGS_LOG:-}" ]]; then
  printf "%s\n" "$*" >> "${MOCK_GH_ARGS_LOG}"
fi

exit 0
EOF
  chmod +x "${mock_dir}/gh"
}

build_mock_create_script() {
  local script_path="$1"
  cat > "${script_path}" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf "%s\n" "$*" > "${MOCK_CREATE_ARGS_LOG}"
echo "$*"
if [[ "$*" == *"--dry-run"* ]]; then
  echo "Dry-run mode. Issue was not created."
fi
EOF
  chmod +x "${script_path}"
}

run_case() {
  local name="$1"
  local expected_exit="$2"
  local expected_pattern="$3"
  local command="$4"

  TESTS_RUN=$((TESTS_RUN + 1))

  local tmp
  tmp="$(shell_test_mktemp_dir "manager_issues_tests")"
  local out_file="${tmp}/out.txt"
  local err_file="${tmp}/err.txt"
  local merged_file="${tmp}/merged.txt"
  local status=0

  build_mock_bin "${tmp}/bin"
  build_mock_create_script "${tmp}/mock_create_issue.sh"

  (
    cd "${ROOT_DIR}"
    PATH="${tmp}/bin:${PATH}" \
    MOCK_GH_ARGS_LOG="${tmp}/gh_args.log" \
    MOCK_CREATE_ARGS_LOG="${tmp}/create_args.log" \
    MANAGER_ISSUES_CREATE_SCRIPT="${tmp}/mock_create_issue.sh" \
    /bin/bash -c "${command}"
  ) >"${out_file}" 2>"${err_file}" || status=$?

  cat "${out_file}" "${err_file}" > "${merged_file}"

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
  echo "Running manager_issues regression tests"

  run_case \
    "create-adds-default-issue-label" \
    0 \
    "--label issue" \
    "/bin/bash '${TARGET_SCRIPT}' create --title 'feat(shell): x' --context 'ctx' --problem 'pb' --acceptance 'a1' --dry-run"

  run_case \
    "create-can-disable-default-issue-label" \
    0 \
    "Dry-run mode\\. Issue was not created\\." \
    "/bin/bash '${TARGET_SCRIPT}' create --title 'feat(shell): x' --context 'ctx' --problem 'pb' --acceptance 'a1' --no-default-issue-label --dry-run && ! grep -q -- '--label issue' \"\$MOCK_CREATE_ARGS_LOG\""

  run_case \
    "read-lists-issues" \
    0 \
    "" \
    "/bin/bash '${TARGET_SCRIPT}' read && grep -q -- 'issue list' \"\$MOCK_GH_ARGS_LOG\""

  run_case \
    "read-views-single-issue-with-json" \
    0 \
    "" \
    "/bin/bash '${TARGET_SCRIPT}' read --issue 42 --repo myorg/repo --json number,title --jq '.number' && grep -q -- 'issue view 42 -R myorg/repo --json number,title --jq .number' \"\$MOCK_GH_ARGS_LOG\""

  run_case \
    "read-rejects-non-numeric-issue" \
    2 \
    "must be a positive integer" \
    "/bin/bash '${TARGET_SCRIPT}' read --issue nope"

  run_case \
    "update-requires-edit-args" \
    2 \
    "update requires at least one edit option" \
    "/bin/bash '${TARGET_SCRIPT}' update --issue 12"

  run_case \
    "update-applies-gh-edit" \
    0 \
    "Issue #12 updated\\." \
    "/bin/bash '${TARGET_SCRIPT}' update --issue 12 --title 'feat(shell): y'"

  run_case \
    "close-works-with-reason" \
    0 \
    "Issue #12 closed \\(reason: completed\\)\\." \
    "/bin/bash '${TARGET_SCRIPT}' close --issue 12 --reason completed"

  run_case \
    "reopen-works" \
    0 \
    "Issue #12 reopened\\." \
    "/bin/bash '${TARGET_SCRIPT}' reopen --issue 12"

  run_case \
    "delete-soft-closes-not-planned" \
    0 \
    "Issue #12 soft-deleted \\(closed with reason: not_planned\\)\\." \
    "/bin/bash '${TARGET_SCRIPT}' delete --issue 12 --repo my/repo && grep -q -- 'issue close 12 --reason not_planned -R my/repo' \"\$MOCK_GH_ARGS_LOG\""

  echo ""
  echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
