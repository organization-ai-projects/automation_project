// projects/products/code_agent_sandbox/src/actions/action_executor.rs
use crate::{
    actions::{Action, ActionResult},
    command_runner::CommandRunner,
    engine::EngineCtx,
};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn execute_action(
    ctx: &mut EngineCtx,
    action: &Action,
    run_dir: &Path,
) -> Result<ActionResult> {
    match action {
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
        Action::RunCargo { subcommand, args } => run_cargo_action(ctx, subcommand, args),
        Action::GenerateCode { language, code } => save_generated_code(run_dir, language, code),
    }
}

pub fn run_cargo_action(
    ctx: &mut EngineCtx,
    subcommand: &str,
    args: &[String],
) -> Result<ActionResult> {
    if CommandRunner::requires_bunker(subcommand) {
        let mut argv = Vec::with_capacity(1 + args.len());
        argv.push(subcommand.to_string());
        argv.extend(args.iter().cloned());
        ctx.runner.run_in_bunker("cargo", &argv)
    } else {
        ctx.runner.run_cargo(subcommand, args)
    }
}

// Moved `save_generated_code` here from `agent_driver.rs`
fn save_generated_code(run_dir: &Path, language: &str, code: &str) -> Result<ActionResult> {
    let file_path = run_dir.join(format!("generated_code.{}", language));
    fs::write(&file_path, code)?;
    Ok(ActionResult::success(
        "CodeGenerated",
        format!("Generated code saved to {}", file_path.display()),
        None,
    ))
}
