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
  "start_work|scripts/versioning/file_versioning/orchestrators/execute/start_work.sh|Primary start-work flow"
  "branching|scripts/versioning/file_versioning/git/create_branch.sh|Create branch from dev"
  "branching|scripts/versioning/file_versioning/git/create_work_branch.sh|Create branch by type/description"
  "commit_push|scripts/versioning/file_versioning/git/add_commit_push.sh|Validate message then commit+push"
  "commit_push|scripts/versioning/file_versioning/git/push_branch.sh|Push current branch"
  "pre_push|scripts/automation/pre_push_check.sh|Repository pre-push checks"
  "pr_creation|scripts/versioning/file_versioning/orchestrators/read/create_pr.sh|Create PR with defaults"
  "pr_creation|scripts/versioning/file_versioning/github/generate_pr_description.sh|Generate/refresh PR body or create PR"
  "issue_creation|scripts/versioning/file_versioning/github/create_direct_issue.sh|Create direct issue from contract"
)

print_inventory() {
  echo "Workflow | Script | Purpose"
  echo "--- | --- | ---"
  for row in "${USER_FACING_ROWS[@]}"; do
    IFS='|' read -r workflow script_path purpose <<< "$row"
    echo "${workflow} | ${script_path} | ${purpose}"
  done
}

parse_root_rel_from_line() {
  local line="$1"
  sed -nE 's/.*\$SCRIPT_DIR\/([^\"]+)\".*$/\1/p' <<< "$line"
}

check_root_resolution() {
  local script_path="$1"
  local root_line
  local rel
  local computed

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

  for script_path in \
    scripts/automation/*.sh \
    scripts/versioning/file_versioning/git/*.sh \
    scripts/versioning/file_versioning/orchestrators/execute/*.sh \
    scripts/versioning/file_versioning/orchestrators/read/*.sh \
    scripts/versioning/file_versioning/github/*.sh; do
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
  echo "\nScript integrity checks passed."
  exit 0
fi

echo "\nScript integrity checks failed." >&2
exit 1
