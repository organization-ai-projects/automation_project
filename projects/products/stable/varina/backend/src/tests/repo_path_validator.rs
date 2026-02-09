// projects/products/stable/varina/backend/src/tests/repo_path_validator.rs

use std::path::PathBuf;

use crate::{
    repo_path_validator::RepoPathValidator,
    validation_error::{
        E_REPO_PATH_INVALID_FORMAT, E_REPO_PATH_NOT_WHITELISTED, E_REPO_PATH_TRAVERSAL,
    },
};
#[test]
fn test_path_traversal_rejected() {
    let validator = RepoPathValidator::default();
    // This path tries to escape to /opt which is not in the default whitelist
    let result = validator.validate("/home/user/../../opt/config");
    assert!(result.is_err());
    let err = result.unwrap_err();
    // After normalization, this should be caught by whitelist check
    assert_eq!(err.code, E_REPO_PATH_NOT_WHITELISTED);
}

#[test]
fn test_empty_path_rejected() {
    let validator = RepoPathValidator::default();
    let result = validator.validate("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, E_REPO_PATH_INVALID_FORMAT);
    assert!(err.message.contains("cannot be empty"));
}

#[test]
fn test_whitespace_path_rejected() {
    let validator = RepoPathValidator::default();
    let result = validator.validate("   ");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, E_REPO_PATH_INVALID_FORMAT);
}

#[test]
fn test_whitelisted_path_accepted() {
    let validator = RepoPathValidator::new(vec![PathBuf::from("/home")]);
    // Use a path that doesn't need to exist for the test
    let result = validator.validate("/home/user/repo");
    assert!(result.is_ok());
}

#[test]
fn test_non_whitelisted_path_rejected() {
    let validator = RepoPathValidator::new(vec![PathBuf::from("/home")]);
    let result = validator.validate("/etc/config");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, E_REPO_PATH_NOT_WHITELISTED);
    assert!(err.message.contains("not in the whitelist"));
}

#[test]
fn test_empty_whitelist_allows_all() {
    let validator = RepoPathValidator::new(vec![]);
    let result = validator.validate("/any/path");
    assert!(result.is_ok());
}

#[test]
fn test_tmp_path_accepted_by_default() {
    let validator = RepoPathValidator::default();
    let result = validator.validate("/tmp/test-repo");
    assert!(result.is_ok());
}

#[test]
fn test_workspace_path_accepted_by_default() {
    let validator = RepoPathValidator::default();
    let result = validator.validate("/workspace/project");
    assert!(result.is_ok());
}

#[test]
fn test_legitimate_parent_dir_navigation_accepted() {
    let validator = RepoPathValidator::new(vec![PathBuf::from("/home")]);
    // Legitimate path with parent dir that stays within whitelist
    let result = validator.validate("/home/user/../user/repo");
    assert!(result.is_ok());
    // Verify it normalizes to the expected path
    let path = result.unwrap();
    assert_eq!(path, PathBuf::from("/home/user/repo"));
}

#[test]
fn test_traversal_above_root_rejected() {
    let validator = RepoPathValidator::new(vec![PathBuf::from("/home")]);
    // Path that tries to traverse above root - these normalize but may be rejected by whitelist
    let result = validator.validate("/../../etc/passwd");
    assert!(result.is_err());
    let err = result.unwrap_err();
    // This will either be traversal (above root) or whitelist error
    assert!(err.code == E_REPO_PATH_TRAVERSAL || err.code == E_REPO_PATH_NOT_WHITELISTED);
}
