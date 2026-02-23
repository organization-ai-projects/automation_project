// projects/products/unstable/platform_ide/backend/src/tests/contract_tests.rs
//! Contract tests for the platform-IDE API usage and security properties.
//!
//! These tests verify:
//! 1. Auth flow: session construction and token redaction.
//! 2. Slice load: manifest enforces path membership.
//! 3. Submit: change set validation before submission.
//! 4. Verification results: forbidden paths are never leaked.
//! 5. Offline policy: disabled by default.

use crate::auth::Session;
use crate::changes::ChangeSet;
use crate::diff::{local_diff::DiffLine, LocalDiff};
use crate::editor::FileBuffer;
use crate::errors::IdeError;
use crate::issues::IssueSummary;
use crate::offline::OfflinePolicy;
use crate::slices::{AllowedPath, SliceManifest};
use crate::verification::{FindingSeverity, RawFinding, VerificationResultView};

// ---------------------------------------------------------------------------
// Auth contract: Session never leaks the token in debug output
// ---------------------------------------------------------------------------

#[test]
fn session_debug_redacts_token() {
    let session = Session::new("secret-token-abc", "alice");
    let debug_str = format!("{session:?}");
    assert!(
        !debug_str.contains("secret-token-abc"),
        "token must not appear in debug output: {debug_str}"
    );
    assert!(
        debug_str.contains("[REDACTED]"),
        "token placeholder must be present: {debug_str}"
    );
    assert!(
        debug_str.contains("alice"),
        "subject must be visible: {debug_str}"
    );
}

#[test]
fn session_bearer_token_is_accessible() {
    let session = Session::new("my-token", "bob");
    assert_eq!(session.bearer_token(), "my-token");
    assert_eq!(session.subject, "bob");
}

// ---------------------------------------------------------------------------
// Slice manifest: only allowed paths can be opened
// ---------------------------------------------------------------------------

#[test]
fn manifest_allows_listed_path() {
    let manifest = SliceManifest::new("issue-1", "abc", ["src/main.rs", "README.md"]);
    assert!(manifest.allow("src/main.rs").is_ok());
    assert!(manifest.allow("README.md").is_ok());
}

#[test]
fn manifest_rejects_unlisted_path() {
    let manifest = SliceManifest::new("issue-1", "abc", ["src/main.rs"]);
    let err = manifest.allow("src/secret.rs").unwrap_err();
    let msg = err.to_string();
    assert!(!msg.contains("secret.rs"), "error leaks forbidden path name");
    assert!(!msg.contains("main.rs"), "error leaks allowed path name");
}

#[test]
fn manifest_rejects_traversal_attacks() {
    let manifest = SliceManifest::new("issue-1", "abc", ["../etc/passwd"]);
    assert!(manifest.allow("../etc/passwd").is_err());
    assert!(manifest.allow("src/../../etc/passwd").is_err());
}

#[test]
fn manifest_rejects_absolute_paths() {
    let manifest = SliceManifest::new("issue-1", "abc", ["/etc/passwd"]);
    assert!(manifest.allow("/etc/passwd").is_err());
}

#[test]
fn manifest_rejects_empty_path() {
    let manifest = SliceManifest::new("issue-1", "abc", Vec::<String>::new());
    assert!(manifest.allow("").is_err());
}

// ---------------------------------------------------------------------------
// Editor: FileBuffer only stores AllowedPath
// ---------------------------------------------------------------------------

#[test]
fn file_buffer_tracks_dirty_state() {
    let path = AllowedPath::new_validated("src/lib.rs".to_string());
    let mut buf = FileBuffer::open(path, b"original".to_vec());
    assert!(!buf.is_dirty());
    buf.write(b"modified".to_vec());
    assert!(buf.is_dirty());
    buf.revert();
    assert!(!buf.is_dirty());
}

// ---------------------------------------------------------------------------
// Diff: local diff computes changes correctly
// ---------------------------------------------------------------------------

#[test]
fn local_diff_detects_added_line() {
    let path = AllowedPath::new_validated("a.txt".to_string());
    let mut buf = FileBuffer::open(path, b"line1\n".to_vec());
    buf.write(b"line1\nline2\n".to_vec());
    let diff = LocalDiff::from_buffer(&buf);
    assert!(diff.has_changes());
    assert!(diff.lines.iter().any(|l| matches!(l, DiffLine::Added(s) if s == "line2")));
}

#[test]
fn local_diff_no_changes_when_content_identical() {
    let path = AllowedPath::new_validated("a.txt".to_string());
    let buf = FileBuffer::open(path, b"same content\n".to_vec());
    let diff = LocalDiff::from_buffer(&buf);
    assert!(!diff.has_changes());
}

// ---------------------------------------------------------------------------
// ChangeSet: submission validation
// ---------------------------------------------------------------------------

#[test]
fn change_set_rejects_empty_submit() {
    let cs = ChangeSet::new();
    let err = cs.validate().unwrap_err();
    assert!(matches!(err, IdeError::EmptyChangeSet));
}

#[test]
fn change_set_accepts_dirty_buffer() {
    let mut cs = ChangeSet::new();
    let path = AllowedPath::new_validated("src/lib.rs".to_string());
    let mut buf = FileBuffer::open(path, b"old".to_vec());
    buf.write(b"new".to_vec());
    cs.add_buffer(&buf);
    assert!(cs.validate().is_ok());
    assert_eq!(cs.entries()[0].content, b"new");
}

#[test]
fn change_set_skips_clean_buffers() {
    let mut cs = ChangeSet::new();
    let path = AllowedPath::new_validated("src/lib.rs".to_string());
    let buf = FileBuffer::open(path, b"unchanged".to_vec());
    let added = cs.add_buffer(&buf);
    assert!(!added);
    assert!(cs.is_empty());
}

// ---------------------------------------------------------------------------
// Verification results: forbidden paths never leak
// ---------------------------------------------------------------------------

fn make_manifest() -> SliceManifest {
    SliceManifest::new("issue-1", "abc", ["src/main.rs", "README.md"])
}

#[test]
fn verification_allowed_path_finding_shown_in_full() {
    let view = VerificationResultView::from_raw(
        false,
        [RawFinding {
            severity: FindingSeverity::Error,
            summary: "undefined variable".to_string(),
            path: Some("src/main.rs".to_string()),
            line: Some(42),
        }],
        &make_manifest(),
    );
    assert_eq!(view.findings.len(), 1);
    let f = &view.findings[0];
    assert_eq!(f.path.as_deref(), Some("src/main.rs"));
    assert_eq!(f.line, Some(42));
    assert_eq!(f.summary, "undefined variable");
}

#[test]
fn verification_forbidden_path_error_becomes_generic_without_path() {
    let view = VerificationResultView::from_raw(
        false,
        [RawFinding {
            severity: FindingSeverity::Error,
            summary: "panic in forbidden module".to_string(),
            path: Some("internal/secret.rs".to_string()),
            line: Some(7),
        }],
        &make_manifest(),
    );
    assert_eq!(view.findings.len(), 1);
    let f = &view.findings[0];
    assert!(f.path.is_none(), "forbidden path must be hidden");
    assert!(f.line.is_none(), "line for forbidden path must be hidden");
    let all_text = format!("{:?}", f);
    assert!(
        !all_text.contains("secret.rs"),
        "finding debug output must not leak path: {all_text}"
    );
    assert!(
        !all_text.contains("internal"),
        "finding debug output must not leak directory: {all_text}"
    );
    assert!(
        !all_text.contains("forbidden module"),
        "original summary must not leak: {all_text}"
    );
}

#[test]
fn verification_forbidden_path_info_finding_is_dropped() {
    let view = VerificationResultView::from_raw(
        true,
        [RawFinding {
            severity: FindingSeverity::Info,
            summary: "info hint about internal path".to_string(),
            path: Some("internal/private.rs".to_string()),
            line: None,
        }],
        &make_manifest(),
    );
    assert!(
        view.findings.is_empty(),
        "forbidden info finding must not appear in results"
    );
}

#[test]
fn verification_no_path_finding_always_shown() {
    let view = VerificationResultView::from_raw(
        false,
        [RawFinding {
            severity: FindingSeverity::Warning,
            summary: "build configuration warning".to_string(),
            path: None,
            line: None,
        }],
        &make_manifest(),
    );
    assert_eq!(view.findings.len(), 1);
    assert!(view.findings[0].path.is_none());
}

// ---------------------------------------------------------------------------
// Offline policy: disabled by default
// ---------------------------------------------------------------------------

#[test]
fn offline_policy_disabled_by_default() {
    let policy = OfflinePolicy::default();
    assert!(
        !policy.is_allowed(),
        "offline mode must be disabled by default"
    );
    assert!(
        policy.require_allowed().is_err(),
        "offline must not be permitted by default"
    );
}

#[test]
fn offline_policy_disabled_helper() {
    let policy = OfflinePolicy::disabled();
    assert!(!policy.is_allowed());
}

#[test]
fn offline_policy_can_be_enabled_by_platform() {
    let policy = OfflinePolicy {
        allowed: true,
        notice: Some("Admin-approved".to_string()),
    };
    assert!(policy.is_allowed());
    assert!(policy.require_allowed().is_ok());
}

// ---------------------------------------------------------------------------
// Issue visibility: IssueSummary carries only safe fields
// ---------------------------------------------------------------------------

#[test]
fn issue_summary_serialization_round_trip() {
    let issue = IssueSummary {
        id: "repo-42".to_string(),
        name: "Feature Branch".to_string(),
        description: Some("Implement X".to_string()),
        created_at: 1_700_000_000,
    };
    let json = common_json::to_string(&issue).unwrap();
    let back: IssueSummary = common_json::from_json_str(&json).unwrap();
    assert_eq!(back.id, "repo-42");
    assert_eq!(back.name, "Feature Branch");
    assert_eq!(back.created_at, 1_700_000_000);
}
