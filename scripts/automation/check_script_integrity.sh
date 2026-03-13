#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

cd "$ROOT_DIR"

if ! command -v git >/dev/null 2>&1; then
	echo "Error: git is required." >&2
	exit 3
fi

REPO_ROOT="$(git rev-parse --show-toplevel)"

declare -a USER_FACING_ROWS=(
	"branching|versioning_automation git create-branch ...|Create branch from dev"
	"branching|versioning_automation git create-work-branch ...|Create branch by type/description"
	"commit_push|versioning_automation git add-commit-push ...|Validate message then commit+push"
	"commit_push|versioning_automation git push-branch ...|Push current branch"
	"pre_push|scripts/automation/git_hooks/pre-push|Repository pre-push checks"
	"pr_creation|versioning_automation pr generate-description ...|Canonical PR create/refresh entrypoint (Rust CLI)"
	"issue_creation|versioning_automation issue create ...|Canonical direct issue creation entrypoint (Rust CLI)"
	"issue_lifecycle|versioning_automation issue <read/update/close/reopen/delete> ...|Canonical issue lifecycle entrypoint (Rust CLI)"
)

print_inventory() {
	echo "Workflow | Script | Purpose"
	echo "--- | --- | ---"
	for row in "${USER_FACING_ROWS[@]}"; do
		IFS='|' read -r workflow script_path purpose <<<"$row"
		echo "${workflow} | ${script_path} | ${purpose}"
	done
}

parse_root_rel_from_line() {
	local line="$1"
	# shellcheck disable=SC2016
	sed -nE 's/.*\$SCRIPT_DIR\/([^\"]+)\".*$/\1/p' <<<"$line"
}

check_root_resolution() {
	local script_path="$1"
	local root_line
	local rel
	local computed

	# shellcheck disable=SC2016
	root_line="$(grep -E '^[[:space:]]*ROOT_DIR=\"\$\(cd \"\$SCRIPT_DIR/.+\" && pwd\)\"' "$script_path" | head -n1 || true)"
	if [[ -z "$root_line" ]]; then
		return 0
	fi

	rel="$(parse_root_rel_from_line "$root_line")"
	if [[ -z "$rel" ]]; then
		echo "ERROR [$script_path] Unable to parse ROOT_DIR relative path from: $root_line" >&2
		return 1
	fi

	computed="$(cd "$(dirname "$script_path")/$rel" && pwd)"
	if [[ "$computed" != "$REPO_ROOT" ]]; then
		echo "ERROR [$script_path] ROOT_DIR resolves to '$computed' (expected '$REPO_ROOT')." >&2
		return 1
	fi

	return 0
}

check_root_source_paths_exist() {
	local script_path="$1"
	local source_rel
	local missing=0

	# shellcheck disable=SC2016
	while IFS= read -r source_rel; do
		[[ -z "$source_rel" ]] && continue
		if [[ ! -f "$REPO_ROOT/$source_rel" ]]; then
			echo "ERROR [$script_path] Missing sourced file: $source_rel" >&2
			missing=1
		fi
	done < <(sed -nE 's|^[[:space:]]*source "\$ROOT_DIR/([^"]+)".*$|\1|p' "$script_path")

	[[ "$missing" -eq 0 ]]
}

check_required_helper_imports() {
	local script_path="$1"

	if grep -qE '\bgit_fetch_prune\b' "$script_path"; then
		# shellcheck disable=SC2016
		if ! grep -q 'source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/synch.sh"' "$script_path"; then
			echo "ERROR [$script_path] Uses git_fetch_prune but does not source git/synch.sh" >&2
			return 1
		fi
	fi

	return 0
}

run_checks_for_script() {
	local script_path="$1"
	local failed=0

	if ! bash -n "$script_path"; then
		echo "ERROR [$script_path] bash -n failed" >&2
		failed=1
	fi

	check_root_resolution "$script_path" || failed=1
	check_root_source_paths_exist "$script_path" || failed=1
	check_required_helper_imports "$script_path" || failed=1

	return "$failed"
}

run_all_checks() {
	local failed=0
	local script_path

	for script_path in scripts/automation/*.sh; do
		[[ -f "$script_path" ]] || continue
		run_checks_for_script "$script_path" || failed=1
	done

	return "$failed"
}

if [[ "${1:-}" == "--inventory" ]]; then
	print_inventory
	exit 0
fi

print_inventory
if run_all_checks; then
	printf "\nScript integrity checks passed.\n"
	exit 0
fi

printf "\nScript integrity checks failed.\n" >&2
exit 1
