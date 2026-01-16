// projects/products/code_agent_sandbox/src/score.rs
use common_json::JsonAccess;
use serde::Serialize;

use crate::actions::ActionResult;

/// Small "symbolic" scoring layer: tries to estimate code quality signals
/// from outputs (cargo/clippy) and from file contents in results (if present).
///
/// This is NOT perfect. It's intentionally simple and deterministic.
/// The goal is to shape rewards and guide the agent.
#[derive(Debug, Clone)]
pub struct ScoreConfig {
    pub penalize_unwrap_outside_tests: bool,
    pub unwrap_penalty: i32,

    pub penalize_panic_outside_tests: bool,
    pub panic_penalty: i32,

    pub penalize_dbg_outside_tests: bool,
    pub dbg_penalty: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreSummary {
    pub score: i32,
    pub notes: Vec<String>,
    pub cargo_ok: bool,
    pub cargo_failures: usize,
}

impl ScoreSummary {
    pub fn from_results(results: &[ActionResult], cfg: ScoreConfig) -> Self {
        let mut score = 0i32;
        let mut notes = Vec::new();

        let mut cargo_ok = true;
        let mut cargo_failures = 0usize;

        for r in results {
            // Cargo results
            if r.kind.starts_with("Cargo") {
                if !r.ok {
                    cargo_ok = false;
                    cargo_failures += 1;
                    score -= 100;
                    notes.push(format!("Cargo failure: {}", r.kind));
                } else {
                    score += 50;
                }
            }

            // If a ReadFile result contains contents, scan it for patterns.
            if let Some(data) = &r.data
                && let Some(contents) = data.get_field("contents").ok().and_then(|v| v.as_str())
            {
                let path = data
                    .get_field("path")
                    .ok()
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let is_tests = path.contains("/tests/")
                    || path.starts_with("tests/")
                    || path.contains("#[test]");

                if cfg.penalize_unwrap_outside_tests && !is_tests && contents.contains(".unwrap()")
                {
                    score -= cfg.unwrap_penalty;
                    notes.push(format!(
                        "Penalized unwrap outside tests in {path} (-{})",
                        cfg.unwrap_penalty
                    ));
                }
                if cfg.penalize_panic_outside_tests && !is_tests && contents.contains("panic!(") {
                    score -= cfg.panic_penalty;
                    notes.push(format!(
                        "Penalized panic outside tests in {path} (-{})",
                        cfg.panic_penalty
                    ));
                }
                if cfg.penalize_dbg_outside_tests && !is_tests && contents.contains("dbg!(") {
                    score -= cfg.dbg_penalty;
                    notes.push(format!(
                        "Penalized dbg outside tests in {path} (-{})",
                        cfg.dbg_penalty
                    ));
                }
            }

            // If it contains stderr from cargo/clippy, you can add more heuristics later.
        }

        Self {
            score,
            notes,
            cargo_ok,
            cargo_failures,
        }
    }
}

impl Default for ScoreSummary {
    fn default() -> Self {
        Self {
            score: 0,
            notes: Vec::new(),
            cargo_ok: true,
            cargo_failures: 0,
        }
    }
}
