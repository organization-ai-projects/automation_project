#!/usr/bin/env bash
set -u

# Exit codes (stable contract for automation)
E_USAGE=2
E_DEPENDENCY=3
E_GIT=4
E_NO_DATA=5
E_PARTIAL=6

SCRIPT_PATH="./scripts/versioning/file_versioning/github/generate_pr_description.sh"
SCRIPT_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${SCRIPT_DIR}/lib/classification.sh"
source "${SCRIPT_DIR}/pr/cli/load.sh"
source "${SCRIPT_DIR}/pr/common/load.sh"
source "${SCRIPT_DIR}/pr/extraction/load.sh"
source "${SCRIPT_DIR}/pr/compare/load.sh"
source "${SCRIPT_DIR}/pr/footprint/load.sh"
source "${SCRIPT_DIR}/lib/issue_refs.sh"
source "${SCRIPT_DIR}/issues/required_fields/load.sh"
source "${SCRIPT_DIR}/pr/issue/load.sh"
source "${SCRIPT_DIR}/pr/metrics/load.sh"
source "${SCRIPT_DIR}/pr/runtime/load.sh"
source "${SCRIPT_DIR}/pr/pipeline/load.sh"
source "${SCRIPT_DIR}/pr/body/load.sh"
source "${SCRIPT_DIR}/lib/rendering.sh"

pr_args_init_defaults
pr_args_parse_cli "$@"
pr_args_finalize

pr_pipeline_init_artifacts_and_state
trap pr_cleanup_tmp_files EXIT

pr_pipeline_check_dependencies
pr_pipeline_resolve_refs_and_modes
pr_pipeline_extract_pr_refs
pr_pipeline_init_issue_tracking

pr_seed_pr_ref_cache
pr_pipeline_collect_issues_from_pr_bodies
pr_pipeline_collect_issues_from_commits_and_main_pr
pr_pipeline_render_issue_outcomes_files

pr_body_build_content
pr_body_apply_validation_only_if_needed
pr_body_emit_generated_output

pr_process_duplicate_mode
pr_body_handle_create_pr
pr_body_handle_auto_edit_pr
pr_body_finalize_exit_status
