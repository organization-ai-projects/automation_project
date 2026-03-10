//! projects/products/stable/code_agent_sandbox/backend/src/tests/execution_paths.rs
use std::path::PathBuf;

use crate::execution_paths::ExecutionPaths;

#[test]
fn execution_paths_new_preserves_fields() {
    let run_dir = PathBuf::from("/tmp/run");
    let work_root = PathBuf::from("/tmp/work");

    let paths = ExecutionPaths::new(run_dir.clone(), work_root.clone());
    assert_eq!(paths.run_dir, run_dir);
    assert_eq!(paths.work_root, work_root);
}
