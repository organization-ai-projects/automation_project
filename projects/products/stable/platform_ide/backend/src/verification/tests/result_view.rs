//! projects/products/stable/platform_ide/backend/src/verification/tests/result_view.rs
use crate::slices::SliceManifest;
use crate::verification::result_view::ResultView;
use crate::verification::{FindingSeverity, RawFinding};

fn manifest() -> SliceManifest {
    SliceManifest::new("issue-1", "abc", ["src/main.rs", "README.md"])
}

fn raw(severity: FindingSeverity, summary: &str, path: Option<&str>) -> RawFinding {
    RawFinding {
        severity,
        summary: summary.to_string(),
        path: path.map(str::to_string),
        line: path.map(|_| 10),
    }
}

#[test]
fn allowed_path_finding_is_shown() {
    let view = ResultView::from_raw(
        false,
        [raw(
            FindingSeverity::Error,
            "syntax error",
            Some("src/main.rs"),
        )],
        &manifest(),
    );
    assert_eq!(view.findings.len(), 1);
    let finding = &view.findings[0];
    assert_eq!(finding.path.as_deref(), Some("src/main.rs"));
    assert!(finding.line.is_some());
}

#[test]
fn forbidden_path_error_finding_becomes_generic() {
    let view = ResultView::from_raw(
        false,
        [raw(
            FindingSeverity::Error,
            "secret error",
            Some("forbidden/internal.rs"),
        )],
        &manifest(),
    );
    assert_eq!(view.findings.len(), 1);
    let finding = &view.findings[0];
    assert!(finding.path.is_none());
    assert!(finding.line.is_none());
    assert!(!finding.summary.contains("forbidden"));
}

#[test]
fn forbidden_path_info_finding_is_suppressed() {
    let view = ResultView::from_raw(
        true,
        [raw(
            FindingSeverity::Info,
            "style note",
            Some("forbidden/internal.rs"),
        )],
        &manifest(),
    );
    assert!(view.findings.is_empty());
}

#[test]
fn no_path_finding_is_always_shown() {
    let view = ResultView::from_raw(
        false,
        [raw(FindingSeverity::Warning, "global warning", None)],
        &manifest(),
    );
    assert_eq!(view.findings.len(), 1);
    assert!(view.findings[0].path.is_none());
}

#[test]
fn healthy_result_has_no_findings() {
    let view = ResultView::healthy();
    assert!(view.healthy);
    assert!(view.findings.is_empty());
}
