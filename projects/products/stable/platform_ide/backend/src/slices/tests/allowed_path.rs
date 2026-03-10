//! projects/products/stable/platform_ide/backend/src/slices/tests/allowed_path.rs
use crate::slices::AllowedPath;
use crate::slices::allowed_path::is_safe_path;

#[test]
fn safe_path_accepts_normal_paths() {
    assert!(is_safe_path("src/main.rs"));
    assert!(is_safe_path("README.md"));
    assert!(is_safe_path("a/b/c/d.txt"));
}

#[test]
fn safe_path_rejects_traversal() {
    assert!(!is_safe_path("../etc/passwd"));
    assert!(!is_safe_path("src/../../secret"));
    assert!(!is_safe_path("/absolute/path"));
    assert!(!is_safe_path(""));
}

#[test]
fn allowed_path_display() {
    let p = AllowedPath::new_validated("src/lib.rs".to_string());
    assert_eq!(p.to_string(), "src/lib.rs");
    assert_eq!(p.as_str(), "src/lib.rs");
}

#[test]
fn allowed_path_equality() {
    let a = AllowedPath::new_validated("src/lib.rs".to_string());
    let b = AllowedPath::new_validated("src/lib.rs".to_string());
    assert_eq!(a, b);
}
