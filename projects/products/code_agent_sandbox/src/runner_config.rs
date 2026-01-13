// projects/products/code_agent_sandbox/src/runner.rs
#[derive(Debug, Clone, Default)]
pub struct RunnerConfig {
    pub allowed_bins: Vec<String>,
    pub allowed_cargo_subcommands: Vec<String>,
    pub timeout_ms: u64,
    pub env_allowlist: Vec<String>,
    pub cargo_path: String, // Absolute path to the cargo binary
}
