// projects/products/stable/platform_ide/backend/src/verification/result_view.rs
use serde::{Deserialize, Serialize};

use crate::slices::SliceManifest;

/// The severity level of a verification finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// An error that must be resolved before the change can be accepted.
    Error,
    /// A warning that should be reviewed.
    Warning,
    /// Informational note.
    Info,
}

/// A single finding from a verification run, pre-filtered for display.
///
/// Findings that reference forbidden paths are demoted to a generic
/// contract-level error with no path details exposed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationFinding {
    /// Severity of the finding.
    pub severity: FindingSeverity,
    /// A safe, human-readable summary of the finding.
    pub summary: String,
    /// The allowed file path this finding relates to, if any.
    /// `None` if the finding is not file-specific or if the path is forbidden.
    pub path: Option<String>,
    /// The line number within `path`, if relevant and path is allowed.
    pub line: Option<u32>,
}

/// A safe, user-facing view of a verification run result.
///
/// This type applies the slice manifest to scrub any forbidden file paths
/// from the raw platform response before presenting findings to the user.
/// Findings that reference forbidden areas are replaced with a generic
/// contract-level error message with no path details.
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResultView {
    /// Whether the verification run passed overall.
    pub healthy: bool,
    /// The filtered findings visible to the current user.
    pub findings: Vec<VerificationFinding>,
}

impl VerificationResultView {
    /// Creates an empty (healthy) result view.
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            findings: Vec::new(),
        }
    }

    /// Creates a result view from raw platform findings, filtering out any
    /// findings that reference paths not in `manifest`.
    pub fn from_raw(
        healthy: bool,
        raw_findings: impl IntoIterator<Item = RawFinding>,
        manifest: &SliceManifest,
    ) -> Self {
        let findings = raw_findings
            .into_iter()
            .filter_map(|f| Self::filter_finding(f, manifest))
            .collect();

        Self { healthy, findings }
    }

    fn filter_finding(raw: RawFinding, manifest: &SliceManifest) -> Option<VerificationFinding> {
        match raw.path.as_deref() {
            None => Some(VerificationFinding {
                severity: raw.severity,
                summary: raw.summary,
                path: None,
                line: None,
            }),
            Some(p) => {
                if manifest.allow(p).is_ok() {
                    Some(VerificationFinding {
                        severity: raw.severity,
                        summary: raw.summary,
                        path: Some(p.to_string()),
                        line: raw.line,
                    })
                } else {
                    match raw.severity {
                        FindingSeverity::Info => None,
                        _ => Some(VerificationFinding {
                            severity: FindingSeverity::Error,
                            summary: "A contract-level error was detected in a restricted area."
                                .to_string(),
                            path: None,
                            line: None,
                        }),
                    }
                }
            }
        }
    }
}

/// A raw finding as received from the platform API before filtering.
#[derive(Debug, Serialize, Deserialize)]
pub struct RawFinding {
    pub severity: FindingSeverity,
    pub summary: String,
    pub path: Option<String>,
    pub line: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::slices::SliceManifest;

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
        let view = VerificationResultView::from_raw(
            false,
            [raw(
                FindingSeverity::Error,
                "syntax error",
                Some("src/main.rs"),
            )],
            &manifest(),
        );
        assert_eq!(view.findings.len(), 1);
        let f = &view.findings[0];
        assert_eq!(f.path.as_deref(), Some("src/main.rs"));
        assert!(f.line.is_some());
        assert_eq!(f.summary, "syntax error");
    }

    #[test]
    fn forbidden_path_error_finding_becomes_generic() {
        let view = VerificationResultView::from_raw(
            false,
            [raw(
                FindingSeverity::Error,
                "secret error",
                Some("forbidden/internal.rs"),
            )],
            &manifest(),
        );
        assert_eq!(view.findings.len(), 1);
        let f = &view.findings[0];
        assert!(f.path.is_none(), "forbidden path must not appear in output");
        assert!(f.line.is_none(), "line must not appear for forbidden path");
        assert!(!f.summary.contains("forbidden"), "summary leaks path info");
        assert!(
            !f.summary.contains("internal.rs"),
            "summary leaks path info"
        );
    }

    #[test]
    fn forbidden_path_info_finding_is_suppressed() {
        let view = VerificationResultView::from_raw(
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
        let view = VerificationResultView::from_raw(
            false,
            [raw(FindingSeverity::Warning, "global warning", None)],
            &manifest(),
        );
        assert_eq!(view.findings.len(), 1);
        assert!(view.findings[0].path.is_none());
    }

    #[test]
    fn healthy_result_has_no_findings() {
        let view = VerificationResultView::healthy();
        assert!(view.healthy);
        assert!(view.findings.is_empty());
    }
}
