//! projects/products/code_agent_sandbox/src/tests/execution_context.rs
use std::path::PathBuf;

use crate::execution_context::ExecutionContext;
use crate::sandbox_engine::EnginePaths;

#[test]
fn execution_context_new_uses_engine_and_runtime_paths() {
    let engine_paths = EnginePaths {
        repo_root: PathBuf::from("/repo"),
        runs_root: PathBuf::from("/runs"),
    };
    let run_dir = PathBuf::from("/runs/abc");
    let work_root = PathBuf::from("/runs/abc/work");

    let context = ExecutionContext::new(&engine_paths, &run_dir, work_root.clone());

    assert_eq!(context.source_repo_root, PathBuf::from("/repo"));
    assert_eq!(context.paths.run_dir, run_dir);
    assert_eq!(context.paths.work_root, work_root);
}
