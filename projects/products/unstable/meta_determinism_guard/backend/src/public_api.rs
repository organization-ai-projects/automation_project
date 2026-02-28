use crate::protocol::request::Request;
use crate::protocol::response::{Response, ReportData};
use crate::scan::scan_config::ScanConfig;
use crate::scan::forbidden_finder;
use crate::canon::canonical_json_checker;
use crate::stability::stability_runner;
use tracing::info;

pub struct BackendState {
    scan_findings: Vec<String>,
    canon_issues: Vec<String>,
    stability: Option<crate::stability::stability_report::StabilityReport>,
}

impl BackendState {
    pub fn new() -> Self {
        Self {
            scan_findings: Vec::new(),
            canon_issues: Vec::new(),
            stability: None,
        }
    }

    pub fn handle(&mut self, request: Request) -> Response {
        match request {
            Request::ScanForbidden { root } => {
                info!("ScanForbidden: {}", root);
                let config = ScanConfig::default();
                match forbidden_finder::find_forbidden(&root, &config) {
                    Ok(findings) => {
                        self.scan_findings = findings;
                        Response::Ok
                    }
                    Err(e) => Response::Error { message: e.to_string() },
                }
            }
            Request::CheckCanonicalJson { path } => {
                info!("CheckCanonicalJson: {}", path);
                match canonical_json_checker::check_file(&path) {
                    Ok(issues) => {
                        self.canon_issues = issues;
                        Response::Ok
                    }
                    Err(e) => Response::Error { message: e.to_string() },
                }
            }
            Request::RunStabilityHarness { cmd, runs } => {
                info!("RunStabilityHarness: {} x{}", cmd, runs);
                match stability_runner::run_stability(&cmd, runs) {
                    Ok(report) => {
                        self.stability = Some(report);
                        Response::Ok
                    }
                    Err(e) => Response::Error { message: e.to_string() },
                }
            }
            Request::GetReport => {
                Response::Report {
                    data: ReportData {
                        scan_findings: self.scan_findings.clone(),
                        canon_issues: self.canon_issues.clone(),
                        stability: self.stability.clone(),
                    },
                }
            }
        }
    }
}
