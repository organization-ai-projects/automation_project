//! projects/products/stable/code_agent_sandbox/backend/src/tests/sandbox_fs.rs
use std::path::PathBuf;

use common_time::TimeSpan;

use crate::execution_context::ExecutionContext;
use crate::execution_paths::ExecutionPaths;
use crate::policies::{Policy, PolicyConfig};
use crate::sandbox_engine::EngineConfig;
use crate::sandbox_fs::SandboxFs;

fn sandbox_fs_for(work_root: PathBuf) -> SandboxFs {
    let mut engine_cfg = EngineConfig::new(TimeSpan::from_secs(1));
    engine_cfg.max_read_bytes = 8 * 1024;
    engine_cfg.max_write_bytes = 8 * 1024;
    engine_cfg.max_files_per_request = 128;

    let context = ExecutionContext {
        source_repo_root: work_root.clone(),
        paths: ExecutionPaths::new(work_root.clone(), work_root.clone()),
    };
    let cfg = PolicyConfig {
        context,
        max_read_bytes: engine_cfg.max_read_bytes,
        max_write_bytes: engine_cfg.max_write_bytes,
        max_files_per_request: engine_cfg.max_files_per_request,
        forbid_globs: vec![],
        allow_write_globs: vec!["**".to_string()],
        allow_read_globs: vec!["**".to_string()],
    };

    let policy = Policy::new(cfg).expect("policy build");
    SandboxFs::new(policy)
}

#[test]
fn write_then_read_file_returns_success() {
    let temp = tempfile::tempdir().expect("temp dir");
    let sfs = sandbox_fs_for(temp.path().to_path_buf());

    let write = sfs
        .write_file("src/lib.rs", "pub fn x() {}", true)
        .expect("write result");
    assert!(write.ok);

    let read = sfs.read_file("src/lib.rs").expect("read result");
    assert!(read.ok);
    assert_eq!(read.kind, "ReadFile");
}
