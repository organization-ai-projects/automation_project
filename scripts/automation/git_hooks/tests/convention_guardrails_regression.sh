#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../../../.." && pwd)"
HOOKS_DIR="${ROOT_DIR}/scripts/automation/git_hooks"
FIXTURES_DIR="${SCRIPT_DIR}/fixtures"

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

contains_issue() {
  local issue="$1"
  local list="${MOCK_ROOT_PARENT_ISSUES:-617}"
  [[ " ${list} " == *" ${issue} "* ]]
}

issue_has_children() {
  local issue="$1"
  local list="${MOCK_PARENT_WITH_CHILDREN:-${MOCK_ROOT_PARENT_ISSUES:-617}}"
  [[ " ${list} " == *" ${issue} "* ]]
}

issue_parent_mode() {
  local issue="$1"
  local epic_list="${MOCK_EPIC_PARENT_ISSUES:-${MOCK_ROOT_PARENT_ISSUES:-617}}"
  local base_list="${MOCK_BASE_PARENT_ISSUES:-}"
  local none_list="${MOCK_NONE_PARENT_ISSUES:-}"

  if [[ " ${epic_list} " == *" ${issue} "* ]]; then
    echo "epic"
    return
  fi
  if [[ " ${base_list} " == *" ${issue} "* ]]; then
    echo "base"
    return
  fi
  if [[ " ${none_list} " == *" ${issue} "* ]]; then
    echo "none"
    return
  fi
  echo "#617"
}

if [[ "${1:-}" == "repo" && "${2:-}" == "view" ]]; then
  echo "owner/repo"
  exit 0
fi

if [[ "${1:-}" == "api" && "${2:-}" == "user" ]]; then
  echo "${MOCK_GH_LOGIN:-devuser}"
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

  if [[ -n "$issue_number" ]] && issue_has_children "$issue_number"; then
    echo "1"
  else
    echo "0"
  fi
  exit 0
fi

if [[ "${1:-}" == "issue" && "${2:-}" == "view" ]]; then
  issue_number="${3:-0}"
  json_fields=""
  while [[ $# -gt 0 ]]; do
    if [[ "${1:-}" == "--json" ]]; then
      json_fields="${2:-}"
      break
    fi
    shift
  done

  if [[ "$json_fields" == *"state"* ]]; then
    closed_list="${MOCK_CLOSED_ISSUES:-}"
    if [[ " ${closed_list} " == *" ${issue_number} "* ]]; then
      echo "CLOSED"
    else
      echo "OPEN"
    fi
    exit 0
  fi

  if [[ "$json_fields" == *"assignees"* ]]; then
    if [[ " ${MOCK_MULTI_ASSIGNEE_ISSUES:-124} " == *" ${issue_number} "* ]]; then
      echo "${MOCK_GH_LOGIN:-devuser}"
      echo "pairdev"
    elif [[ " ${MOCK_UNASSIGNED_ISSUES:-} " == *" ${issue_number} "* ]]; then
      :
    else
      echo "${MOCK_GH_LOGIN:-devuser}"
    fi
    exit 0
  fi

  if [[ "$json_fields" == *"body"* ]]; then
    parent_mode="$(issue_parent_mode "$issue_number")"
    echo "Parent: ${parent_mode}"
    exit 0
  fi

  parent_mode="$(issue_parent_mode "$issue_number")"
  echo "Parent: ${parent_mode}"
  exit 0
fi

exit 0
EOF
	chmod +x "${mock_dir}/gh"

	cat >"${mock_dir}/versioning_automation" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

contains_issue() {
  local issue="$1"
  local list="${MOCK_ROOT_PARENT_ISSUES:-617}"
  [[ " ${list} " == *" ${issue} "* ]]
}

issue_has_children() {
  local issue="$1"
  local list="${MOCK_PARENT_WITH_CHILDREN:-${MOCK_ROOT_PARENT_ISSUES:-617}}"
  [[ " ${list} " == *" ${issue} "* ]]
}

issue_parent_mode() {
  local issue="$1"
  local epic_list="${MOCK_EPIC_PARENT_ISSUES:-${MOCK_ROOT_PARENT_ISSUES:-617}}"
  local base_list="${MOCK_BASE_PARENT_ISSUES:-}"
  local none_list="${MOCK_NONE_PARENT_ISSUES:-}"

  if [[ " ${epic_list} " == *" ${issue} "* ]]; then
    echo "epic"
    return
  fi
  if [[ " ${base_list} " == *" ${issue} "* ]]; then
    echo "base"
    return
  fi
  if [[ " ${none_list} " == *" ${issue} "* ]]; then
    echo "none"
    return
  fi
  echo "#617"
}

contains_list_item() {
  local value="$1"
  local list="$2"
  [[ " ${list} " == *" ${value} "* ]]
}

if [[ "${1:-}" == "automation" && "${2:-}" == "pre-push-check" ]]; then
  commits="$(git log origin/dev..HEAD --format=%B 2>/dev/null || true)"
  changed_files="$(git diff --name-only origin/dev..HEAD 2>/dev/null || true)"

  refs="$(echo "$commits" | awk '
    {
      line = $0
      lower = tolower($0)
      while (match(lower, /(closes|fixes|part[[:space:]]+of|reopen|reopens)[[:space:]]+#[0-9]+/)) {
        matched = substr(line, RSTART, RLENGTH)
        keyword = tolower(matched)
        gsub(/[[:space:]]+#[0-9]+$/, "", keyword)
        issue = matched
        sub(/^.*#/, "", issue)
        print keyword "|" issue
        line = substr(line, RSTART + RLENGTH)
        lower = substr(lower, RSTART + RLENGTH)
      }
    }
  ')"

  epic_list="${MOCK_EPIC_PARENT_ISSUES:-${MOCK_ROOT_PARENT_ISSUES:-617}}"
  while IFS='|' read -r action issue; do
    [[ -z "$issue" ]] && continue
    if contains_list_item "$issue" "$epic_list"; then
      echo "Root parent issue references detected in commits being pushed:"
      echo "   - ${action} #${issue}"
      exit 1
    fi
  done <<< "$refs"

  current_login="${MOCK_GH_LOGIN:-devuser}"
  declare -A has_part_of=()
  declare -A has_closing=()
  while IFS='|' read -r action issue; do
    [[ -z "$issue" ]] && continue
    if [[ "$action" == "part of" ]]; then
      has_part_of["$issue"]=1
    fi
    if [[ "$action" == "closes" || "$action" == "fixes" ]]; then
      has_closing["$issue"]=1
    fi
  done <<< "$refs"

  if [[ "${ALLOW_PART_OF_ONLY_PUSH:-}" != "1" ]]; then
    for issue in "${!has_part_of[@]}"; do
      [[ -n "${has_closing[$issue]:-}" ]] && continue
      multi="${MOCK_MULTI_ASSIGNEE_ISSUES:-124}"
      if ! contains_list_item "$issue" "$multi"; then
        echo "Push blocked by assignment policy."
        echo "   - #${issue} is assigned only to @${current_login}: 'Closes #${issue}' is required (Part of only is not allowed)"
        exit 1
      fi
    done
  fi

  docs_only=1
  markdown_files=0
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    case "$file" in
      documentation/*|.github/documentation/*|.github/ISSUE_TEMPLATE/*|.github/PULL_REQUEST_TEMPLATE/*|.github/workflows/*|scripts/*|*.md)
        ;;
      *)
        docs_only=0
        ;;
    esac
    if [[ "$file" == *.md ]]; then
      markdown_files=1
    fi
  done <<< "$changed_files"

  if [[ "$docs_only" -eq 1 ]]; then
    if [[ "$markdown_files" -eq 1 ]]; then
      if [[ "${MOCK_MARKDOWNLINT_FAIL:-0}" == "1" ]]; then
        echo "Markdown lint failed."
        exit 1
      fi
      echo "Markdown lint OK"
    fi
    echo "Pre-push checks PASSED (docs/scripts-only mode)"
    exit 0
  fi

  echo "All pre-push checks PASSED"
  exit 0
fi

if [[ "${1:-}" != "issue" ]]; then
  exit 0
fi

subcommand="${2:-}"
shift 2 || true

case "$subcommand" in
  repo-name)
    echo "owner/repo"
    ;;

  current-login)
    echo "${MOCK_GH_LOGIN:-devuser}"
    ;;

  subissue-refs)
    issue_number=""
    while [[ $# -gt 0 ]]; do
      case "${1:-}" in
        --issue)
          issue_number="${2:-}"
          shift 2
          ;;
        --owner|--repo)
          shift 2
          ;;
        *)
          shift
          ;;
      esac
    done

    if [[ -n "$issue_number" ]] && issue_has_children "$issue_number"; then
      echo "#${issue_number}"
    fi
    ;;

  field)
    issue_number=""
    field_name=""
    while [[ $# -gt 0 ]]; do
      case "${1:-}" in
        --issue)
          issue_number="${2:-}"
          shift 2
          ;;
        --name)
          field_name="${2:-}"
          shift 2
          ;;
        --repo)
          shift 2
          ;;
        *)
          shift
          ;;
      esac
    done

    if [[ "$field_name" == "body" ]]; then
      parent_mode="$(issue_parent_mode "$issue_number")"
      echo "Parent: ${parent_mode}"
    fi
    ;;

  assignee-logins)
    issue_number=""
    while [[ $# -gt 0 ]]; do
      case "${1:-}" in
        --issue)
          issue_number="${2:-}"
          shift 2
          ;;
        --repo)
          shift 2
          ;;
        *)
          shift
          ;;
      esac
    done

    if [[ " ${MOCK_MULTI_ASSIGNEE_ISSUES:-124} " == *" ${issue_number} "* ]]; then
      echo "${MOCK_GH_LOGIN:-devuser}"
      echo "pairdev"
    elif [[ " ${MOCK_UNASSIGNED_ISSUES:-} " == *" ${issue_number} "* ]]; then
      :
    else
      echo "${MOCK_GH_LOGIN:-devuser}"
    fi
    ;;

  extract-refs)
    profile="hook"
    raw_text=""
    file_path=""
    while [[ $# -gt 0 ]]; do
      case "${1:-}" in
        --profile)
          profile="${2:-hook}"
          shift 2
          ;;
        --text)
          raw_text="${2:-}"
          shift 2
          ;;
        --file)
          file_path="${2:-}"
          shift 2
          ;;
        *)
          shift
          ;;
      esac
    done

    if [[ -n "$file_path" && -f "$file_path" ]]; then
      raw_text="$(cat "$file_path")"
    fi

    if [[ "$profile" == "audit" ]]; then
      echo "$raw_text" | awk '
        {
          line = $0
          lower = tolower($0)
          while (match(lower, /(closes|fixes|resolves|part[[:space:]]+of|related[[:space:]]+to|reopen|reopens)[[:space:]]+#[0-9]+/)) {
            matched = substr(line, RSTART, RLENGTH)
            keyword = tolower(matched)
            gsub(/[[:space:]]+#[0-9]+$/, "", keyword)
            issue = matched
            sub(/^.*#/, "", issue)
            print keyword "|" issue
            line = substr(line, RSTART + RLENGTH)
            lower = substr(lower, RSTART + RLENGTH)
          }
        }
      ' | awk '!seen[$0]++'
    else
      echo "$raw_text" | awk '
        {
          line = $0
          lower = tolower($0)
          while (match(lower, /(closes|fixes|part[[:space:]]+of|reopen|reopens)[[:space:]]+#[0-9]+/)) {
            matched = substr(line, RSTART, RLENGTH)
            keyword = tolower(matched)
            gsub(/[[:space:]]+#[0-9]+$/, "", keyword)
            issue = matched
            sub(/^.*#/, "", issue)
            print keyword "|" issue
            line = substr(line, RSTART + RLENGTH)
            lower = substr(lower, RSTART + RLENGTH)
          }
        }
      ' | awk '!seen[$0]++'
    fi
    ;;
esac

exit 0
EOF
	chmod +x "${mock_dir}/versioning_automation"

	cat >"${mock_dir}/pnpm" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ -n "${MOCK_PNPM_ARGS_LOG:-}" ]]; then
  printf "%s\n" "$*" >> "${MOCK_PNPM_ARGS_LOG}"
fi
if [[ "${MOCK_MARKDOWNLINT_FAIL:-0}" == "1" ]]; then
  echo "mock markdownlint failure" >&2
  exit 1
fi
exit 0
EOF
	chmod +x "${mock_dir}/pnpm"

	cat >"${mock_dir}/markdownlint-cli2" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--version" ]]; then
  echo "0.21.0"
  exit 0
fi
if [[ "${MOCK_MARKDOWNLINT_FAIL:-0}" == "1" ]]; then
  echo "mock markdownlint failure" >&2
  exit 1
fi
exit 0
EOF
	chmod +x "${mock_dir}/markdownlint-cli2"
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
		mkdir -p node_modules/.bin
		echo "base" >documentation/base.md
		cat >package.json <<'EOF'
{"name":"hook-tests","private":true,"scripts":{"lint-md-files":"echo lint-md-files"},"devDependencies":{"markdownlint-cli2":"0.21.0"}}
EOF
		cat >node_modules/.bin/markdownlint-cli2 <<'EOF'
#!/usr/bin/env bash
if [[ "${1:-}" == "--version" ]]; then
  echo "0.21.0"
  exit 0
fi
if [[ "${MOCK_MARKDOWNLINT_FAIL:-0}" == "1" ]]; then
  echo "mock markdownlint failure" >&2
  exit 1
fi
exit 0
EOF
		chmod +x node_modules/.bin/markdownlint-cli2
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
	tmp_dir="$(shell_test_mktemp_dir "hook_guardrails_tests")"
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

	cat "${out_file}" "${err_file}" >"${merged_file}"

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
		"cp '${FIXTURES_DIR}/commit_msg_valid_child.txt' .git/COMMIT_EDITMSG && MOCK_MULTI_ASSIGNEE_ISSUES='123' /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-rejects-fixes-footer" \
		4 \
		"Invalid issue footer keyword: 'Fixes' is not allowed" \
		"printf 'docs: update hook policy wording\n\nFixes #123\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	# commit-msg: allow non-root issue refs in body and normalize to footer block.
	run_case \
		"commit-msg-allows-and-normalizes-body-ref" \
		0 \
		"Part of #123" \
		"printf 'docs: update hook policy wording\n\nContext line\nPart of #123\nMore notes\n' > .git/COMMIT_EDITMSG && MOCK_MULTI_ASSIGNEE_ISSUES='123' /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG && tail -n1 .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-normalizes-lowercase-and-deduplicates" \
		0 \
		"^2$" \
		"printf 'docs: update hook policy wording\n\npart of #123\nPart of #123\nREOPEN #456\n' > .git/COMMIT_EDITMSG && MOCK_MULTI_ASSIGNEE_ISSUES='123 456' /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG && grep -Ec '^(Part of #123|Reopen #456)$' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-blocks-single-assignee-part-of-only" \
		10 \
		"Closes #123' is required" \
		"printf 'docs: update hook policy wording\n\nPart of #123\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-accepts-first-nonempty-line-as-subject" \
		0 \
		"" \
		"printf '\n\nfix(shell): trim leading blanks\n\nbody\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	# commit-msg: block issue refs in subject.
	run_case \
		"commit-msg-blocks-subject-issue-ref" \
		4 \
		"Issue references must be in commit footer" \
		"cp '${FIXTURES_DIR}/commit_msg_invalid_subject_ref.txt' .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	# commit-msg: block root parent refs in footer.
	run_case \
		"commit-msg-blocks-root-parent" \
		5 \
		"Protected parent issue references are not allowed" \
		"cp '${FIXTURES_DIR}/commit_msg_invalid_root_parent.txt' .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-allows-base-parent-reference" \
		0 \
		"" \
		"printf 'docs: update hook policy wording\n\nPart of #618\n' > .git/COMMIT_EDITMSG && MOCK_BASE_PARENT_ISSUES='618' MOCK_PARENT_WITH_CHILDREN='618' MOCK_MULTI_ASSIGNEE_ISSUES='618' /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-bypass-works" \
		0 \
		"" \
		"cp '${FIXTURES_DIR}/commit_msg_invalid_subject_ref.txt' .git/COMMIT_EDITMSG && SKIP_COMMIT_VALIDATION=1 /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-requires-scope-for-library-change" \
		7 \
		"Missing required scope in commit message" \
		"mkdir -p projects/libraries/layers/domain/security/src && echo 'pub fn x() {}' > projects/libraries/layers/domain/security/src/lib.rs && git add projects/libraries/layers/domain/security/src/lib.rs && printf 'fix: patch\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-rejects-wrong-scope-for-library-change" \
		8 \
		"Commit scope does not match touched files" \
		"mkdir -p projects/libraries/layers/domain/security/src && echo 'pub fn x() {}' > projects/libraries/layers/domain/security/src/lib.rs && git add projects/libraries/layers/domain/security/src/lib.rs && printf 'fix(projects/libraries/other): patch\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-allows-correct-scope-for-library-change" \
		0 \
		"" \
		"mkdir -p projects/libraries/layers/domain/security/src && echo 'pub fn x() {}' > projects/libraries/layers/domain/security/src/lib.rs && git add projects/libraries/layers/domain/security/src/lib.rs && printf 'fix(projects/libraries/layers): patch\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-rejects-parent-product-scope-for-ui-and-backend-mix" \
		8 \
		"Commit scope does not match touched files" \
		"mkdir -p projects/products/stable/varina/ui/src projects/products/stable/varina/backend/src && printf '[package]\nname = \"varina-ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n' > projects/products/stable/varina/ui/Cargo.toml && printf '[package]\nname = \"varina-backend\"\nversion = \"0.1.0\"\nedition = \"2021\"\n' > projects/products/stable/varina/backend/Cargo.toml && echo 'pub fn ui() {}' > projects/products/stable/varina/ui/src/lib.rs && echo 'pub fn api() {}' > projects/products/stable/varina/backend/src/lib.rs && git add projects/products/stable/varina/ui/Cargo.toml projects/products/stable/varina/backend/Cargo.toml projects/products/stable/varina/ui/src/lib.rs projects/products/stable/varina/backend/src/lib.rs && printf 'fix(projects/products/stable/varina): patch\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-requires-scope-for-staged-deletions" \
		7 \
		"Missing required scope in commit message" \
		"mkdir -p projects/libraries/layers/domain/security/src && echo 'pub fn x() {}' > projects/libraries/layers/domain/security/src/lib.rs && git add projects/libraries/layers/domain/security/src/lib.rs && git commit -m 'chore: add temp lib file' >/dev/null && git rm -q projects/libraries/layers/domain/security/src/lib.rs && printf 'fix: remove old file\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-allows-scope-for-staged-deletions" \
		0 \
		"" \
		"mkdir -p projects/libraries/layers/domain/security/src && echo 'pub fn x() {}' > projects/libraries/layers/domain/security/src/lib.rs && git add projects/libraries/layers/domain/security/src/lib.rs && git commit -m 'chore: add temp lib file' >/dev/null && git rm -q projects/libraries/layers/domain/security/src/lib.rs && printf 'fix(projects/libraries/layers): remove old file\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-allows-parent-product-scope-for-parent-only-change" \
		0 \
		"" \
		"mkdir -p projects/products/stable/varina && echo '# Varina' > projects/products/stable/varina/README.md && git add projects/products/stable/varina/README.md && printf 'docs(projects/products/stable/varina): update readme\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-falls-back-to-parent-scope-when-ui-is-not-a-crate" \
		0 \
		"" \
		"mkdir -p projects/products/stable/varina/ui/src && echo 'console.log(\"ui\")' > projects/products/stable/varina/ui/src/app.ts && git add projects/products/stable/varina/ui/src/app.ts && printf 'fix(projects/products/stable/varina): patch ui files\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-rejects-ui-scope-when-ui-is-not-a-crate" \
		8 \
		"Commit scope does not match touched files" \
		"mkdir -p projects/products/stable/varina/ui/src && echo 'console.log(\"ui\")' > projects/products/stable/varina/ui/src/app.ts && git add projects/products/stable/varina/ui/src/app.ts && printf 'fix(projects/products/stable/varina/ui): patch ui files\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-detects-nonstandard-product-crate-by-cargo" \
		0 \
		"" \
		"mkdir -p projects/products/stable/varina/worker/src && printf '[package]\nname = \"varina-worker\"\nversion = \"0.1.0\"\nedition = \"2021\"\n' > projects/products/stable/varina/worker/Cargo.toml && echo 'pub fn work() {}' > projects/products/stable/varina/worker/src/lib.rs && git add projects/products/stable/varina/worker/Cargo.toml projects/products/stable/varina/worker/src/lib.rs && printf 'fix(projects/products/stable/varina/worker): patch worker crate\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-requires-shell-scope-for-shell-only-change" \
		8 \
		"Commit scope does not match touched files" \
		"printf '#!/usr/bin/env bash\necho hi\n' > helper.sh && git add helper.sh && printf 'chore(workspace): add helper script\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-allows-shell-scope-for-shell-only-change" \
		0 \
		"" \
		"printf '#!/usr/bin/env bash\necho hi\n' > helper.sh && git add helper.sh && printf 'chore(shell): add helper script\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-requires-markdown-scope-for-markdown-only-change" \
		8 \
		"Commit scope does not match touched files" \
		"echo '# title' > README.md && git add README.md && printf 'docs(workspace): update readme\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-allows-markdown-scope-for-markdown-only-change" \
		0 \
		"" \
		"echo '# title' > README.md && git add README.md && printf 'docs(markdown): update readme\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-requires-workspace-scope-for-root-level-non-rust-non-shell-non-markdown-change" \
		7 \
		"Missing required scope in commit message" \
		"echo 'x=1' > settings.toml && git add settings.toml && printf 'chore: add settings\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-rejects-wrong-scope-for-root-level-non-rust-non-shell-non-markdown-change" \
		8 \
		"Commit scope does not match touched files" \
		"echo 'x=1' > settings.toml && git add settings.toml && printf 'chore(config): add settings\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-allows-workspace-scope-for-non-rust-non-shell-non-markdown-change" \
		0 \
		"" \
		"echo 'x=1' > settings.toml && git add settings.toml && printf 'chore(workspace): add settings\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-allows-common-path-scope-for-non-rust-non-shell-non-markdown-nested-change" \
		0 \
		"" \
		"mkdir -p configs/env && echo 'x=1' > configs/env/app.toml && git add configs/env/app.toml && printf 'chore(configs/env): add app config\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-blocks-mixed-shell-and-markdown" \
		6 \
		"Mixed file format categories are not allowed" \
		"printf '#!/usr/bin/env bash\necho hi\n' > helper.sh && echo '# title' > README.md && git add helper.sh README.md && printf 'chore(shell): mixed change\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"commit-msg-blocks-mixed-rust-and-shell" \
		6 \
		"Mixed file format categories are not allowed" \
		"mkdir -p projects/libraries/layers/domain/security/src && echo 'pub fn x() {}' > projects/libraries/layers/domain/security/src/lib.rs && printf '#!/usr/bin/env bash\necho hi\n' > helper.sh && git add projects/libraries/layers/domain/security/src/lib.rs helper.sh && printf 'fix(projects/libraries/layers/domain/security): mixed change\n' > .git/COMMIT_EDITMSG && /bin/bash '${HOOKS_DIR}/commit-msg' .git/COMMIT_EDITMSG"

	run_case \
		"pre-commit-docs-only-ignores-unstaged-rust-syntax-errors" \
		0 \
		"Pre-commit checks passed" \
		"mkdir -p src documentation && printf '[package]\nname = \"tmp\"\nversion = \"0.1.0\"\nedition = \"2021\"\n' > Cargo.toml && printf 'fn main() { println!(\"ok\"); }\n' > src/main.rs && git add Cargo.toml src/main.rs && git commit -m 'chore: add minimal rust crate' >/dev/null && printf 'fn main( {\n' > src/main.rs && echo 'note' > documentation/precommit.md && git add documentation/precommit.md && /bin/bash '${HOOKS_DIR}/pre-commit'"

	run_case \
		"pre-commit-runs-markdownlint-on-staged-markdown" \
		0 \
		"Pre-commit checks passed" \
		"echo '# markdown title' > documentation/precommit_markdownlint.md && git add documentation/precommit_markdownlint.md && /bin/bash '${HOOKS_DIR}/pre-commit'"

	run_case \
		"pre-commit-blocks-markdownlint-failure" \
		1 \
		"Markdown lint failed on staged markdown files" \
		"echo '# markdown title' > documentation/precommit_markdownlint_fail.md && git add documentation/precommit_markdownlint_fail.md && MOCK_MARKDOWNLINT_FAIL=1 /bin/bash '${HOOKS_DIR}/pre-commit'"

	# pre-push: block tracking-only push unless explicit override.
	run_case \
		"pre-push-blocks-part-of-only" \
		1 \
		"Push blocked by assignment policy" \
		"echo 'note' >> documentation/work.md && git add documentation/work.md && git commit -m 'docs: update workflow note' -m 'Part of #123' >/dev/null && /bin/bash '${HOOKS_DIR}/pre-push'"

	run_case \
		"pre-push-allows-part-of-only-with-override" \
		0 \
		"Pre-push checks PASSED" \
		"echo 'note' >> documentation/work.md && git add documentation/work.md && git commit -m 'docs: update workflow note' -m 'Part of #123' >/dev/null && ALLOW_PART_OF_ONLY_PUSH=1 /bin/bash '${HOOKS_DIR}/pre-push'"

	run_case \
		"pre-push-allows-part-of-only-when-multi-assignee" \
		0 \
		"Pre-push checks PASSED" \
		"echo '# workflow note' > documentation/work.md && git add documentation/work.md && git commit -m 'docs(markdown): update workflow note' -m 'Part of #123' >/dev/null && MOCK_MULTI_ASSIGNEE_ISSUES='123' /bin/bash '${HOOKS_DIR}/pre-push'"

	run_case \
		"pre-push-docs-only-runs-markdownlint" \
		0 \
		"Markdown lint OK" \
		"echo '# markdown update' > documentation/markdownlint.md && git add documentation/markdownlint.md && git commit -m 'docs(markdown): add markdownlint doc file' >/dev/null && /bin/bash '${HOOKS_DIR}/pre-push'"

	run_case \
		"pre-push-docs-only-blocks-on-markdownlint-failure" \
		1 \
		"Markdown lint failed" \
		"echo '# markdown update' > documentation/markdownlint_fail.md && git add documentation/markdownlint_fail.md && git commit -m 'docs(markdown): add markdownlint failing file' >/dev/null && MOCK_MARKDOWNLINT_FAIL=1 /bin/bash '${HOOKS_DIR}/pre-push'"

	# pre-push: block root parent refs in pushed commit range.
	run_case \
		"pre-push-blocks-root-parent" \
		1 \
		"Root parent issue references detected" \
		"echo 'note' >> documentation/work.md && git add documentation/work.md && git commit -m 'docs: update workflow note' -m 'Part of #617' >/dev/null && /bin/bash '${HOOKS_DIR}/pre-push'"

	run_case \
		"pre-push-allows-base-parent-reference" \
		0 \
		"Pre-push checks PASSED" \
		"echo 'note' >> documentation/work.md && git add documentation/work.md && git commit -m 'docs: update workflow note' -m 'Part of #618' >/dev/null && MOCK_BASE_PARENT_ISSUES='618' MOCK_PARENT_WITH_CHILDREN='618' MOCK_MULTI_ASSIGNEE_ISSUES='618' /bin/bash '${HOOKS_DIR}/pre-push'"

	# post-checkout: warn when branch history references root parent.
	run_case \
		"post-checkout-warns-on-root-parent" \
		0 \
		"Convention warning on branch checkout" \
		"echo 'note' >> documentation/work.md && git add documentation/work.md && git commit -m 'docs: update workflow note' -m 'Part of #617' >/dev/null && /bin/bash '${HOOKS_DIR}/post-checkout' HEAD~1 HEAD 1"

	run_case \
		"post-checkout-no-warning-on-base-parent-reference" \
		0 \
		"" \
		"echo 'note' >> documentation/work.md && git add documentation/work.md && git commit -m 'docs: update workflow note' -m 'Part of #618' >/dev/null && MOCK_BASE_PARENT_ISSUES='618' MOCK_PARENT_WITH_CHILDREN='618' /bin/bash '${HOOKS_DIR}/post-checkout' HEAD~1 HEAD 1"

	echo ""
	echo "Summary: ${TESTS_RUN} run, ${TESTS_FAILED} failed."
	if [[ "${TESTS_FAILED}" -ne 0 ]]; then
		exit 1
	fi
}

main "$@"
