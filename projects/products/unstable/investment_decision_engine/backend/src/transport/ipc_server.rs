use std::io::{BufRead, Write};

use crate::config::{EngineConfig, FeatureGateConfig};
use crate::transport::{Request, Response, ResponsePayload};

pub struct IpcServer {
    config: EngineConfig,
    gate: FeatureGateConfig,
}

impl IpcServer {
    pub fn new(config: EngineConfig) -> Self {
        let gate = FeatureGateConfig::from_config(&config);
        Self { config, gate }
    }

    pub fn handle_request(&self, request: &Request) -> Response {
        let payload = match &request.payload {
            crate::transport::request::RequestPayload::AnalyzeAsset {
                asset_json,
                market_json,
            } => self.handle_analyze_asset(asset_json, market_json),
            crate::transport::request::RequestPayload::AnalyzePortfolio {
                portfolio_json,
                market_json,
            } => self.handle_analyze_portfolio(portfolio_json, market_json),
            crate::transport::request::RequestPayload::ReplayDecision { replay_json } => {
                self.handle_replay(replay_json)
            }
            crate::transport::request::RequestPayload::ReadJournal { journal_path } => {
                self.handle_read_journal(journal_path)
            }
            crate::transport::request::RequestPayload::EvaluateScenario {
                scenario_json,
                asset_json,
            } => self.handle_evaluate_scenario(scenario_json, asset_json),
            crate::transport::request::RequestPayload::Shutdown => ResponsePayload::Error {
                message: "shutdown acknowledged".to_string(),
            },
        };

        Response {
            id: request.id.clone(),
            payload,
        }
    }

    pub fn run_stdio(&self) {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let mut stdout_lock = stdout.lock();

        for line in stdin.lock().lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => break,
            };
            if line.trim().is_empty() {
                continue;
            }

            let request: Request = match common_json::from_str(&line) {
                Ok(r) => r,
                Err(e) => {
                    let error = Response::error(None, format!("invalid request: {e}"));
                    if let Ok(json) = common_json::to_json_string(&error) {
                        let _ = writeln!(stdout_lock, "{json}");
                        let _ = stdout_lock.flush();
                    }
                    continue;
                }
            };

            let response = self.handle_request(&request);
            if let Ok(json) = common_json::to_json_string(&response) {
                let _ = writeln!(stdout_lock, "{json}");
                let _ = stdout_lock.flush();
            }
        }
    }

    fn handle_analyze_asset(&self, asset_json: &str, market_json: &str) -> ResponsePayload {
        let asset = match common_json::from_str(asset_json) {
            Ok(a) => a,
            Err(e) => {
                return ResponsePayload::Error {
                    message: format!("invalid asset: {e}"),
                };
            }
        };
        let market = match common_json::from_str(market_json) {
            Ok(m) => m,
            Err(e) => {
                return ResponsePayload::Error {
                    message: format!("invalid market: {e}"),
                };
            }
        };

        let report =
            crate::report::AssetReport::generate(&asset, &market, &self.config, &self.gate);
        ResponsePayload::AssetReport { report }
    }

    fn handle_analyze_portfolio(&self, portfolio_json: &str, market_json: &str) -> ResponsePayload {
        let portfolio = match common_json::from_str(portfolio_json) {
            Ok(p) => p,
            Err(e) => {
                return ResponsePayload::Error {
                    message: format!("invalid portfolio: {e}"),
                };
            }
        };
        let market = match common_json::from_str(market_json) {
            Ok(m) => m,
            Err(e) => {
                return ResponsePayload::Error {
                    message: format!("invalid market: {e}"),
                };
            }
        };

        let report =
            crate::report::PortfolioReport::generate(&portfolio, &market, &self.config, &self.gate);
        ResponsePayload::PortfolioReport { report }
    }

    fn handle_replay(&self, replay_json: &str) -> ResponsePayload {
        let replay_file = match common_json::from_str(replay_json) {
            Ok(r) => r,
            Err(e) => {
                return ResponsePayload::Error {
                    message: format!("invalid replay: {e}"),
                };
            }
        };

        let summary = crate::replay::ReplayEngine::execute(&replay_file, &self.config, &self.gate);
        let report = crate::report::DecisionReport::new("replay", "replay", summary);
        ResponsePayload::ReplayReport { report }
    }

    fn handle_read_journal(&self, journal_path: &str) -> ResponsePayload {
        let content = match std::fs::read_to_string(journal_path) {
            Ok(c) => c,
            Err(e) => {
                return ResponsePayload::Error {
                    message: format!("read journal failed: {e}"),
                };
            }
        };
        let entries = match common_json::from_str(&content) {
            Ok(e) => e,
            Err(e) => {
                return ResponsePayload::Error {
                    message: format!("parse journal failed: {e}"),
                };
            }
        };
        ResponsePayload::JournalEntries { entries }
    }

    fn handle_evaluate_scenario(&self, scenario_json: &str, asset_json: &str) -> ResponsePayload {
        let scenario = match common_json::from_str(scenario_json) {
            Ok(s) => s,
            Err(e) => {
                return ResponsePayload::Error {
                    message: format!("invalid scenario: {e}"),
                };
            }
        };
        let asset = match common_json::from_str(asset_json) {
            Ok(a) => a,
            Err(e) => {
                return ResponsePayload::Error {
                    message: format!("invalid asset: {e}"),
                };
            }
        };

        let _result = crate::scenario::ScenarioEngine::evaluate(&scenario, &asset, &self.config);
        let summary = crate::decision::DecisionSummary::gated();
        let report = crate::report::DecisionReport::new("scenario", "scenario", summary);
        ResponsePayload::DecisionReport { report }
    }
}
