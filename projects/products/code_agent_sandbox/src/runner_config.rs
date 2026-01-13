// projects/products/code_agent_sandbox/src/runner.rs
#[derive(Debug, Clone, Default)]
pub struct RunnerConfig {
    pub allowed_cargo_subcommands: Vec<String>,
    pub cargo_path: String, // Absolute path to the cargo binary
}
