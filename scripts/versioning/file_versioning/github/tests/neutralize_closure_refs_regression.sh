#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
NEUTRALIZER="${ROOT_DIR}/scripts/versioning/file_versioning/github/neutralize_non_compliant_closure_refs.sh"
REEVALUATOR="${ROOT_DIR}/scripts/versioning/file_versioning/github/reevaluate_prs_on_issue_edit.sh"

# shellcheck source=scripts/common_lib/testing/shell_test_helpers.sh
source "${ROOT_DIR}/scripts/common_lib/testing/shell_test_helpers.sh"

TESTS_RUN=0
TESTS_FAILED=0

# Build a mock gh+jq+perl bin directory.
# Environment variables used by the mock:
#   MOCK_ISSUE_COMPLIANT: "1" → issue is compliant (empty reason); "0" → non-compliant
#   MOCK_PR_BODY: initial PR body
#   MOCK_PR_EDIT_LOG: file path to append edit calls to
#   MOCK_API_PULLS_JSON: raw JSON to return for pulls?state=open listing (for reevaluator tests)
build_mock_bin() {
  local mock_dir="$1"
  mkdir -p "$mock_dir"

  # ── gh mock ──────────────────────────────────────────────────────────────
  cat > "${mock_dir}/gh" <<'GHEOF'
#!/usr/bin/env bash
set -euo pipefail

args="$*"

if [[ -n "${MOCK_GH_ARGS_LOG:-}" ]]; then
  printf "%s\n" "$args" >> "${MOCK_GH_ARGS_LOG}"
fi

# repo view
if [[ "$args" == repo\ view* ]]; then
  echo "owner/repo"
  exit 0
fi

# pr view (body + url + number)
if [[ "$args" == pr\ view* && "$args" == *"--json body,url,number"* ]]; then
  body="${MOCK_PR_BODY:-Closes #42}"
  printf '{"body":%s,"url":"https://github.com/owner/repo/pull/1","number":1}\n' \
    "$(printf '%s' "$body" | jq -Rs '.')"
  exit 0
fi

# issue view (labels,title,body)
if [[ "$args" == issue\ view* && "$args" == *"--json labels,title,body"* ]]; then
  if [[ "${MOCK_ISSUE_COMPLIANT:-1}" == "1" ]]; then
    # Title matches ISSUE_TITLE_REGEX; body includes all required sections and Parent field
    compliant_body='## Context\n\nOK\n\n## Problem\n\nOK\n\n## Acceptance Criteria\n\nOK\n\n## Hierarchy\n\nParent: none'
    printf '{"labels":[],"title":"feat(scope): valid title","body":"%s"}\n' "$compliant_body"
  else
    printf '%s\n' '{"labels":[],"title":"bad title","body":"## Context\n\nOK"}'
  fi
  exit 0
fi

# pr edit (body update)
if [[ "$args" == pr\ edit* ]]; then
  if [[ -n "${MOCK_PR_EDIT_LOG:-}" ]]; then
    printf "%s\n" "$args" >> "${MOCK_PR_EDIT_LOG}"
  fi
  exit 0
fi

# issues/<pr>/comments (list)
if [[ "$args" == api\ repos/*/issues/*/comments* ]] && \
   [[ "$args" != *"-X PATCH"* ]] && [[ "$args" != *"-f body"* ]]; then
  echo "[]"
  exit 0
fi

# issues/comments/<id> (PATCH)
if [[ "$args" == api\ -X\ PATCH\ repos/*/issues/comments/* ]]; then
  exit 0
fi

# issues/<pr>/comments (POST)
if [[ "$args" == api\ repos/*/issues/*/comments* ]]; then
  exit 0
fi

# pulls?state=open listing (for reevaluator)
if [[ "$args" == api\ repos/*/pulls* ]]; then
  if [[ -n "${MOCK_API_PULLS_JSON:-}" ]]; then
    printf '%s\n' "$MOCK_API_PULLS_JSON"
  else
    echo "[]"
  fi
  exit 0
fi

exit 0
GHEOF
  chmod +x "${mock_dir}/gh"

  # ── jq mock: real jq passthrough ─────────────────────────────────────────
  # We need real jq for the reevaluator JSON parsing; use actual jq if present.
  if command -v jq >/dev/null 2>&1; then
    ln -sf "$(command -v jq)" "${mock_dir}/jq"
  else
    shell_test_write_passthrough_jq_mock "${mock_dir}"
  fi

  # ── perl passthrough ─────────────────────────────────────────────────────
  if command -v perl >/dev/null 2>&1; then
    ln -sf "$(command -v perl)" "${mock_dir}/perl"
  fi
}

run_case() {
  local name="$1"
  local expected_exit="$2"
  local expected_pattern="$3"
  local target="$4"
  local command="$5"
  local expected_edit_pattern="${6:-}"
  shift 6

  TESTS_RUN=$((TESTS_RUN + 1))

  local tmp
  tmp="$(shell_test_mktemp_dir "closure_neutralizer_tests")"
  local out_file="${tmp}/out.txt"
  local err_file="${tmp}/err.txt"
  local merged="${tmp}/merged.txt"
  local edit_log="${tmp}/edits.log"
  local status=0

  build_mock_bin "${tmp}/bin"

  (
    cd "${ROOT_DIR}"
    PATH="${tmp}/bin:${PATH}" \
    GH_REPO="owner/repo" \
    MOCK_PR_EDIT_LOG="${edit_log}" \
    MOCK_GH_ARGS_LOG="${tmp}/gh_args.log" \
    "$@" \
    /bin/bash "$target" ${command}
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
      else
        echo "  (edit log not created)"
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
  echo "Running regression tests for neutralize_non_compliant_closure_refs.sh and reevaluate_prs_on_issue_edit.sh"

  # ── neutralizer: missing --pr arg ────────────────────────────────────────
  run_case \
    "neutralizer-missing-pr-arg" \
    2 \
    "--pr is required" \
    "$NEUTRALIZER" \
    "" \
    ""

  # ── neutralizer: non-compliant issue gets neutralized ────────────────────
  run_case \
    "neutralizer-non-compliant-issue-adds-rejected" \
    0 \
    "Closure neutralization evaluated" \
    "$NEUTRALIZER" \
    "--pr 1" \
    "pr edit" \
    env MOCK_ISSUE_COMPLIANT=0 MOCK_PR_BODY="Closes #42"

  # ── neutralizer: compliant issue body left untouched ────────────────────
  run_case \
    "neutralizer-compliant-issue-no-change" \
    0 \
    "Closure neutralization evaluated" \
    "$NEUTRALIZER" \
    "--pr 1" \
    "" \
    env MOCK_ISSUE_COMPLIANT=1 MOCK_PR_BODY="Closes #42"

  # ── neutralizer: already-neutralized body is un-neutralized when compliant
  run_case \
    "neutralizer-removes-rejected-when-issue-becomes-compliant" \
    0 \
    "Closure neutralization evaluated" \
    "$NEUTRALIZER" \
    "--pr 1" \
    "pr edit" \
    env MOCK_ISSUE_COMPLIANT=1 MOCK_PR_BODY="Closes rejected #42"

  # ── neutralizer: already-neutralized body kept when still non-compliant ──
  run_case \
    "neutralizer-keeps-rejected-when-still-non-compliant" \
    0 \
    "Closure neutralization evaluated" \
    "$NEUTRALIZER" \
    "--pr 1" \
    "" \
    env MOCK_ISSUE_COMPLIANT=0 MOCK_PR_BODY="Closes rejected #42"

  # ── reevaluator: missing --issue arg ────────────────────────────────────
  run_case \
    "reevaluator-missing-issue-arg" \
    2 \
    "--issue is required" \
    "$REEVALUATOR" \
    "" \
    ""

  # ── reevaluator: no open PRs found ──────────────────────────────────────
  run_case \
    "reevaluator-no-prs-found" \
    0 \
    "No open PRs found" \
    "$REEVALUATOR" \
    "--issue 42" \
    "" \
    env MOCK_API_PULLS_JSON="[]"

  # ── reevaluator: finds and evaluates PR referencing the issue ────────────
  run_case \
    "reevaluator-evaluates-referencing-pr" \
    0 \
    "Re-evaluation complete.*1 PR" \
    "$REEVALUATOR" \
    "--issue 42" \
    "" \
    env MOCK_ISSUE_COMPLIANT=1 \
        MOCK_PR_BODY="Closes #42" \
        MOCK_API_PULLS_JSON='[{"number":1,"body":"Closes #42"}]'

  # ── reevaluator: PR not referencing the issue is ignored ─────────────────
  run_case \
    "reevaluator-ignores-unrelated-pr" \
    0 \
    "No open PRs found" \
    "$REEVALUATOR" \
    "--issue 42" \
    "" \
    env MOCK_API_PULLS_JSON='[{"number":2,"body":"Closes #99"}]'

  echo ""
  echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
