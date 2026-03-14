#!/usr/bin/env bash

resolve_versioning_automation_bin() {
	if [[ -n "${VERSIONING_AUTOMATION_BIN:-}" && -x "${VERSIONING_AUTOMATION_BIN}" ]]; then
		printf '%s\n' "${VERSIONING_AUTOMATION_BIN}"
		return 0
	fi

	if command -v versioning_automation >/dev/null 2>&1; then
		command -v versioning_automation
		return 0
	fi

	local root_dir
	root_dir="$(git rev-parse --show-toplevel 2>/dev/null || true)"
	if [[ -n "$root_dir" && -x "$root_dir/target/debug/versioning_automation" ]]; then
		printf '%s\n' "$root_dir/target/debug/versioning_automation"
		return 0
	fi

	return 1
}

versioning_automation_output_required() {
	local va_bin
	va_bin="$(resolve_versioning_automation_bin)" || {
		echo "❌ versioning_automation binary is required but was not found." >&2
		echo "   Build it with: cargo build -p versioning_automation" >&2
		return 1
	}

	local output
	if ! output="$("$va_bin" "$@" 2>/dev/null)"; then
		echo "❌ versioning_automation failed for command: $*" >&2
		return 1
	fi

	printf '%s\n' "$output"
	return 0
}

resolve_repo_name_with_owner() {
	if [[ -n "${GH_REPO:-}" ]]; then
		printf '%s\n' "$GH_REPO"
		return 0
	fi

	local va_repo
	va_repo="$(versioning_automation_output_required issue repo-name)" || return 1
	if [[ -z "$va_repo" ]]; then
		echo "❌ versioning_automation returned an empty repository name." >&2
		return 1
	fi
	printf '%s\n' "$va_repo"
	return 0
}

extract_issue_refs_from_text() {
	local text="$1"
	local refs_file
	local output
	refs_file="$(mktemp)"
	printf '%s' "$text" >"$refs_file"
	if ! output="$(versioning_automation_output_required issue extract-refs --profile hook --file "$refs_file")"; then
		rm -f "$refs_file"
		return 1
	fi
	rm -f "$refs_file"
	printf '%s\n' "$output"
	return 0
}

issue_assignee_logins() {
	local issue_number="$1"
	local repo="$2"

	local assignees
	assignees="$(versioning_automation_output_required issue assignee-logins --issue "$issue_number" --repo "$repo")" || return 1
	printf '%s\n' "$assignees"
	return 0
}

issue_current_login() {
	local login
	login="$(versioning_automation_output_required issue current-login)" || return 1
	printf '%s\n' "$login"
	return 0
}

issue_is_root_parent() {
	local issue_number="$1"
	local repo="$2"
	local is_root_parent

	is_root_parent="$(versioning_automation_output_required issue is-root-parent --issue "$issue_number" --repo "$repo")" || return 1
	case "$is_root_parent" in
	true)
		return 0
		;;
	false)
		return 1
		;;
	*)
		echo "❌ Unexpected output from versioning_automation issue is-root-parent: '$is_root_parent'" >&2
		return 1
		;;
	esac
}
