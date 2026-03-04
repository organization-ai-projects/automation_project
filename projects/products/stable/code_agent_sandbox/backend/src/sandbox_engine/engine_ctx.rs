// projects/products/code_agent_sandbox/src/engine/engine_ctx.rs
use crate::{command_runner::CommandRunner, journal::Journal, sandbox_fs::SandboxFs};

// replace run_id issue 67
pub(crate) struct EngineCtx<'a> {
    pub(crate) run_id: String,
    pub(crate) sfs: SandboxFs,
    pub(crate) runner: CommandRunner,
    pub(crate) journal: &'a mut Journal,
}
