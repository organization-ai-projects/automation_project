#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../.." && pwd)"
HOOKS_DIR="${ROOT_DIR}/scripts/automation/git_hooks"
FIXTURES_DIR="${SCRIPT_DIR}/fixtures"

TESTS_RUN=0
TESTS_FAILED=0

mktemp_compat() {
  mktemp -d 2>/dev/null || mktemp -d -t hook_guardrails_tests
}

build_mock_bin() {
  local mock_dir="$1"
  mkdir -p "${mock_dir}"

  cat > "${mock_dir}/gh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

contains_issue() {
  local issue="$1"
  local list="${MOCK_ROOT_PARENT_ISSUES:-617}"
  [[ " ${list} " == *" ${issue} "* ]]
}

if [[ "${1:-}" == "repo" && "${2:-}" == "view" ]]; then
  echo "owner/repo"
  exit 0
fi

if [[ "${1:-}" == "issue" && "${2:-}" == "list" ]]; then
  search=""
  while [[ $# -gt 0 ]]; do
    if [[ "${1:-}" == "--search" ]]; then
      search="${2:-}"
      break
    fi
    shift
  done

  issue_number=""
  if [[ "$search" =~ \#([0-9]+) ]]; then
    issue_number="${BASH_REMATCH[1]}"
  fi

  if [[ -n "$issue_number" ]] && contains_issue "$issue_number"; then
    echo "1"
  else
    echo "0"
  fi
  exit 0
fi

if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  issue_number="${3:-0}"
  if contains_issue "$issue_number"; then
    echo "Parent: none"
  else
    echo "Parent: #617"
  fi
  exit 0
fi

exit 0
EOF
  chmod +x "${mock_dir}/gh"
}

setup_repo() {
  local tmp_dir="$1"
  local repo_dir="${tmp_dir}/repo"
  local remote_dir="${tmp_dir}/remote.git"

  git init --bare "${remote_dir}" >/dev/null 2>&1
  git init "${repo_dir}" >/dev/null 2>&1

  (
    cd "${repo_dir}"
    git config user.name "Hook Tests"
    git config user.email "hook-tests@example.com"
    ln -s "${ROOT_DIR}/scripts" scripts

    git checkout -b dev >/dev/null 2>&1
    mkdir -p documentation
    echo "base" > documentation/base.md
    git add documentation/base.md
    git commit -m "chore: base" >/dev/null 2>&1
    git remote add origin "${remote_dir}"
    git push -u origin dev >/dev/null 2>&1
    git checkout -b topic >/dev/null 2>&1
  )
}

run_case() {
  local name="$1"
  local expected_exit="$2"
  local expected_pattern="$3"
  local command="$4"

  TESTS_RUN=$((TESTS_RUN + 1))

  local tmp_dir out_file err_file merged_file status
  tmp_dir="$(mktemp_compat)"
  out_file="${tmp_dir}/out.txt"
  err_file="${tmp_dir}/err.txt"
  merged_file="${tmp_dir}/merged.txt"
  status=0

  setup_repo "${tmp_dir}"
  build_mock_bin "${tmp_dir}/bin"

  (
    cd "${tmp_dir}/repo"
    PATH="${tmp_dir}/bin:${PATH}" \
    GH_REPO="owner/repo" \
    MOCK_ROOT_PARENT_ISSUES="617" \
    /bin/bash -c "${command}"
  ) >"${out_file}" 2>"${err_file}" || status=$?

  cat "${out_file}" "${err_file}" > "${merged_file}"

  if [[ "${status}" -ne "${expected_exit}" ]]; then
    echo "FAIL [${name}] expected exit ${expected_exit}, got ${status}"
    sed -n '1,120p' "${merged_file}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp_dir}"
    return
  fi

  if [[ -n "${expected_pattern}" ]] && ! grep -qE -- "${expected_pattern}" "${merged_file}"; then
    echo "FAIL [${name}] missing pattern: ${expected_pattern}"
    sed -n '1,120p' "${merged_file}"
    TESTS_FAILED=$((TESTS_FAILED + 1))
    rm -rf "${tmp_dir}"
    return
  fi

  echo "PASS [${name}]"
  rm -rf "${tmp_dir}"
}

main() {
  echo "Running convention guardrails regression tests"

  # commit-msg: allow child issue refs in footer.
  run_case \
    "commit-msg-allows-child-footer" \
    0 \
    "" \
    "cp '${FIXTURES_DIR}/commit_msg_valid_child.txt' .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

  # commit-msg: allow non-root issue refs in body and normalize to footer block.
  run_case \
    "commit-msg-allows-and-normalizes-body-ref" \
    0 \
    "Part of #123" \
    "printf 'docs: update hook policy wording\n\nContext line\nPart of #123\nMore notes\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG && tail -n1 .git/COMMIT_EDITMSG"

  run_case \
    "commit-msg-normalizes-lowercase-and-deduplicates" \
    0 \
    "^2$" \
    "printf 'docs: update hook policy wording\n\npart of #123\nPart of #123\nRELATED TO #456\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG && grep -Ec '^(Part of #123|Related to #456)$' .git/COMMIT_EDITMSG"

  # commit-msg: block issue refs in subject.
  run_case \
    "commit-msg-blocks-subject-issue-ref" \
    1 \
    "Issue references must be in commit footer" \
    "cp '${FIXTURES_DIR}/commit_msg_invalid_subject_ref.txt' .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

  # commit-msg: block root parent refs in footer.
  run_case \
    "commit-msg-blocks-root-parent" \
    1 \
    "Root parent issue references are not allowed" \
    "cp '${FIXTURES_DIR}/commit_msg_invalid_root_parent.txt' .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

  run_case \
    "commit-msg-bypass-works" \
    0 \
    "" \
    "cp '${FIXTURES_DIR}/commit_msg_invalid_subject_ref.txt' .git/COMMIT_EDITMSG && SKIP_COMMIT_VALIDATION=1 /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

  run_case \
    "commit-msg-requires-scope-for-library-change" \
    1 \
    "Missing required scope in commit message" \
    "mkdir -p projects/libraries/security/src && echo 'pub fn x() {}' > projects/libraries/security/src/lib.rs && git add projects/libraries/security/src/lib.rs && printf 'fix: patch\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

  run_case \
    "commit-msg-rejects-wrong-scope-for-library-change" \
    1 \
    "Commit scope does not match touched files" \
    "mkdir -p projects/libraries/security/src && echo 'pub fn x() {}' > projects/libraries/security/src/lib.rs && git add projects/libraries/security/src/lib.rs && printf 'fix(projects/libraries/other): patch\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

  run_case \
    "commit-msg-allows-correct-scope-for-library-change" \
    0 \
    "" \
    "mkdir -p projects/libraries/security/src && echo 'pub fn x() {}' > projects/libraries/security/src/lib.rs && git add projects/libraries/security/src/lib.rs && printf 'fix(projects/libraries/security): patch\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

  run_case \
    "commit-msg-allows-parent-product-scope-for-ui-and-backend" \
    0 \
    "" \
    "mkdir -p projects/products/stable/varina/ui/src projects/products/stable/varina/backend/src && echo 'pub fn ui() {}' > projects/products/stable/varina/ui/src/lib.rs && echo 'pub fn api() {}' > projects/products/stable/varina/backend/src/lib.rs && git add projects/products/stable/varina/ui/src/lib.rs projects/products/stable/varina/backend/src/lib.rs && printf 'fix(projects/products/stable/varina): patch\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

  run_case \
    "commit-msg-requires-scope-for-staged-deletions" \
    1 \
    "Missing required scope in commit message" \
    "mkdir -p projects/libraries/security/src && echo 'pub fn x() {}' > projects/libraries/security/src/lib.rs && git add projects/libraries/security/src/lib.rs && git commit -m 'chore: add temp lib file' >/dev/null && git rm -q projects/libraries/security/src/lib.rs && printf 'fix: remove old file\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

  run_case \
    "commit-msg-allows-scope-for-staged-deletions" \
    0 \
    "" \
    "mkdir -p projects/libraries/security/src && echo 'pub fn x() {}' > projects/libraries/security/src/lib.rs && git add projects/libraries/security/src/lib.rs && git commit -m 'chore: add temp lib file' >/dev/null && git rm -q projects/libraries/security/src/lib.rs && printf 'fix(projects/libraries/security): remove old file\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

  run_case \
    "pre-commit-docs-only-ignores-unstaged-rust-syntax-errors" \
    0 \
    "Pre-commit checks passed" \
    "mkdir -p src documentation && printf '[package]\nname = \"tmp\"\nversion = \"0.1.0\"\nedition = \"2021\"\n' > Cargo.toml && printf 'fn main() { println!(\"ok\"); }\n' > src/main.rs && git add Cargo.toml src/main.rs && git commit -m 'chore: add minimal rust crate' >/dev/null && printf 'fn main( {\n' > src/main.rs && echo 'note' > documentation/precommit.md && git add documentation/precommit.md && /bin/bash '${HOOKS_DIR}/pre-commit'"

  run_case \
    "pre-commit-ignores-unstaged-orchestrator-permission-mismatches" \
    0 \
    "Pre-commit checks passed" \
    "rm scripts && mkdir -p scripts/common_lib/automation scripts/versioning/file_versioning/orchestrators/read scripts/versioning/file_versioning/orchestrators/execute documentation && cp '${ROOT_DIR}/scripts/common_lib/automation/scope_resolver.sh' scripts/common_lib/automation/scope_resolver.sh && printf '#!/usr/bin/env bash\necho read\n' > scripts/versioning/file_versioning/orchestrators/read/check.sh && chmod 644 scripts/versioning/file_versioning/orchestrators/read/check.sh && git add scripts/common_lib/automation/scope_resolver.sh scripts/versioning/file_versioning/orchestrators/read/check.sh && git commit -m 'chore: add local scripts tree' >/dev/null && chmod +x scripts/versioning/file_versioning/orchestrators/read/check.sh && echo 'note' > documentation/precommit_perm.md && git add documentation/precommit_perm.md && /bin/bash '${HOOKS_DIR}/pre-commit'"

  run_case \
    "pre-commit-blocks-staged-orchestrator-permission-mismatches" \
    1 \
    "Script permission errors detected" \
    "rm scripts && mkdir -p scripts/common_lib/automation scripts/versioning/file_versioning/orchestrators/read scripts/versioning/file_versioning/orchestrators/execute && cp '${ROOT_DIR}/scripts/common_lib/automation/scope_resolver.sh' scripts/common_lib/automation/scope_resolver.sh && printf '#!/usr/bin/env bash\necho read\n' > scripts/versioning/file_versioning/orchestrators/read/check.sh && chmod 644 scripts/versioning/file_versioning/orchestrators/read/check.sh && git add scripts/common_lib/automation/scope_resolver.sh scripts/versioning/file_versioning/orchestrators/read/check.sh && git commit -m 'chore: add local scripts tree' >/dev/null && chmod +x scripts/versioning/file_versioning/orchestrators/read/check.sh && git add scripts/versioning/file_versioning/orchestrators/read/check.sh && /bin/bash '${HOOKS_DIR}/pre-commit'"

  # pre-push: block tracking-only push unless explicit override.
  run_case \
    "pre-push-blocks-part-of-only" \
    1 \
    "Push blocked: commit range contains tracking refs" \
    "echo 'note' >> documentation/work.md && git add documentation/work.md && git commit -m 'docs: update workflow note' -m 'Part of #123' >/dev/null && /bin/bash '${HOOKS_DIR}/pre-push'"

  run_case \
    "pre-push-allows-part-of-only-with-override" \
    0 \
    "Pre-push checks PASSED" \
    "echo 'note' >> documentation/work.md && git add documentation/work.md && git commit -m 'docs: update workflow note' -m 'Part of #123' >/dev/null && ALLOW_PART_OF_ONLY_PUSH=1 /bin/bash '${HOOKS_DIR}/pre-push'"

  # pre-push: block root parent refs in pushed commit range.
  run_case \
    "pre-push-blocks-root-parent" \
    1 \
    "Root parent issue references detected" \
    "echo 'note' >> documentation/work.md && git add documentation/work.md && git commit -m 'docs: update workflow note' -m 'Part of #617' >/dev/null && /bin/bash '${HOOKS_DIR}/pre-push'"

  # post-checkout: warn when branch history references root parent.
  run_case \
    "post-checkout-warns-on-root-parent" \
    0 \
    "Convention warning on branch checkout" \
    "echo 'note' >> documentation/work.md && git add documentation/work.md && git commit -m 'docs: update workflow note' -m 'Part of #617' >/dev/null && /bin/bash '${HOOKS_DIR}/post-checkout' HEAD~1 HEAD 1"

  echo ""
  echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
  if [[ "${TESTS_FAILED}" -ne 0 ]]; then
    exit 1
  fi
}

main "$@"
