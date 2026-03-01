use crate::config::enforcement_mode::EnforcementMode;
use crate::config::path_classification::PathClassification;
use crate::config::severity::Severity;
use crate::protocol::request::{Request, RequestPayload};
use crate::protocol::response::{Response, ResponsePayload};
use crate::report::report::{Report, ReportSummary};
use crate::report::report_hash::ReportHash;
use crate::report::violation::Violation;
use crate::report::violation_code::ViolationCode;
use crate::rules::rule_id::RuleId;
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

    let mut violations = Vec::new();
    let product_name = product_dir
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown_product".to_string());

    let backend = product_dir.join("backend");
    let ui = product_dir.join("ui");
    if !(backend.is_dir() && ui.is_dir()) {
        violations.push(make_violation(
            RuleId::Structure,
            ViolationCode::StructMissingBackendOrUi,
            scope,
            product_dir,
            "product must include backend/ and ui/ crates",
            mode,
            true,
        ));
    }

    let root_cargo = product_dir.join("Cargo.toml");
    if root_cargo.exists() {
        let txt = std::fs::read_to_string(&root_cargo).unwrap_or_default();
        if !(txt.contains("\"backend\"") && txt.contains("\"ui\"")) {
            violations.push(make_violation(
                RuleId::Structure,
                ViolationCode::StructInvalidWorkspaceMembers,
                scope,
                &root_cargo,
                "workspace members must contain exactly backend and ui",
                mode,
                true,
            ));
        }
    }

    if !product_dir.join("README.md").exists() {
        violations.push(make_violation(
            RuleId::Structure,
            ViolationCode::StructMissingReadme,
            scope,
            product_dir,
            "product root must contain README.md",
            mode,
            true,
        ));
    }

    for bad in ["tooling", "tools", "devtools", "utils"] {
        if product_dir.join(bad).exists() {
            violations.push(make_violation(
                RuleId::Structure,
                ViolationCode::StructForbiddenFolderName,
                scope,
                &product_dir.join(bad),
                "forbidden folder name detected",
                mode,
                true,
            ));
        }
    }

    for (crate_name, crate_path, expected_name) in [
        (
            "backend",
            backend.as_path(),
            format!("{product_name}_backend"),
        ),
        ("ui", ui.as_path(), format!("{product_name}_ui")),
    ] {
        let cargo = crate_path.join("Cargo.toml");
        let main_rs = crate_path.join("src/main.rs");
        if !main_rs.exists() {
            violations.push(make_violation(
                RuleId::Crate,
                ViolationCode::CrateMissingMain,
                scope,
                &main_rs,
                "crate must contain src/main.rs",
                mode,
                true,
            ));
        }

        if cargo.exists() {
            let txt = std::fs::read_to_string(&cargo).unwrap_or_default();
            if txt.contains("[lib]") {
                violations.push(make_violation(
                    RuleId::Crate,
                    ViolationCode::CrateNotBinOnly,
                    scope,
                    &cargo,
                    "crate must be bin-only (no [lib])",
                    mode,
                    true,
                ));
            }

            if !txt.contains(&format!("name = \"{expected_name}\"")) {
                violations.push(make_violation(
                    RuleId::Naming,
                    ViolationCode::NameCrateMismatch,
                    scope,
                    &cargo,
                    &format!("{crate_name} crate name must be {expected_name}"),
                    mode,
                    true,
                ));
            }
        }
    }

    if ui.exists() {
        let pattern = format!("use {product_name}_backend::");
        let rs_files = gather_rs_files(&ui);
        for file in rs_files {
            let txt = std::fs::read_to_string(&file).unwrap_or_default();
            if txt.contains(&pattern) {
                violations.push(make_violation(
                    RuleId::Layering,
                    ViolationCode::LayerUiImportsBackend,
                    scope,
                    &file,
                    "ui must not import backend internals",
                    mode,
                    true,
                ));
            }
        }
    }

    if backend.exists() {
        let rs_files = gather_rs_files(&backend);
        for file in rs_files {
            let txt = std::fs::read_to_string(&file).unwrap_or_default();
            if txt.contains("SystemTime") || txt.contains("Instant") {
                violations.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetWallClockUsage,
                    scope,
                    &file,
                    "wall-clock API usage is forbidden",
                    mode,
                    true,
                ));
            }
            if txt.contains("chrono") {
                violations.push(make_violation(
                    RuleId::Determinism,
                    ViolationCode::DetForbiddenTimeDep,
                    scope,
                    &file,
                    "chrono usage is forbidden in backend core",
                    mode,
                    true,
                ));
            }
        }
    }

    violations
}

fn gather_rs_files(root: &Path) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let read_dir = match std::fs::read_dir(dir) {
            Ok(rd) => rd,
            Err(_) => continue,
        };
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn make_violation(
    rule_id: RuleId,
    code: ViolationCode,
    scope: PathClassification,
    path: &Path,
    message: &str,
    mode: EnforcementMode,
    default_blocking: bool,
) -> Violation {
    let mut severity = if default_blocking {
        Severity::Error
    } else {
        Severity::Warning
    };

    if mode == EnforcementMode::Relaxed || scope == PathClassification::Unstable {
        severity = Severity::Warning;
    }

    Violation {
        rule_id,
        violation_code: code,
        severity,
        scope,
        path: path.to_string_lossy().to_string(),
        message: message.to_string(),
        line: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::request::{Request, RequestPayload};
    use crate::protocol::response::ResponsePayload;
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
                let ja = serde_json::to_string(&a).expect("serialize report a");
                let jb = serde_json::to_string(&b).expect("serialize report b");
                assert_eq!(ja, jb);
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
}
