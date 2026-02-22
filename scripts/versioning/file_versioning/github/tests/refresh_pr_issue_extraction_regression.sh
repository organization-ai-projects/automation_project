#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/github/generate_pr_description.sh"

# shellcheck source=scripts/common_lib/testing/shell_test_helpers.sh
source "${ROOT_DIR}/scripts/common_lib/testing/shell_test_helpers.sh"

build_mock_bin() {
  local mock_dir="$1"
  mkdir -p "${mock_dir}"

  cat > "${mock_dir}/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "repo" && "${2:-}" == "view" ]]; then
  if [[ "${3:-}" == "--json" && "${4:-}" == "nameWithOwner" && "${5:-}" == "-q" ]]; then
    echo "owner/repo"
  else
    echo "owner/repo"
  fi
  exit 0
fi

if [[ "${1:-}" == "pr" && "${2:-}" == "view" ]]; then
  if [[ "${4:-}" == "--json" && "${6:-}" == "-q" ]]; then
    case "${7:-}" in
      .baseRefName) echo "dev" ;;
      .headRefName) echo "test-head" ;;
      .body) echo "" ;;
      .comments[].body) echo "" ;;
      *) echo "" ;;
    esac
    exit 0
  fi

  # Generic JSON payload for non -q access.
  echo '{"baseRefName":"dev","headRefName":"test-head","body":"","title":"mock pr","labels":[]}'
  exit 0
fi

if [[ "${1:-}" == "api" && "${2:-}" == "repos/owner/repo/compare/dev...test-head" ]]; then
  if [[ "${3:-}" == "--jq" ]]; then
    printf "fix(test): synthetic change\n\nCloses #648\n"
  else
    echo '{"commits":[{"commit":{"message":"fix(test): synthetic change\n\nCloses #648"}}]}'
  fi
  exit 0
fi

if [[ "${1:-}" == "api" && "${2:-}" =~ ^repos/owner/repo/pulls/[0-9]+$ ]]; then
  # `is_pull_request_ref` probes this endpoint. Return 404-like failure so issue refs are kept.
  exit 1
fi

if [[ "${1:-}" == "api" && "${2:-}" == "-X" && "${3:-}" == "PATCH" && "${4:-}" == "repos/owner/repo/pulls/400" ]]; then
  shift 4
  while [[ $# -gt 0 ]]; do
    if [[ "${1:-}" == "--raw-field" ]]; then
      value="${2:-}"
      if [[ "$value" =~ ^body= ]]; then
        printf "%s\n" "${value#body=}" > "${MOCK_PATCH_BODY_FILE}"
        exit 0
      fi
    fi
    shift
  done
  exit 0
fi

exit 0
EOF
  chmod +x "${mock_dir}/gh"

  cat > "${mock_dir}/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "show-ref" ]]; then
  exit 0
fi
if [[ "${1:-}" == "rev-parse" && "${2:-}" == "--abbrev-ref" && "${3:-}" == "HEAD" ]]; then
  echo "test-head"
  exit 0
fi
if [[ "${1:-}" == "log" ]]; then
  printf "fix(test): synthetic change\n\nCloses #648\n"
  exit 0
fi

exit 0
EOF
  chmod +x "${mock_dir}/git"

  shell_test_write_passthrough_jq_mock "${mock_dir}"
}

main() {
  echo "Running refresh-pr issue extraction regression"

  local tmp
  tmp="$(shell_test_mktemp_dir "refresh_pr_issue_extract")"
  local out_file="${tmp}/out.txt"
  local err_file="${tmp}/err.txt"
  local patch_body_file="${tmp}/patched_body.md"

  build_mock_bin "${tmp}/bin"

  if ! (
    cd "${ROOT_DIR}"
    MOCK_PATCH_BODY_FILE="${patch_body_file}" \
    PATH="${tmp}/bin:${PATH}" \
      /bin/bash "${TARGET_SCRIPT}" --refresh-pr 400 400 --yes
  ) >"${out_file}" 2>"${err_file}"; then
    echo "FAIL [refresh-pr-runs]"
    cat "${err_file}" || true
    rm -rf "${tmp}"
    exit 1
  fi

  if ! grep -q "Closes #648" "${patch_body_file}"; then
    echo "FAIL [refresh-pr-includes-closure-refs]"
    sed -n '1,140p' "${patch_body_file}" || true
    rm -rf "${tmp}"
    exit 1
  fi

  echo "PASS [refresh-pr-includes-closure-refs]"
  rm -rf "${tmp}"
}

main "$@"
