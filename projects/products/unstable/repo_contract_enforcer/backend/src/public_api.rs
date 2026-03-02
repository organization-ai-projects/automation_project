use crate::config::enforcement_mode::EnforcementMode;
use crate::config::path_classification::PathClassification;
use crate::config::severity::Severity;
use crate::protocol::message::{Request, RequestPayload};
use crate::protocol::response::{Response, ResponsePayload};
use crate::reports::report::{Report, ReportSummary};
use crate::reports::report_hash::ReportHash;
use crate::reports::violation::Violation;
use crate::rules::rule_engine::RuleEngine;
use crate::scan::workspace_scanner::WorkspaceScanner;
use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
pub struct BackendState {
    reports: HashMap<String, Report>,
    should_shutdown: bool,
}

impl BackendState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn should_shutdown(&self) -> bool {
        self.should_shutdown
    }

    pub fn handle(&mut self, request: Request) -> Response {
        let id = request.id.clone();

        let payload = match request.payload {
            RequestPayload::Ping => ResponsePayload::Ok,
            RequestPayload::Shutdown => {
                self.should_shutdown = true;
                ResponsePayload::Ok
            }
            RequestPayload::GetReport { report_hash } => match self.reports.get(&report_hash) {
                Some(report) => ResponsePayload::Report {
                    report_json: report.clone(),
                    report_hash: report.report_hash.clone(),
                    summary: report.summary.clone(),
                },
                None => ResponsePayload::Error {
                    code: "REPORT_NOT_FOUND".to_string(),
                    message: "report hash not found".to_string(),
                    details: Some(report_hash),
                },
            },
            RequestPayload::CheckRepo { root_path, mode } => {
                self.build_repo_report(Path::new(&root_path), mode)
            }
            RequestPayload::CheckProduct { product_path, mode } => {
                self.build_product_report(Path::new(&product_path), mode)
            }
        };

        Response { id, payload }
    }

    fn build_repo_report(&mut self, root: &Path, mode: EnforcementMode) -> ResponsePayload {
        let mut violations = Vec::new();
        for product in WorkspaceScanner::discover_products(root) {
            violations.extend(check_product(&product, mode));
        }

        finalize_report(root, mode, &mut self.reports, violations)
    }

    fn build_product_report(&mut self, product: &Path, mode: EnforcementMode) -> ResponsePayload {
        let violations = check_product(product, mode);
        finalize_report(product, mode, &mut self.reports, violations)
    }
}

fn finalize_report(
    root: &Path,
    mode: EnforcementMode,
    cache: &mut HashMap<String, Report>,
    mut violations: Vec<Violation>,
) -> ResponsePayload {
    violations.sort_by(|a, b| {
        a.scope
            .cmp(&b.scope)
            .then(a.path.cmp(&b.path))
            .then(a.rule_id.cmp(&b.rule_id))
            .then(a.line.cmp(&b.line))
    });

    let summary = summarize(&violations);
    let mut report = Report {
        repository_root: root.to_string_lossy().to_string(),
        mode,
        violations,
        summary,
        report_hash: String::new(),
    };

    let hash = match ReportHash::compute(&report) {
        Ok(h) => h,
        Err(err) => {
            return ResponsePayload::Error {
                code: "REPORT_HASH_FAILED".to_string(),
                message: "failed to hash report".to_string(),
                details: Some(err.to_string()),
            };
        }
    };
    report.report_hash = hash.clone();
    cache.insert(hash.clone(), report.clone());
    let _canonical_json = crate::reports::json_report_codec::JsonReportCodec::to_json(&report);

    ResponsePayload::Report {
        report_json: report,
        report_hash: hash,
        summary: cache
            .values()
            .last()
            .map(|r| r.summary.clone())
            .unwrap_or(ReportSummary {
                stable_error_count: 0,
                stable_warning_count: 0,
                unstable_error_count: 0,
                unstable_warning_count: 0,
            }),
    }
}

fn summarize(violations: &[Violation]) -> ReportSummary {
    let mut summary = ReportSummary {
        stable_error_count: 0,
        stable_warning_count: 0,
        unstable_error_count: 0,
        unstable_warning_count: 0,
    };

    for v in violations {
        match (v.scope, v.severity) {
            (PathClassification::Stable, Severity::Error) => summary.stable_error_count += 1,
            (PathClassification::Stable, Severity::Warning) => summary.stable_warning_count += 1,
            (PathClassification::Unstable, Severity::Error) => summary.unstable_error_count += 1,
            (PathClassification::Unstable, Severity::Warning) => {
                summary.unstable_warning_count += 1
            }
            (PathClassification::Other, _) => {}
        }
    }

    summary
}

fn check_product(product_dir: &Path, mode: EnforcementMode) -> Vec<Violation> {
    let scope = PathClassification::from_product_path(product_dir);
    if scope == PathClassification::Other {
        return Vec::new();
    }
    RuleEngine::evaluate_product(product_dir, scope, mode)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::request::{Request, RequestPayload};
    use crate::protocol::response::ResponsePayload;
    use crate::reports::report::Report;
    use crate::reports::report_hash::ReportHash;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root() -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time before epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("repo_contract_enforcer_test_{stamp}"));
        fs::create_dir_all(root.join("projects/products/unstable/bad_product"))
            .expect("create test directory tree");
        root
    }

    #[test]
    fn check_repo_is_deterministic_for_same_tree() {
        let root = temp_root();
        let mut state = BackendState::new();
        let req = Request {
            id: Some("t1".to_string()),
            payload: RequestPayload::CheckRepo {
                root_path: root.to_string_lossy().to_string(),
                mode: EnforcementMode::Auto,
            },
        };

        let first = state.handle(req.clone());
        let second = state.handle(req);

        match (first.payload, second.payload) {
            (
                ResponsePayload::Report {
                    report_json: a,
                    report_hash: ha,
                    ..
                },
                ResponsePayload::Report {
                    report_json: b,
                    report_hash: hb,
                    ..
                },
            ) => {
                assert_eq!(ha, hb);
                assert_eq!(a, b);
            }
            _ => panic!("expected two report payloads"),
        }
    }

    #[test]
    fn unstable_scope_downgrades_errors_to_warnings() {
        let root = temp_root();
        let mut state = BackendState::new();
        let req = Request {
            id: Some("t2".to_string()),
            payload: RequestPayload::CheckProduct {
                product_path: root
                    .join("projects/products/unstable/bad_product")
                    .to_string_lossy()
                    .to_string(),
                mode: EnforcementMode::Auto,
            },
        };

        let response = state.handle(req);
        let report = match response.payload {
            ResponsePayload::Report { report_json, .. } => report_json,
            _ => panic!("expected report payload"),
        };

        assert!(
            report
                .violations
                .iter()
                .all(|v| v.severity == Severity::Warning)
        );
    }

    fn fixture_root(name: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/repos")
            .join(name)
    }

    fn golden_path(name: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/golden")
            .join(name)
    }

    fn normalized_report(mut report: Report, root: &std::path::Path) -> Report {
        let root_marker = root.to_string_lossy().to_string();
        report.repository_root = report
            .repository_root
            .replace(&root_marker, "<FIXTURE_ROOT>");
        for violation in &mut report.violations {
            violation.path = violation.path.replace(&root_marker, "<FIXTURE_ROOT>");
        }
        report.report_hash = ReportHash::compute(&report).expect("re-hash normalized report");
        report
    }

    fn run_check_repo_report(root: &std::path::Path, mode: EnforcementMode) -> Report {
        let mut state = BackendState::new();
        let req = Request {
            id: Some("fixture".to_string()),
            payload: RequestPayload::CheckRepo {
                root_path: root.to_string_lossy().to_string(),
                mode,
            },
        };
        let response = state.handle(req);
        match response.payload {
            ResponsePayload::Report { report_json, .. } => report_json,
            _ => panic!("expected report response"),
        }
    }

    fn assert_matches_golden(golden_file: &str, report: &Report) {
        let golden = golden_path(golden_file);
        if std::env::var("UPDATE_GOLDEN").as_deref() == Ok("1") {
            let json = common_json::to_string_pretty(report).expect("serialize report");
            fs::write(&golden, json).expect("write golden");
            return;
        }

        let expected: Report = common_json::from_json_str(
            &fs::read_to_string(&golden).expect("read expected golden report"),
        )
        .expect("parse expected golden report");
        assert_eq!(expected, *report);
    }

    #[test]
    fn fixture_valid_stable_matches_golden() {
        let root = fixture_root("valid_stable_product");
        let report = run_check_repo_report(&root, EnforcementMode::Auto);
        let normalized = normalized_report(report, &root);
        assert_matches_golden("expected_report_valid_stable.json", &normalized);
    }

    #[test]
    fn fixture_invalid_stable_third_crate_matches_golden() {
        let root = fixture_root("invalid_stable_third_crate");
        let report = run_check_repo_report(&root, EnforcementMode::Strict);
        let normalized = normalized_report(report, &root);
        assert_matches_golden(
            "expected_report_invalid_stable_third_crate.json",
            &normalized,
        );
    }

    #[test]
    fn fixture_invalid_unstable_third_crate_matches_golden() {
        let root = fixture_root("invalid_unstable_third_crate");
        let report = run_check_repo_report(&root, EnforcementMode::Auto);
        let normalized = normalized_report(report, &root);
        assert_matches_golden(
            "expected_report_invalid_unstable_third_crate.json",
            &normalized,
        );
        assert!(
            normalized
                .violations
                .iter()
                .all(|v| v.severity == Severity::Warning)
        );
    }
}
