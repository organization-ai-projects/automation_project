// projects/products/code_agent_sandbox/src/engine/engine_ctx.rs
use crate::{command_runner::CommandRunner, journal::Journal, sandbox_fs::SandboxFs};

pub struct EngineCtx<'a> {
    pub run_id: String,
    pub sfs: SandboxFs,
    pub runner: CommandRunner,
    pub journal: &'a mut Journal,
}
