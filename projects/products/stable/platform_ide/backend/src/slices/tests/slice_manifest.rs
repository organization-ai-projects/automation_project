//! projects/products/stable/platform_ide/backend/src/slices/tests/slice_manifest.rs
use crate::slices::SliceManifest;

fn manifest() -> SliceManifest {
    SliceManifest::new("issue-42", "abc123", ["src/main.rs", "README.md"])
}

#[test]
fn allow_returns_path_for_manifest_member() {
    let m = manifest();
    let p = m.allow("src/main.rs").expect("path should be allowed");
    assert_eq!(p.as_str(), "src/main.rs");
}

#[test]
fn allow_rejects_non_manifest_path() {
    let m = manifest();
    let err = m
        .allow("src/secret.rs")
        .expect_err("path should be rejected");
    let msg = err.to_string();
    assert!(!msg.contains("secret.rs"));
    assert!(!msg.contains("src/main.rs"));
}

#[test]
fn allow_rejects_traversal() {
    let m = manifest();
    assert!(m.allow("../etc/passwd").is_err());
    assert!(m.allow("src/../../other").is_err());
}

#[test]
fn allow_rejects_absolute() {
    let m = manifest();
    assert!(m.allow("/absolute").is_err());
}

#[test]
fn len_and_iter() {
    let m = manifest();
    assert_eq!(m.len(), 2);
    let mut paths: Vec<&str> = m.iter().collect();
    paths.sort_unstable();
    assert_eq!(paths, ["README.md", "src/main.rs"]);
}
