// projects/products/code_agent_sandbox/src/engine/engine_config.rs
use common_time::TimeSpan;

#[derive(Clone, Debug, Default)]
pub(crate) struct EngineConfig {
    pub(crate) max_read_bytes: usize,
    pub(crate) max_write_bytes: usize,
    pub(crate) max_files_per_request: usize,
    pub(crate) timeout: TimeSpan,
}

impl EngineConfig {
    pub(crate) fn new(timeout: TimeSpan) -> Self {
        Self {
            max_read_bytes: Default::default(),
            max_write_bytes: Default::default(),
            max_files_per_request: Default::default(),
            timeout,
        }
    }
}
