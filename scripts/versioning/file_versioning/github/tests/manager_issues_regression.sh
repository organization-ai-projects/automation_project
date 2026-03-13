#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_BIN="${ROOT_DIR}/target/debug/versioning_automation"

# shellcheck source=scripts/common_lib/testing/shell_test_helpers.sh
source "${ROOT_DIR}/scripts/common_lib/testing/shell_test_helpers.sh"

TESTS_RUN=0
TESTS_FAILED=0

build_mock_bin() {
  local mock_dir="$1"
  mkdir -p "${mock_dir}"

  cat >"${mock_dir}/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if [[ -n "${MOCK_GH_ARGS_LOG:-}" ]]; then
  printf "%s\n" "$*" >> "${MOCK_GH_ARGS_LOG}"
fi

# manager_issues update now validates issue contract by reading current issue
# content through `gh issue view --json title,body,labels`.
# Return a minimal compliant payload for that call.
if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  if [[ " $* " == *" --json title,body,labels "* ]]; then
    cat <<'JSON'
{"title":"feat(shell): valid title","body":"## Context\nx\n\n## Problem\ny\n\n## Acceptance Criteria\nDone when :\n\n- [ ] z\n\n## Hierarchy\nParent: none","labels":[{"name":"issue"}]}
JSON
    exit 0
  fi
fi

exit 0
EOF
  chmod +x "${mock_dir}/gh"
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

  (
    cd "${ROOT_DIR}"
    PATH="${tmp}/bin:${PATH}" \
      MOCK_GH_ARGS_LOG="${tmp}/gh_args.log" \
      /bin/bash -c "${command}"
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
  echo "Running manager_issues regression tests"
  cargo build -q -p versioning_automation

  run_case \
    "create-dry-run" \
    0 \
    "Dry-run mode\\. Issue was not created\\." \
    "'${TARGET_BIN}' issue create --title 'feat(shell): x' --context 'ctx' --problem 'pb' --acceptance 'a1' --dry-run"

  run_case \
    "create-can-disable-default-issue-label" \
    0 \
    "Dry-run mode\\. Issue was not created\\." \
    "'${TARGET_BIN}' issue create --title 'feat(shell): x' --context 'ctx' --problem 'pb' --acceptance 'a1' --no-default-issue-label --dry-run"

  run_case \
    "create-passes-through-assignee-and-related-refs" \
    0 \
    "Dry-run mode\\. Issue was not created\\." \
    "'${TARGET_BIN}' issue create --title 'feat(shell): x' --context 'ctx' --problem 'pb' --acceptance 'a1' --assignee 'octocat' --related-issue '#12' --related-pr '#34' --dry-run"

  run_case \
    "read-lists-issues" \
    0 \
    "" \
    "'${TARGET_BIN}' issue read && grep -q -- 'issue list' \"\$MOCK_GH_ARGS_LOG\""

  run_case \
    "read-views-single-issue-with-json" \
    0 \
    "" \
    "'${TARGET_BIN}' issue read --issue 42 --repo myorg/repo --json number,title --jq '.number' && grep -q -- 'issue view 42 -R myorg/repo --json number,title --jq .number' \"\$MOCK_GH_ARGS_LOG\""

  run_case \
    "read-rejects-non-numeric-issue" \
    2 \
    "--issue requires a positive integer" \
    "'${TARGET_BIN}' issue read --issue nope"

  run_case \
    "update-requires-edit-args" \
    2 \
    "update requires at least one edit option" \
    "'${TARGET_BIN}' issue update --issue 12"

  run_case \
    "update-applies-gh-edit" \
    0 \
    "Issue #12 updated\\." \
    "'${TARGET_BIN}' issue update --issue 12 --title 'feat(shell): y'"

  run_case \
    "close-works-with-reason" \
    0 \
    "Issue #12 closed \\(reason: completed\\)\\." \
    "'${TARGET_BIN}' issue close --issue 12 --reason completed"

  run_case \
    "reopen-works" \
    0 \
    "Issue #12 reopened\\." \
    "'${TARGET_BIN}' issue reopen --issue 12"

  run_case \
    "delete-soft-closes-not-planned" \
    0 \
    "Issue #12 soft-deleted \\(closed with reason: not_planned\\)\\." \
    "'${TARGET_BIN}' issue delete --issue 12 --repo my/repo && grep -q -- 'issue close 12 --reason not_planned -R my/repo' \"\$MOCK_GH_ARGS_LOG\""

  echo ""
  echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
