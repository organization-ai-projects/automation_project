//! projects/products/stable/platform_ide/backend/src/verification/result_view.rs
use crate::slices::SliceManifest;
use crate::verification::{FindingSeverity, RawFinding, VerificationFinding};
use serde::{Deserialize, Serialize};

/// A safe, user-facing view of a verification run result.
///
/// This type applies the slice manifest to scrub any forbidden file paths
/// from the raw platform response before presenting findings to the user.
/// Findings that reference forbidden areas are replaced with a generic
/// contract-level error message with no path details.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultView {
    /// Whether the verification run passed overall.
    pub healthy: bool,
    /// The filtered findings visible to the current user.
    pub findings: Vec<VerificationFinding>,
}

impl ResultView {
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

pub type VerificationResultView = ResultView;
