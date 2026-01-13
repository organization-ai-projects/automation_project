use crate::actions::low_level_action_context::{run_low_level_actions, LowLevelActionContext};
use crate::command_runner::CommandRunner;
use crate::engine::EngineConfig;
use crate::engine::Request;
use crate::engine::Response;
use crate::journal::Journal;
use crate::policy::Policy;
use crate::sandbox_fs::SandboxFs;
use anyhow::Error;
use std::path::Path;

pub fn handle_actions(
    req: &Request,
    sandbox_fs: &SandboxFs,
    command_runner: &CommandRunner,
) -> Result<Response, Error> {
    let mut ctx = LowLevelActionContext {
        policy: &Policy::default(), // Replace with actual policy if needed
        sfs: sandbox_fs,
        runner: command_runner,
        run_dir: &Path::new("."), // Replace with actual run_dir if needed
        journal: &mut Journal::default(), // Replace with actual journal if needed
        config: &EngineConfig::default(), // Replace with actual config if needed
    };

    let results = run_low_level_actions("run_id_placeholder", &req.actions, &mut ctx)?;
    Ok(Response::new(results))
}
