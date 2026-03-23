//! projects/products/stable/code_agent_sandbox/backend/src/sandbox_engine/tests/engine_config.rs

use common_time::TimeSpan;

use crate::sandbox_engine::EngineConfig;

#[test]
fn engine_config_new_sets_timeout_and_zero_limits() {
    let timeout = TimeSpan::from_secs(30);
    let cfg = EngineConfig::new(timeout);

    assert_eq!(cfg.timeout, timeout);
    assert_eq!(cfg.max_read_bytes, 0);
    assert_eq!(cfg.max_write_bytes, 0);
    assert_eq!(cfg.max_files_per_request, 0);
}
