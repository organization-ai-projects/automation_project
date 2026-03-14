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

normalize_parent_value() {
	local raw="$1"
	raw="$(printf '%s' "$raw" | tr '[:upper:]' '[:lower:]')"
	raw="$(printf '%s' "$raw" | sed -E 's/^[[:space:]]+//; s/[[:space:]]+$//')"
	raw="${raw#(}"
	raw="${raw%)}"
	printf '%s\n' "$raw"
}

split_repo_owner_name() {
	local repo="$1"
	local out_owner_var="$2"
	local out_name_var="$3"
	local parsed_owner="${repo%%/*}"
	local parsed_name="${repo#*/}"

	if [[ -z "$parsed_owner" || -z "$parsed_name" || "$parsed_owner" == "$repo" ]]; then
		return 1
	fi

	printf -v "$out_owner_var" '%s' "$parsed_owner"
	printf -v "$out_name_var" '%s' "$parsed_name"
	return 0
}

extract_issue_refs_from_text() {
	local text="$1"
	echo "$text" | awk '
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
  ' | sort -u
}

issue_has_children() {
	local issue_number="$1"
	local repo="$2"

	local owner
	local repo_name
	local subissue_refs
	if ! split_repo_owner_name "$repo" owner repo_name; then
		echo "❌ Invalid repository format '$repo' (expected owner/name)." >&2
		return 1
	fi

	subissue_refs="$(versioning_automation_output_required issue subissue-refs --owner "$owner" --repo "$repo_name" --issue "$issue_number")" || return 1
	[[ -n "$subissue_refs" ]]
}

issue_body_value() {
	local issue_number="$1"
	local repo="$2"

	local body
	body="$(versioning_automation_output_required issue field --issue "$issue_number" --name body --repo "$repo")" || return 1
	printf '%s\n' "$body"
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

issue_parent_value() {
	local issue_number="$1"
	local repo="$2"
	local body
	local parent_line
	local parent_value

	body="$(issue_body_value "$issue_number" "$repo")"
	parent_line="$(printf '%s\n' "$body" | grep -iE '^[[:space:]]*Parent:[[:space:]]*(#?[0-9]+|none|base|epic|\(none\)|\(base\)|\(epic\))[[:space:]]*$' | tail -n1 || true)"
	if [[ -z "$parent_line" ]]; then
		printf 'none\n'
		return 0
	fi

	parent_value="$(printf '%s\n' "$parent_line" | sed -E 's/^[[:space:]]*Parent:[[:space:]]*//I')"
	normalize_parent_value "$parent_value"
}

issue_is_root_parent() {
	local issue_number="$1"
	local repo="$2"
	local parent_value

	parent_value="$(issue_parent_value "$issue_number" "$repo")"
	case "$parent_value" in
	epic)
		return 0
		;;
	base)
		# `Parent: base` is a cascade root marker and can still be referenced in
		# commit trailers by project policy.
		return 1
		;;
	none | "")
		# `Parent: none` means independent issue. If children are present, treat as protected parent
		# to prevent accidental closure and prompt explicit `Parent: base`.
		issue_has_children "$issue_number" "$repo"
		return $?
		;;
	\#*)
		# Child/middle nodes remain referenceable in commit trailers.
		return 1
		;;
	*)
		return 1
		;;
	esac
}
