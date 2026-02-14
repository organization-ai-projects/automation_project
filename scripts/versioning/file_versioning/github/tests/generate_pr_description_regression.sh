#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/github/generate_pr_description.sh"

TESTS_RUN=0
TESTS_FAILED=0

mktemp_compat() {
  mktemp -d 2>/dev/null || mktemp -d -t pr_desc_tests
}

build_mock_bin() {
  local mock_dir="$1"
  mkdir -p "${mock_dir}"

  cat > "${mock_dir}/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "repo" && "${2:-}" == "view" ]]; then
  echo "owner/repo"
  exit 0
fi
if [[ "${1:-}" == "pr" && "${2:-}" == "create" ]]; then
  echo "https://github.com/owner/repo/pull/999"
  exit 0
fi
exit 0
EOF
  chmod +x "${mock_dir}/gh"

  cat > "${mock_dir}/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
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

  cat > "${mock_dir}/jq" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cat
EOF
  chmod +x "${mock_dir}/jq"
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

  cat > "${mock_dir}/jq" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
cat
EOF
  chmod +x "${mock_dir}/jq"
}

run_case() {
  local name="$1"
  local expected_exit="$2"
  local expected_pattern="$3"
  local mode="${4:-mock}"
  shift 4

  TESTS_RUN=$((TESTS_RUN + 1))

  local tmp
  tmp="$(mktemp_compat)"
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
  run_case "base-missing-value" 2 "--base requiert une valeur" mock --base
  run_case "head-missing-value" 2 "--head requiert une valeur" mock --head
  run_case "duplicate-mode-invalid" 2 "--duplicate-mode doit être 'safe' ou 'auto-close'" mock --duplicate-mode invalid --dry-run
  run_case "auto-edit-missing-value" 2 "--auto-edit requiert une valeur" mock --auto-edit
  run_case "auto-edit-non-numeric" 2 "--auto-edit requiert un PR_NUMBER numérique" mock --auto-edit abc --dry-run
  run_case "create-pr-without-dry-run" 2 "--create-pr est uniquement supporté avec --dry-run" mock --create-pr 42
  run_case "allow-partial-without-create-pr" 2 "--allow-partial-create nécessite --create-pr" mock --allow-partial-create 42
  run_case "auto-edit-conflicts-with-create-pr" 2 "--auto-edit ne peut pas être combiné avec --create-pr/--auto" mock --dry-run --create-pr --auto-edit 400
  run_case "auto-forbids-positional" 2 "--auto ne prend pas d'OUTPUT_FILE positional" mock --auto output.md
  run_case "auto-edit-dry-run-forbids-positional" 2 "En mode --auto-edit \\(dry-run\\), OUTPUT_FILE positional n'est pas autorisé" mock --dry-run --auto-edit 400 output.md
  run_case "auto-edit-main-forbids-output-file" 2 "En mode --auto-edit \\(MAIN_PR_NUMBER\\), OUTPUT_FILE positional n'est pas autorisé" mock --auto-edit 400 42 out.md
  run_case "dry-run-too-many-positionals" 2 "Trop d'arguments positionnels pour --dry-run" mock --dry-run out.md extra.md
  run_case "dry-run-minimal" 0 "Fichier généré:" mock --dry-run --base dev --head test-head /tmp/pr_description_test.md
  run_case "auto-create-success-does-not-return-no-data" 0 "PR créée:" mock --auto --base dev --head test-head --yes
  run_case "auto-edit-dry-run-in-memory" 0 "mode --auto-edit" mock --dry-run --auto-edit 400 --base dev --head test-head --yes
  run_case "dry-run-without-gh" 0 "Fichier généré:" no_gh --dry-run --base dev --head test-head /tmp/pr_description_test_no_gh.md
  run_case "missing-gh-required-main-mode" 3 "la commande 'gh' est introuvable" no_gh 42
  run_case "duplicate-mode-dry-run-output" 0 "Duplicate mode \\(safe\\): no duplicate declarations detected" mock --dry-run --base dev --head test-head --duplicate-mode safe
  run_case "debug-flag-emits-trace" 0 "\\[debug\\] extract_child_prs_dry" mock --dry-run --base dev --head test-head --debug

  TESTS_RUN=$((TESTS_RUN + 1))
  tmp="$(mktemp_compat)"
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
  tmp="$(mktemp_compat)"
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
  tmp="$(mktemp_compat)"
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
  tmp="$(mktemp_compat)"
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

  echo ""
  echo "Tests run: ${TESTS_RUN}"
  echo "Tests failed: ${TESTS_FAILED}"

  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
