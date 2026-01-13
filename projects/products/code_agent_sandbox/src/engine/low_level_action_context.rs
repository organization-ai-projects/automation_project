// projects/products/code_agent_sandbox/src/engine/low_level_action_context.rs
use std::path::Path;

use crate::{
    actions::{Action, ActionResult},
    command_runner::CommandRunner,
    engine::{
        handle_generate_code, record_action_event, record_and_push_result, records, EngineConfig,
    },
    journal::Journal,
    policy::Policy,
    sandbox_fs::SandboxFs,
};

pub struct LowLevelActionContext<'a> {
    pub policy: &'a Policy,
    pub sfs: &'a SandboxFs,
    pub runner: &'a CommandRunner,
    pub run_dir: &'a Path,
    pub journal: &'a mut Journal,
    pub config: &'a EngineConfig,
}

pub fn run_low_level_actions(
    run_id: &str,
    actions: &[Action],
    ctx: &mut LowLevelActionContext, // Regroupement des paramètres dans une structure
) -> Result<Vec<ActionResult>, anyhow::Error> {
    if actions.is_empty() {
        return Ok(Vec::new());
    }

    let mut results = Vec::with_capacity(actions.len());

    let estimated_touches: usize = actions.iter().map(|a| a.estimated_file_touch_count()).sum();
    if estimated_touches > ctx.config.max_files_per_request {
        let res = ActionResult::error(
            "PolicyViolation",
            format!(
                "Too many files touched in one request (estimated {} > max {})",
                estimated_touches, ctx.config.max_files_per_request
            ),
        );
        record_and_push_result(ctx.journal, run_id, res, &mut results)?;
        return Ok(results);
    }

    let mut files_touched = 0usize;

    for action in actions {
        files_touched += action.estimated_file_touch_count();

        record_action_event(ctx.journal, run_id, action)?;

        if records::check_file_limit(
            files_touched,
            ctx.config.max_files_per_request,
            run_id,
            ctx.journal,
            &mut results,
        )? {
            break;
        }

        let exec = match action {
            Action::ReadFile { path } => ctx.sfs.read_file(path),
            Action::ListDir { path, max_depth } => ctx.sfs.list_dir(path, *max_depth),
            Action::WriteFile {
                path,
                contents,
                create_dirs,
            } => ctx.sfs.write_file(path, contents, *create_dirs),
            Action::ApplyUnifiedDiff { path, unified_diff } => {
                ctx.sfs.apply_unified_diff(path, unified_diff)
            }
            Action::RunCargo { subcommand, args } => ctx.runner.run_cargo(subcommand, args),
            Action::GenerateCode { language, code } => handle_generate_code(language, code, ctx),
        };

        let res = match exec {
            Ok(ok) => ok,
            Err(e) => ActionResult::error("ExecutionError", format!("{:#}", e)),
        };

        record_and_push_result(ctx.journal, run_id, res, &mut results)?;
    }

    Ok(results)
}
