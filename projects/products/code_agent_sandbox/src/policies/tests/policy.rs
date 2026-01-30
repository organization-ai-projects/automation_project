// projects/products/code_agent_sandbox/src/policies/tests/policy.rs
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        execution_context::ExecutionContext,
        execution_paths::ExecutionPaths,
        policies::{Policy, PolicyConfig},
    };

    #[test]
    fn test_forbid_wins_over_allow() {
        let context = ExecutionContext {
            paths: ExecutionPaths {
                run_dir: PathBuf::from("/run"),
                work_root: PathBuf::from("/work"),
            },
            source_repo_root: PathBuf::from("/repo"),
        };

        let cfg = PolicyConfig {
            context,
            max_read_bytes: 1024,
            max_write_bytes: 1024,
            max_files_per_request: 10,
            forbid_globs: vec!["src/forbidden/**".into()],
            allow_read_globs: vec!["src/**".into()],
            allow_write_globs: vec![],
        };

        let policy =
            Policy::new(cfg).expect("Failed to create Policy with the given configuration");

        // Path is allowed by `allow_read_globs` but forbidden by `forbid_globs`
        let result = policy.resolve_work_path_for_read("src/forbidden/file.txt");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("forbidden path by policy")
        );
    }
}
