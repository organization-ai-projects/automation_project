// projects/products/code_agent_sandbox/src/runner.rs
#[derive(Debug, Clone, Default)]
pub(crate) struct RunnerConfig {
    pub(crate) allowed_cargo_subcommands: Vec<String>,
    pub(crate) cargo_path: String,
}
