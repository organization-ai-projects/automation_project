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
  run_case "auto-edit-dry-run-in-memory" 0 "mode --auto-edit" mock --dry-run --auto-edit 400 --base dev --head test-head --yes
  run_case "missing-gh-dependency" 3 "la commande 'gh' est introuvable" no_gh --dry-run --base dev --head test-head

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

  echo ""
  echo "Tests run: ${TESTS_RUN}"
  echo "Tests failed: ${TESTS_FAILED}"

  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
