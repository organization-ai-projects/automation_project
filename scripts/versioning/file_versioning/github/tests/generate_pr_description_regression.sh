#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/github/generate_pr_description.sh"
SNAPSHOT_DIR="${SCRIPT_DIR}/golden"

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
if [[ "${1:-}" == "repo" && "${2:-}" == "view" ]]; then
  echo "owner/repo"
  exit 0
fi
if [[ "${1:-}" == "api" && "${2:-}" == "repos/owner/repo/compare/dev...test-head" ]]; then
  first_line="fix(test): synthetic change"
  if [[ -n "${MOCK_GIT_LOG_ONELINE:-}" ]]; then
    first_line="$(echo "${MOCK_GIT_LOG_ONELINE}" | cut -d' ' -f2-)"
  fi
  full_message="${first_line}"
  if [[ -n "${MOCK_GIT_LOG_BODY:-}" ]]; then
    full_message="${full_message}

${MOCK_GIT_LOG_BODY}"
  fi

  if [[ "${3:-}" == "--jq" ]]; then
    jq_expr="${4:-}"
    if [[ "${jq_expr}" == *'split("\\n")[0]'* ]]; then
      printf "%s\n" "${first_line}"
    else
      printf "%s\n" "${full_message}"
    fi
  else
    escaped="$(printf "%s" "${full_message}" | sed ':a;N;$!ba;s/\n/\\n/g')"
    printf '{"commits":[{"commit":{"message":"%s"}}]}\n' "${escaped}"
  fi
  exit 0
fi
if [[ "${1:-}" == "api" && "${2:-}" == "repos/owner/repo/pulls/42/commits" ]]; then
  if [[ "${3:-}" == "--paginate" && "${4:-}" == "--jq" ]]; then
    if [[ -n "${MOCK_MAIN_PR_COMMIT_HEADLINES:-}" ]]; then
      printf "%s\n" "${MOCK_MAIN_PR_COMMIT_HEADLINES}"
    fi
    exit 0
  fi
fi
if [[ "${1:-}" == "pr" && "${2:-}" == "create" ]]; then
  echo "https://github.com/owner/repo/pull/999"
  exit 0
fi
if [[ "${1:-}" == "pr" && "${2:-}" == "view" && "${3:-}" == "42" ]]; then
  if [[ "${4:-}" == "--json" && "${5:-}" == "body" && "${6:-}" == "-q" ]]; then
    echo ""
    exit 0
  fi
  if [[ "${4:-}" == "--json" && "${5:-}" == "comments" && "${6:-}" == "-q" ]]; then
    echo ""
    exit 0
  fi
  if [[ "${4:-}" == "--json" && "${5:-}" == "baseRefName" && "${6:-}" == "-q" ]]; then
    echo "main"
    exit 0
  fi
  if [[ "${4:-}" == "--json" && "${5:-}" == "headRefName" && "${6:-}" == "-q" ]]; then
    echo "dev"
    exit 0
  fi
fi
if [[ "${1:-}" == "pr" && "${2:-}" == "view" && "${3:-}" == "693" ]]; then
  if [[ "${4:-}" == "--json" && "${5:-}" == "title,body,labels" ]]; then
    cat <<'JSON'
{"title":"feat(automation): re-evaluate PR closure neutralization on issue edit","body":"Fixes #690","labels":[{"name":"automation"}]}
JSON
    exit 0
  fi
fi
# pull lookup for issue numbers (used to discard PR refs): default to not-a-PR.
if [[ "$*" =~ ^api[[:space:]]+repos/.+/pulls/[0-9]+$ ]]; then
  exit 1
fi
exit 0
EOF
  chmod +x "${mock_dir}/gh"

  cat > "${mock_dir}/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ -n "${MOCK_GIT_ARGS_LOG:-}" ]]; then
  printf "%s\n" "$*" >> "${MOCK_GIT_ARGS_LOG}"
fi
if [[ "${1:-}" == "show-ref" ]]; then
  if [[ "${MOCK_GIT_SHOW_REF_EXIT:-0}" -ne 0 ]]; then
    exit "${MOCK_GIT_SHOW_REF_EXIT}"
  fi
  exit 0
fi
if [[ "${1:-}" == "rev-parse" && "${2:-}" == "--abbrev-ref" && "${3:-}" == "HEAD" ]]; then
  echo "test-head"
  exit 0
fi
if [[ "${1:-}" == "log" ]]; then
  if [[ "${2:-}" == "--oneline" && -n "${MOCK_GIT_LOG_ONELINE:-}" ]]; then
    printf "%s\n" "${MOCK_GIT_LOG_ONELINE}"
    exit 0
  fi
  if [[ "${2:-}" == "--format=%B" && -n "${MOCK_GIT_LOG_BODY:-}" ]]; then
    printf "%s\n" "${MOCK_GIT_LOG_BODY}"
    exit 0
  fi
  exit 0
fi
exit 0
EOF
  chmod +x "${mock_dir}/git"

  shell_test_write_passthrough_jq_mock "${mock_dir}"
}

build_no_gh_bin() {
  local mock_dir="$1"
  mkdir -p "${mock_dir}"

  cat > "${mock_dir}/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "rev-parse" && "${2:-}" == "--abbrev-ref" && "${3:-}" == "HEAD" ]]; then
  echo "test-head"
  exit 0
fi
if [[ "${1:-}" == "log" ]]; then
  exit 0
fi
exit 0
EOF
  chmod +x "${mock_dir}/git"

  shell_test_write_passthrough_jq_mock "${mock_dir}"
}

run_case() {
  local name="$1"
  local expected_exit="$2"
  local expected_pattern="$3"
  local mode="${4:-mock}"
  shift 4

  TESTS_RUN=$((TESTS_RUN + 1))

  local tmp
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  local out_file="${tmp}/out.txt"
  local err_file="${tmp}/err.txt"
  local status

  if [[ "${mode}" == "mock" ]]; then
    build_mock_bin "${tmp}/bin"
    (
      cd "${ROOT_DIR}"
      PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" "$@"
    ) >"${out_file}" 2>"${err_file}" || status=$?
  elif [[ "${mode}" == "no_gh" ]]; then
    build_no_gh_bin "${tmp}/bin"
    (
      cd "${ROOT_DIR}"
      PATH="${tmp}/bin" /bin/bash "${TARGET_SCRIPT}" "$@"
    ) >"${out_file}" 2>"${err_file}" || status=$?
  else
    (
      cd "${ROOT_DIR}"
      PATH="" /bin/bash "${TARGET_SCRIPT}" "$@"
    ) >"${out_file}" 2>"${err_file}" || status=$?
  fi

  status="${status:-0}"
  local merged="${tmp}/merged.txt"
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
  echo "Running regression matrix for generate_pr_description.sh"

  run_case "help" 0 "Usage:" mock --help
  run_case "missing-main-pr" 2 "MAIN_PR_NUMBER est requis" mock
  run_case "base-missing-value" 2 "--base requires a value" mock --base
  run_case "head-missing-value" 2 "--head requires a value" mock --head
  run_case "duplicate-mode-invalid" 2 "--duplicate-mode must be 'safe' or 'auto-close'" mock --duplicate-mode invalid --dry-run
  run_case "auto-edit-missing-value" 2 "--auto-edit requires a value" mock --auto-edit
  run_case "auto-edit-non-numeric" 2 "--auto-edit requires a numeric PR_NUMBER" mock --auto-edit abc --dry-run
  run_case "create-pr-without-dry-run" 2 "--create-pr is only supported with --dry-run" mock --create-pr 42
  run_case "allow-partial-without-create-pr" 2 "--allow-partial-create requires --create-pr" mock --allow-partial-create 42
  run_case "auto-edit-conflicts-with-create-pr" 2 "--auto-edit cannot be combined with --create-pr/--auto" mock --dry-run --create-pr --auto-edit 400
  run_case "auto-forbids-positional" 2 "--auto does not accept a positional OUTPUT_FILE" mock --auto output.md
  run_case "auto-edit-dry-run-forbids-positional" 2 "In --auto-edit dry-run mode, positional OUTPUT_FILE is not allowed" mock --dry-run --auto-edit 400 output.md
  run_case "auto-edit-main-forbids-output-file" 2 "In --auto-edit mode \\(MAIN_PR_NUMBER\\), positional OUTPUT_FILE is not allowed" mock --auto-edit 400 42 out.md
  run_case "dry-run-too-many-positionals" 2 "Too many positional arguments for --dry-run" mock --dry-run out.md extra.md
  run_case "dry-run-minimal" 0 "Generated file:" mock --dry-run --base dev --head test-head /tmp/pr_description_test.md
  run_case "auto-create-success-does-not-return-no-data" 0 "PR created:" mock --auto --base dev --head test-head --yes
  run_case "auto-edit-dry-run-in-memory" 0 "--auto-edit mode" mock --dry-run --auto-edit 400 --base dev --head test-head --yes
  run_case "dry-run-without-gh" 3 "command 'gh' not found" no_gh --dry-run --base dev --head test-head /tmp/pr_description_test_no_gh.md
  run_case "missing-gh-required-main-mode" 3 "command 'gh' not found" no_gh 42
  run_case "duplicate-mode-dry-run-output" 0 "Duplicate mode \\(safe\\): no duplicate declarations detected" mock --dry-run --base dev --head test-head --duplicate-mode safe
  run_case "debug-flag-emits-trace" 0 "\\[debug\\] extract_child_prs_dry" mock --dry-run --base dev --head test-head --debug

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  build_mock_bin "${tmp}/bin"
  args_log="${tmp}/gh_args_label.log"
  if (
    cd "${ROOT_DIR}"
    MOCK_GH_ARGS_LOG="${args_log}" \
    PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" --auto --base dev --head test-head --yes
  ) >/dev/null 2>&1; then
    if grep -q -- "--label pull-request" "${args_log}"; then
      echo "PASS [auto-create-adds-pull-request-label]"
    else
      echo "FAIL [auto-create-adds-pull-request-label] expected --label pull-request in gh pr create args"
      cat "${args_log}" || true
      TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
  else
    echo "FAIL [auto-create-adds-pull-request-label] script execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi
  rm -rf "${tmp}"

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  build_mock_bin "${tmp}/bin"
  out_md="${tmp}/main_parity.md"
  if (
    cd "${ROOT_DIR}"
    MOCK_MAIN_PR_COMMIT_HEADLINES="Merge pull request #693 from organization-ai-projects/copilot/fix-pr-closure-neutralization" \
    PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" 42 "${out_md}"
  ) >/dev/null 2>&1; then
    if grep -q -- "#693" "${out_md}" && grep -q -- "Closes #690" "${out_md}"; then
      echo "PASS [main-mode-detects-merge-pr-and-fixes-ref]"
    else
      echo "FAIL [main-mode-detects-merge-pr-and-fixes-ref] expected #693 and Closes #690"
      sed -n '1,220p' "${out_md}"
      TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
  else
    echo "FAIL [main-mode-detects-merge-pr-and-fixes-ref] script execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi
  rm -rf "${tmp}"

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  build_mock_bin "${tmp}/bin"
  out_md="${tmp}/body.md"
  if (
    cd "${ROOT_DIR}"
    PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" --dry-run --base dev --head test-head "${out_md}"
  ) >/dev/null 2>&1; then
    compat_section="$(awk '
      /^### Compatibility$/ { in_compat=1; next }
      /^### / && in_compat { exit }
      in_compat { print }
    ' "${out_md}")"
    if echo "${compat_section}" | grep -q -- "^- Non-breaking change\\.$" \
      && ! echo "${compat_section}" | grep -q -- "\\[x\\]\\|\\[ \\]"; then
      echo "PASS [compatibility-single-status-line]"
    else
      echo "FAIL [compatibility-single-status-line] compatibility section is not normalized"
      echo "Compatibility section:"
      echo "${compat_section}"
      TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
  else
    echo "FAIL [compatibility-single-status-line] script execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi
  rm -rf "${tmp}"

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  build_mock_bin "${tmp}/bin"
  out_md="${tmp}/range.md"
  args_log="${tmp}/gh_args.log"
  if (
    cd "${ROOT_DIR}"
    MOCK_GH_ARGS_LOG="${args_log}" \
    PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" --dry-run --debug --base dev --head test-head "${out_md}"
  ) >/dev/null 2>&1; then
    if grep -q -- "api repos/owner/repo/compare/dev...test-head" "${args_log}"; then
      echo "PASS [dry-run-uses-github-compare-api]"
    else
      echo "FAIL [dry-run-uses-github-compare-api] expected compare API call"
      sed -n '1,80p' "${args_log}"
      TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
  else
    echo "FAIL [dry-run-uses-github-compare-api] script execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi
  rm -rf "${tmp}"

  TESTS_RUN=$((TESTS_RUN + 1))
  category_out="$(bash -c '
    set -euo pipefail
    source "'"${ROOT_DIR}"'/scripts/versioning/file_versioning/github/lib/classification.sh"
    issue_category_from_labels "documentation||community-health-files"
  ' 2>/dev/null || true)"
  if [[ "${category_out}" == "Docs" ]]; then
    echo "PASS [label-classification-does-not-false-positive-security]"
  else
    echo "FAIL [label-classification-does-not-false-positive-security] expected Docs, got: ${category_out}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi

  TESTS_RUN=$((TESTS_RUN + 1))
  category_out="$(bash -c '
    set -euo pipefail
    source "'"${ROOT_DIR}"'/scripts/versioning/file_versioning/github/lib/classification.sh"
    issue_category_from_labels "documentation||security"
  ' 2>/dev/null || true)"
  if [[ "${category_out}" == "Security" ]]; then
    echo "PASS [label-classification-keeps-explicit-security]"
  else
    echo "FAIL [label-classification-keeps-explicit-security] expected Security, got: ${category_out}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  build_mock_bin "${tmp}/bin"
  out_md="${tmp}/merge_fix.md"
  if (
    cd "${ROOT_DIR}"
    MOCK_GIT_LOG_ONELINE="beef001 Merge pull request #512 from organization-ai-projects/fix/scripts-classification" \
    PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" --dry-run --base dev --head test-head "${out_md}"
  ) >/dev/null 2>&1; then
    bug_section="$(awk '
      /^#### Bug Fixes$/ { in_bug=1; next }
      /^#### / && in_bug { exit }
      in_bug { print }
    ' "${out_md}")"
    features_section="$(awk '
      /^#### Features$/ { in_feat=1; next }
      /^#### / && in_feat { exit }
      in_feat { print }
    ' "${out_md}")"

    if echo "${bug_section}" | grep -q "Merge pull request from organization-ai-projects/fix/scripts-classification"; then
      if ! echo "${features_section}" | grep -q "Merge pull request"; then
        echo "PASS [merge-fix-classified-as-bug-fixes]"
      else
        echo "FAIL [merge-fix-classified-as-bug-fixes] merge line leaked into Features"
        TESTS_FAILED=$((TESTS_FAILED + 1))
      fi
    else
      echo "FAIL [merge-fix-classified-as-bug-fixes] expected merge line not found in Bug Fixes"
      TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
  else
    echo "FAIL [merge-fix-classified-as-bug-fixes] script execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi
  rm -rf "${tmp}"

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  build_mock_bin "${tmp}/bin"
  out_md="${tmp}/merge_misc.md"
  if (
    cd "${ROOT_DIR}"
    MOCK_GIT_LOG_ONELINE="beef777 Merge pull request #777 from organization-ai-projects/remi-bezot/sub-pr-378" \
    PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" --dry-run --base dev --head test-head "${out_md}"
  ) >/dev/null 2>&1; then
    refactor_section="$(awk '
      /^#### Refactoring$/ { in_ref=1; next }
      /^#### / && in_ref { exit }
      in_ref { print }
    ' "${out_md}")"
    features_section="$(awk '
      /^#### Features$/ { in_feat=1; next }
      /^#### / && in_feat { exit }
      in_feat { print }
    ' "${out_md}")"

    if echo "${refactor_section}" | grep -q "Merge pull request from organization-ai-projects/remi-bezot/sub-pr-378"; then
      if ! echo "${features_section}" | grep -q "Merge pull request from organization-ai-projects/remi-bezot/sub-pr-378"; then
        echo "PASS [merge-default-avoids-features-overclassification]"
      else
        echo "FAIL [merge-default-avoids-features-overclassification] merge line leaked into Features"
        TESTS_FAILED=$((TESTS_FAILED + 1))
      fi
    else
      echo "FAIL [merge-default-avoids-features-overclassification] expected merge line not found in Refactoring"
      TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
  else
    echo "FAIL [merge-default-avoids-features-overclassification] script execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi
  rm -rf "${tmp}"

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  build_mock_bin "${tmp}/bin"
  out_md="${tmp}/merge.md"
  if (
    cd "${ROOT_DIR}"
    MOCK_GIT_LOG_ONELINE="abcd123 Merge pull request #402 from organization-ai-projects/sync/main-into-dev" \
    PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" --dry-run --base dev --head test-head "${out_md}"
  ) >/dev/null 2>&1; then
    merge_line="$(grep -F -- "Merge pull request" "${out_md}" | head -n 1 || true)"
    if [[ -n "${merge_line}" ]] && [[ "$(grep -o '#402' <<< "${merge_line}" | wc -l | tr -d ' ')" == "1" ]]; then
      echo "PASS [no-duplicated-pr-ref-in-key-changes]"
    else
      echo "FAIL [no-duplicated-pr-ref-in-key-changes] duplicated or missing PR ref"
      echo "Line: ${merge_line}"
      TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
  else
    echo "FAIL [no-duplicated-pr-ref-in-key-changes] script execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi
  rm -rf "${tmp}"

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  build_mock_bin "${tmp}/bin"
  out_md="${tmp}/snapshot_non_breaking.md"
  if (
    cd "${ROOT_DIR}"
    PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" --dry-run "${out_md}"
  ) >/dev/null 2>&1; then
    if diff -u "${SNAPSHOT_DIR}/dry_run_non_breaking.md" "${out_md}" >/dev/null; then
      echo "PASS [golden-snapshot-dry-run-non-breaking]"
    else
      echo "FAIL [golden-snapshot-dry-run-non-breaking] snapshot mismatch"
      diff -u "${SNAPSHOT_DIR}/dry_run_non_breaking.md" "${out_md}" || true
      TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
  else
    echo "FAIL [golden-snapshot-dry-run-non-breaking] script execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi
  rm -rf "${tmp}"

  TESTS_RUN=$((TESTS_RUN + 1))
  parse_out="$(echo "Commit b5fffa6 closed #520, fixed #521, but the correct footer should be closes #518" | awk '
    BEGIN { IGNORECASE = 1 }
    {
      line = $0
      while (match(line, /(^|[^[:alnum:]_])(close|closes)[[:space:]]+([[:alnum:]_.-]+\/)?#[0-9]+/)) {
        matched = substr(line, RSTART, RLENGTH)
        sub(/^[^[:alnum:]_]/, "", matched)
        split(matched, parts, /[[:space:]]+/)
        token = tolower(parts[1])
        issue_ref = parts[2]
        sub(/^[[:alnum:]_.-]+\//, "", issue_ref)
        if (token ~ /^clos/) {
          action = "Closes"
        } else {
          action = ""
        }
        if (issue_ref ~ /^#[0-9]+$/ && action != "") {
          print action "|" issue_ref
        }
        line = substr(line, RSTART + RLENGTH)
      }
    }
  ' | sort -u)"

  if [[ "${parse_out}" == "Closes|#518" ]]; then
    echo "PASS [closure-keyword-strictness-ignores-closed]"
  else
    echo "FAIL [closure-keyword-strictness-ignores-closed] expected Closes|#518, got: ${parse_out}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi

  TESTS_RUN=$((TESTS_RUN + 1))
  parse_out="$(bash -c '
    set -euo pipefail
    source "'"${ROOT_DIR}"'/scripts/versioning/file_versioning/github/lib/issue_refs.sh"
    parse_pr_body_closing_issue_refs_from_text "Fixes owner/repo#519"
  ' 2>/dev/null || true)"
  if [[ "${parse_out}" == "Closes|#519" ]]; then
    echo "PASS [pr-body-keyword-supports-fixes]"
  else
    echo "FAIL [pr-body-keyword-supports-fixes] expected Closes|#519, got: ${parse_out}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  build_mock_bin "${tmp}/bin"
  out_md="${tmp}/reopen_conflict.md"
  if (
    cd "${ROOT_DIR}"
    MOCK_GIT_LOG_BODY=$'Closes #518\nReopen #518' \
    PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" --dry-run "${out_md}"
  ) >/dev/null 2>&1; then
    if grep -q "Closes rejected #518" "${out_md}" \
      && ! grep -q "Closure Neutralization Notices" "${out_md}" \
      && ! grep -q "conflicting closure directives" "${out_md}"; then
      echo "PASS [reopen-conflict-neutralizes-closing-ref]"
    else
      echo "FAIL [reopen-conflict-neutralizes-closing-ref] expected rejected close without verbose neutralization section"
      sed -n '1,120p' "${out_md}"
      TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
  else
    echo "FAIL [reopen-conflict-neutralizes-closing-ref] script execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi
  rm -rf "${tmp}"

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(shell_test_mktemp_dir "pr_desc_tests")"
  build_mock_bin "${tmp}/bin"
  out_md="${tmp}/snapshot_breaking.md"
  if (
    cd "${ROOT_DIR}"
    MOCK_GIT_LOG_BODY="BREAKING CHANGE: api changed" \
    PATH="${tmp}/bin:${PATH}" /bin/bash "${TARGET_SCRIPT}" --dry-run "${out_md}"
  ) >/dev/null 2>&1; then
    if diff -u "${SNAPSHOT_DIR}/dry_run_breaking.md" "${out_md}" >/dev/null; then
      echo "PASS [golden-snapshot-dry-run-breaking]"
    else
      echo "FAIL [golden-snapshot-dry-run-breaking] snapshot mismatch"
      diff -u "${SNAPSHOT_DIR}/dry_run_breaking.md" "${out_md}" || true
      TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
  else
    echo "FAIL [golden-snapshot-dry-run-breaking] script execution failed"
    TESTS_FAILED=$((TESTS_FAILED + 1))
  fi
  rm -rf "${tmp}"

  echo ""
  echo "Tests run: ${TESTS_RUN}"
  echo "Tests failed: ${TESTS_FAILED}"

  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
