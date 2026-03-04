use crate::canon::canonical_json_checker;
use crate::diagnostics::backend_error::BackendError;
use crate::protocol::report_data::ReportData;
use crate::protocol::request::Request;
use crate::protocol::response::Response;
use crate::scan::forbidden_finder;
use crate::scan::scan_config::ScanConfig;
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
                match self.handle_scan(&root) {
                    Ok(()) => Response::Ok,
                    Err(err) => Response::Error {
                        message: err.to_string(),
                    },
                }
            }
            Request::CheckCanonicalJson { path } => {
                info!("CheckCanonicalJson: {}", path);
                match self.handle_canonical(&path) {
                    Ok(()) => Response::Ok,
                    Err(err) => Response::Error {
                        message: err.to_string(),
                    },
                }
            }
            Request::RunStabilityHarness { cmd, runs } => {
                info!("RunStabilityHarness: {} x{}", cmd, runs);
                match self.handle_stability(&cmd, runs) {
                    Ok(()) => Response::Ok,
                    Err(err) => Response::Error {
                        message: err.to_string(),
                    },
                }
            }
            Request::GetReport => Response::Report {
                data: ReportData {
                    scan_findings: self.scan_findings.clone(),
                    canon_issues: self.canon_issues.clone(),
                    stability: self.stability.clone(),
                },
            },
        }
    }

    fn handle_scan(&mut self, root: &str) -> Result<(), BackendError> {
        let config = ScanConfig::default();
        self.scan_findings = forbidden_finder::find_forbidden(root, &config)
            .map_err(|err| BackendError::Scan(err.to_string()))?;
        Ok(())
    }

    fn handle_canonical(&mut self, path: &str) -> Result<(), BackendError> {
        self.canon_issues = canonical_json_checker::check_file(path)
            .map_err(|err| BackendError::Canon(err.to_string()))?;
        Ok(())
    }

    fn handle_stability(&mut self, cmd: &str, runs: u32) -> Result<(), BackendError> {
        self.stability = Some(
            stability_runner::run_stability(cmd, runs)
                .map_err(|err| BackendError::Stability(err.to_string()))?,
        );
        Ok(())
    }
}
