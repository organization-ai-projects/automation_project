// projects/products/code_agent_sandbox/src/policies/tests/policy.rs
use std::error::Error;
use std::path::PathBuf;

use crate::{
    actions::Action,
    execution_context::ExecutionContext,
    execution_paths::ExecutionPaths,
    policies::{Policy, PolicyConfig},
};

type TestResult = Result<(), Box<dyn Error>>;

/// Helper to create a temporary override file with the given content.
/// Returns a NamedTempFile that will automatically clean up when dropped.
fn create_temp_override_file(content: &str) -> Result<tempfile::NamedTempFile, Box<dyn Error>> {
    // Use `tempfile` to create a uniquely named file in the global temp directory.
    let mut named_file = tempfile::Builder::new()
        .prefix("policy_overrides_")
        .suffix(".toml")
        .tempfile_in(std::env::temp_dir())?;

    // Write the provided content into the temporary file.
    use std::io::Write;
    named_file.write_all(content.as_bytes())?;
    named_file.flush()?;

    Ok(named_file)
}

fn make_policy(
    forbid_globs: Vec<String>,
    allow_read_globs: Vec<String>,
    allow_write_globs: Vec<String>,
) -> Policy {
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
        forbid_globs,
        allow_read_globs,
        allow_write_globs,
    };

    Policy::new(cfg).expect("Failed to create Policy")
}

#[test]
fn test_forbid_wins_over_allow() {
    let policy = make_policy(
        vec!["src/forbidden/**".into()],
        vec!["src/**".into()],
        vec![],
    );

    // Path is allowed by `allow_read_globs` but forbidden by `forbid_globs`
    let result = policy.resolve_work_path_for_read("src/forbidden/file.txt");
    assert!(result.is_err(), "Expected an error, but got Ok");
    let err = result.unwrap_err();
    assert!(err.to_string().contains("forbidden path by policy"));

    // Path is allowed by `allow_read_globs` and not forbidden
    policy
        .resolve_work_path_for_read("src/allowed/file.txt")
        .expect("Expected Ok for allowed path");

    // Path is not specified in any glob
    let result = policy.resolve_work_path_for_read("unknown/file.txt");
    assert!(result.is_err(), "Expected an error for unknown path");
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not allowed by policy"));
}

#[test]
fn test_invalid_relative_path_segments() {
    let policy = make_policy(vec![], vec!["src/**".into()], vec![]);

    let err = policy
        .resolve_work_path_for_read("../src/allowed.txt")
        .unwrap_err();
    assert!(err.to_string().contains("invalid relative path"));

    let err = policy
        .resolve_work_path_for_read("src/./allowed.txt")
        .unwrap_err();
    assert!(err.to_string().contains("invalid relative path"));

    let err = policy
        .resolve_work_path_for_read("src//allowed.txt")
        .unwrap_err();
    assert!(err.to_string().contains("invalid relative path"));
}

#[test]
fn test_write_requires_allow_write_glob() {
    let policy = make_policy(vec![], vec!["read/**".into()], vec!["write/**".into()]);

    assert!(policy.resolve_work_path_for_write("write/file.txt").is_ok());
    let err = policy
        .resolve_work_path_for_write("read/file.txt")
        .unwrap_err();
    assert!(err.to_string().contains("not allowed by policy"));
}

#[test]
fn test_invalid_glob_pattern_is_error() {
    let policy = make_policy(vec!["[".into()], vec!["src/**".into()], vec![]);
    let err = policy
        .resolve_work_path_for_read("src/allowed.txt")
        .unwrap_err();
    assert!(err.to_string().contains("Invalid glob pattern"));
}

#[test]
fn test_authorize_action_wraps_error_context() {
    let policy = make_policy(vec![], vec!["read/**".into()], vec![]);
    let action = Action::ReadFile {
        path: "forbidden/file.txt".to_string(),
    };

    let err = policy.authorize_action(&action).unwrap_err();
    assert!(err.to_string().contains("Policy authorization failed"));
}

#[test]
fn test_load_with_overrides_adds_forbid_globs() -> TestResult {
    let policy = make_policy(vec![], vec!["read/**".into()], vec![]);
    let cfg = policy.config().clone();

    let content = r#"
        forbid_globs = ["read/blocked/**"]
        allow_read_globs = ["ignored/**"]
    "#;
    let temp_file = create_temp_override_file(content)?;

    let policy = Policy::load_with_overrides(cfg, temp_file.path())?;

    assert!(
        policy
            .cfg
            .forbid_globs
            .iter()
            .any(|g| g == "read/blocked/**")
    );
    assert!(policy.cfg.allow_read_globs.iter().any(|g| g == "read/**"));
    assert!(
        !policy
            .cfg
            .allow_read_globs
            .iter()
            .any(|g| g == "ignored/**")
    );

    // temp_file is automatically cleaned up when it goes out of scope
    Ok(())
}
