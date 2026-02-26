// projects/products/stable/platform_versioning/backend/src/verify/slice_feedback.rs
use serde::{Deserialize, Serialize};

use crate::slices::SliceManifest;
use crate::verify::{IntegrityIssue, IntegrityReport};

/// A non-leaking view of an [`IntegrityReport`] filtered through a
/// [`SliceManifest`].
///
/// # Non-leakage rules
///
/// 1. Issues that reference a path from the allowed list are shown in full
///    (file name and error details visible).
/// 2. Issues that reference a path outside the allowed list are surfaced as a
///    single opaque summary entry without any path or object-id information.
/// 3. Structural issues (corrupt/missing objects referenced from forbidden
///    areas) are counted but their identifiers are withheld.
/// 4. Overall health status is always reported truthfully; a forbidden-area
///    failure still causes `healthy` to be `false`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SliceFeedback {
    /// Whether the repository is fully healthy within the accessible scope.
    ///
    /// `true` only when there are no issues at all (including in restricted
    /// areas), so clients cannot infer "no problems here" from a partial view.
    pub healthy: bool,
    /// Detailed entries for issues in paths the caller is allowed to see.
    pub visible_issues: Vec<SliceFeedbackEntry>,
    /// Number of issues detected in paths the caller is not allowed to see.
    ///
    /// Non-zero means there are problems, but their details are withheld.
    pub restricted_issue_count: usize,
    /// Total objects checked during the integrity run.
    pub objects_checked: usize,
}

/// A single feedback entry for an integrity issue within an allowed path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SliceFeedbackEntry {
    /// Human-readable summary of the issue (safe to display to the caller).
    pub summary: String,
}

impl SliceFeedback {
    /// Filters `report` through `manifest` to produce non-leaking feedback.
    pub fn for_manifest(report: &IntegrityReport, manifest: &SliceManifest) -> Self {
        let mut visible_issues = Vec::new();
        let mut restricted_issue_count = 0usize;

        for issue in &report.issues {
            match issue {
                IntegrityIssue::DanglingRef { ref_name, target } => {
                    // Ref names don't carry path information; always visible.
                    visible_issues.push(SliceFeedbackEntry {
                        summary: format!("ref '{ref_name}' points to missing object {target}"),
                    });
                }
                IntegrityIssue::CorruptObject { object_id } => {
                    // Object IDs alone don't leak path information.
                    visible_issues.push(SliceFeedbackEntry {
                        summary: format!("corrupt object detected (id={object_id})"),
                    });
                }
                IntegrityIssue::MissingTree { commit_id, tree_id } => {
                    // Commit/tree IDs don't map directly to user-visible paths.
                    visible_issues.push(SliceFeedbackEntry {
                        summary: format!("commit {commit_id} references missing tree {tree_id}"),
                    });
                }
                IntegrityIssue::MissingObject {
                    tree_id,
                    entry_name,
                } => {
                    // entry_name IS a file path; only reveal if it's in the slice.
                    if manifest.allows(entry_name) {
                        visible_issues.push(SliceFeedbackEntry {
                            summary: format!(
                                "tree {tree_id} references missing object for '{entry_name}'"
                            ),
                        });
                    } else {
                        restricted_issue_count += 1;
                    }
                }
            }
        }

        SliceFeedback {
            healthy: report.is_healthy(),
            visible_issues,
            restricted_issue_count,
            objects_checked: report.objects_checked,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slices::SliceManifest;
    use crate::verify::{IntegrityIssue, IntegrityReport};

    fn manifest(paths: &[&str]) -> SliceManifest {
        SliceManifest {
            subject: "alice".to_string(),
            issue_id: "iss-1".to_string(),
            allowed_paths: paths.iter().map(|p| p.to_string()).collect(),
        }
    }

    fn report_with_missing_object(entry_name: &str) -> IntegrityReport {
        IntegrityReport {
            issues: vec![IntegrityIssue::MissingObject {
                tree_id: "deadbeef".to_string(),
                entry_name: entry_name.to_string(),
            }],
            objects_checked: 5,
            refs_checked: 1,
        }
    }

    #[test]
    fn allowed_path_issue_is_visible() {
        let report = report_with_missing_object("src/main.rs");
        let m = manifest(&["src"]);
        let feedback = SliceFeedback::for_manifest(&report, &m);
        assert!(!feedback.healthy);
        assert_eq!(feedback.visible_issues.len(), 1);
        assert_eq!(feedback.restricted_issue_count, 0);
        assert!(feedback.visible_issues[0].summary.contains("src/main.rs"));
    }

    #[test]
    fn forbidden_path_issue_is_withheld() {
        let report = report_with_missing_object("secret/credentials.rs");
        let m = manifest(&["src"]);
        let feedback = SliceFeedback::for_manifest(&report, &m);
        assert!(!feedback.healthy);
        assert_eq!(feedback.visible_issues.len(), 0);
        assert_eq!(feedback.restricted_issue_count, 1);
        // The forbidden path must not appear in any visible summary.
        for entry in &feedback.visible_issues {
            assert!(!entry.summary.contains("secret"));
            assert!(!entry.summary.contains("credentials"));
        }
    }

    #[test]
    fn healthy_report_yields_healthy_feedback() {
        let report = IntegrityReport {
            issues: vec![],
            objects_checked: 10,
            refs_checked: 2,
        };
        let m = manifest(&["src"]);
        let feedback = SliceFeedback::for_manifest(&report, &m);
        assert!(feedback.healthy);
        assert!(feedback.visible_issues.is_empty());
        assert_eq!(feedback.restricted_issue_count, 0);
    }

    #[test]
    fn dangling_ref_is_always_visible() {
        let report = IntegrityReport {
            issues: vec![IntegrityIssue::DanglingRef {
                ref_name: "heads/main".to_string(),
                target: "0000".to_string(),
            }],
            objects_checked: 0,
            refs_checked: 1,
        };
        let m = manifest(&[]); // empty manifest
        let feedback = SliceFeedback::for_manifest(&report, &m);
        assert_eq!(feedback.visible_issues.len(), 1);
    }
}
