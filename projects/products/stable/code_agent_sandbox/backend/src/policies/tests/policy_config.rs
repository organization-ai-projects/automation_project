//! projects/products/stable/code_agent_sandbox/backend/src/policies/tests/policy_config.rs
use std::path::PathBuf;

use common_time::TimeSpan;

use crate::policies::PolicyConfig;
use crate::sandbox_engine::{EngineConfig, EnginePaths};

#[test]
fn policy_config_new_copies_limits_and_globs() {
    let paths = EnginePaths {
        repo_root: PathBuf::from("/repo"),
        runs_root: PathBuf::from("/runs"),
    };
    let run_dir = PathBuf::from("/runs/run-1");
    let work_root = PathBuf::from("/runs/run-1/work");

    let mut engine = EngineConfig::new(TimeSpan::from_secs(5));
    engine.max_read_bytes = 100;
    engine.max_write_bytes = 200;
    engine.max_files_per_request = 3;

    let cfg = PolicyConfig::new(
        &paths,
        &run_dir,
        work_root.clone(),
        &engine,
        vec!["target/**".to_string()],
        vec!["src/**".to_string()],
        vec!["src/**".to_string()],
    );

    assert_eq!(cfg.max_read_bytes, 100);
    assert_eq!(cfg.max_write_bytes, 200);
    assert_eq!(cfg.max_files_per_request, 3);
    assert_eq!(cfg.context.source_repo_root, PathBuf::from("/repo"));
    assert_eq!(cfg.context.paths.work_root, work_root);
}
