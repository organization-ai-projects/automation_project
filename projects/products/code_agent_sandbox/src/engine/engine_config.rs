// projects/products/code_agent_sandbox/src/engine/engine_config.rs
#[derive(Clone, Debug, Default)]
pub struct EngineConfig {
    pub max_read_bytes: usize,
    pub max_write_bytes: usize,
    pub max_files_per_request: usize,
    pub timeout_ms: u64,
}
