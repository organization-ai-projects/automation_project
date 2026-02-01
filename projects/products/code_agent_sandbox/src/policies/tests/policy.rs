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
    match policy.resolve_work_path_for_read("src/forbidden/file.txt") {
        Ok(_) => panic!("Expected an error, but got Ok"),
        Err(err) => {
            assert!(err.to_string().contains("forbidden path by policy"));
        }
    }

    // Path is allowed by `allow_read_globs` and not forbidden
    match policy.resolve_work_path_for_read("src/allowed/file.txt") {
        Ok(_) => (),
        Err(err) => panic!("Expected Ok, but got error: {}", err),
    }

    // Path is not specified in any glob
    match policy.resolve_work_path_for_read("unknown/file.txt") {
        Ok(_) => panic!("Expected an error, but got Ok"),
        Err(err) => {
            assert!(err.to_string().contains("not allowed by policy"));
        }
    }
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

    let mut overrides_path = std::env::temp_dir();
    overrides_path.push(format!(
        "policy_overrides_{}.toml",
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_nanos()
    ));

    let content = r#"
        forbid_globs = ["read/blocked/**"]
        allow_read_globs = ["ignored/**"]
    "#;
    std::fs::write(&overrides_path, content)?;

    let policy = Policy::load_with_overrides(cfg, &overrides_path)?;

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

    let _ = std::fs::remove_file(&overrides_path);
    Ok(())
}
