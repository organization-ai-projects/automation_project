#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../../.." && pwd)"
TARGET_SCRIPT="${ROOT_DIR}/scripts/versioning/file_versioning/github/auto_link_parent_issue.sh"

# shellcheck source=scripts/common_lib/testing/shell_test_helpers.sh
source "${ROOT_DIR}/scripts/common_lib/testing/shell_test_helpers.sh"

tmp_dir="$(shell_test_mktemp_dir "auto_link_parent_issue_tests")"
trap 'rm -rf "${tmp_dir}"' EXIT

mock_dir="${tmp_dir}/bin"
mkdir -p "${mock_dir}"
args_log="${tmp_dir}/gh_args.log"
out_log="${tmp_dir}/out.log"
err_log="${tmp_dir}/err.log"

cat > "${mock_dir}/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf "%s\n" "$*" >> "${MOCK_GH_ARGS_LOG}"

if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  issue_number="${3:-}"
  if [[ "${issue_number}" == "123" ]]; then
    cat <<'JSON'
{"number":123,"title":"fix(automation): child issue","state":"OPEN","url":"https://example.test/123","body":"## Context\n\nchild context\n\n## Problem\n\nchild problem\n\n## Acceptance Criteria\n\nDone when :\n\n- [ ] child criterion\n\n## Hierarchy\n\nParent: #686\n"}
JSON
    exit 0
  fi
  if [[ "${issue_number}" == "686" ]]; then
    cat <<'JSON'
{"number":686,"title":"refactor(architecture): parent issue","state":"OPEN","url":"https://example.test/686"}
JSON
    exit 0
  fi
fi

if [[ "${1:-}" == "api" && "${2:-}" == "graphql" ]]; then
  joined="$*"
  if [[ "${joined}" == *"child:issue(number:$child)"* ]]; then
    cat <<'JSON'
{"data":{"repository":{"child":{"id":"CHILD_NODE","parent":null},"parent":{"id":"PARENT_NODE","state":"OPEN"}}}}
JSON
    exit 0
  fi
  if [[ "${joined}" == *"addSubIssue("* ]]; then
    cat <<'JSON'
{"errors":[{"message":"Resource not accessible by integration"}],"data":{"addSubIssue":null}}
JSON
    exit 0
  fi
fi

if [[ "${1:-}" == "api" && "${2:-}" == "repos/org/repo/issues/123/comments" ]]; then
  if [[ "${3:-}" == "--paginate" ]]; then
    echo "[]"
    exit 0
  fi
  echo '{}'
  exit 0
fi

if [[ "${1:-}" == "api" && "${2:-}" == "repos/org/repo/issues/123/labels" ]]; then
  echo '{}'
  exit 0
fi

if [[ "${1:-}" == "api" && "${2:-}" == "-X" && "${3:-}" == "DELETE" ]]; then
  echo '{}'
  exit 0
fi

echo '{}'
EOF
chmod +x "${mock_dir}/gh"

(
  cd "${ROOT_DIR}"
  MOCK_GH_ARGS_LOG="${args_log}" \
  GH_REPO="org/repo" \
  PATH="${mock_dir}:${PATH}" \
  bash "${TARGET_SCRIPT}" --issue 123
) >"${out_log}" 2>"${err_log}" || true

if grep -q "Linked issue #123 to parent #686" "${out_log}"; then
  echo "FAIL: mutation GraphQL errors must not be treated as success."
  cat "${out_log}" "${err_log}"
  exit 1
fi

if ! grep -q "labels\\[\\]=automation-failed" "${args_log}"; then
  echo "FAIL: automation-failed label was not set on GraphQL mutation error."
  cat "${args_log}"
  exit 1
fi

echo "PASS: GraphQL mutation errors are handled as runtime failures."
