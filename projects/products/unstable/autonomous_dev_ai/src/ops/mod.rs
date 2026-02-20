// projects/products/unstable/autonomous_dev_ai/src/ops/mod.rs

//! Observability, SLOs, run replay, and incident operations.
//!
//! Defines SLI/SLO contracts, run-replay tooling, and the operational runbook
//! for the autonomous_dev_ai agent.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── SLI/SLO ─────────────────────────────────────────────────────────────────

/// A Service-Level Indicator: a measurable signal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sli {
    pub name: String,
    pub description: String,
}

/// A Service-Level Objective: an SLI with a target threshold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slo {
    pub sli: Sli,
    /// Target threshold value.
    pub target: f64,
    /// Measurement window in seconds.
    pub window_secs: u64,
    /// When true the observed value must be >= target (e.g., success rates).
    /// When false the observed value must be <= target (e.g., latency budgets).
    pub higher_is_better: bool,
}

impl Slo {
    pub fn new(name: &str, description: &str, target: f64, window_secs: u64) -> Self {
        Self {
            sli: Sli {
                name: name.to_string(),
                description: description.to_string(),
            },
            target,
            window_secs,
            higher_is_better: true,
        }
    }

    pub fn lower_is_better(mut self) -> Self {
        self.higher_is_better = false;
        self
    }
}

/// Result of evaluating an SLO against observed data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SloEvaluation {
    pub slo_name: String,
    pub observed_ratio: f64,
    pub target: f64,
    pub met: bool,
}

/// Evaluates a set of SLOs against a snapshot of run metrics.
#[derive(Debug, Clone)]
pub struct SloEvaluator {
    slos: Vec<Slo>,
}

impl SloEvaluator {
    pub fn new(slos: Vec<Slo>) -> Self {
        Self { slos }
    }

    /// Default SLOs for the autonomous_dev_ai agent.
    pub fn default_slos() -> Vec<Slo> {
        vec![
            Slo::new(
                "run_success_rate",
                "Fraction of autonomous runs that reach Done state",
                0.95,
                3600,
            ),
            Slo::new(
                "policy_violation_rate",
                "Fraction of runs with zero policy violations",
                0.99,
                3600,
            ),
            Slo::new(
                "iteration_latency_p95_secs",
                "95th-percentile iteration duration must be under budget",
                60.0, // target: max 60 s per iteration
                3600,
            )
            .lower_is_better(),
            Slo::new(
                "test_pass_rate",
                "Fraction of runs where tests pass",
                0.98,
                3600,
            ),
            Slo::new(
                "recovery_time_secs",
                "Mean time to recover from a transient failure",
                120.0, // target: recover within 120 s
                3600,
            )
            .lower_is_better(),
        ]
    }

    /// Evaluate all SLOs.  `observations` maps SLO name → observed value.
    pub fn evaluate(&self, observations: &HashMap<String, f64>) -> Vec<SloEvaluation> {
        self.slos
            .iter()
            .map(|slo| {
                let observed = observations
                    .get(&slo.sli.name)
                    .copied()
                    .unwrap_or(0.0);
                let met = if slo.higher_is_better {
                    observed >= slo.target
                } else {
                    observed <= slo.target
                };
                SloEvaluation {
                    slo_name: slo.sli.name.clone(),
                    observed_ratio: observed,
                    target: slo.target,
                    met,
                }
            })
            .collect()
    }

    pub fn all_met(&self, evaluations: &[SloEvaluation]) -> bool {
        evaluations.iter().all(|e| e.met)
    }
}

// ─── Run Replay ───────────────────────────────────────────────────────────────

/// A single event in a run's causal timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEvent {
    pub sequence: usize,
    pub kind: String,
    pub payload: String,
    pub timestamp_secs: u64,
}

/// Replay log that can reconstruct the full causal run timeline.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunReplay {
    pub run_id: String,
    pub events: Vec<ReplayEvent>,
}

impl RunReplay {
    pub fn new(run_id: impl Into<String>) -> Self {
        Self {
            run_id: run_id.into(),
            events: Vec::new(),
        }
    }

    pub fn record(&mut self, kind: &str, payload: impl Into<String>) {
        let sequence = self.events.len();
        self.events.push(ReplayEvent {
            sequence,
            kind: kind.to_string(),
            payload: payload.into(),
            timestamp_secs: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        });
    }

    /// Reconstruct timeline as a human-readable string.
    pub fn reconstruct(&self) -> String {
        let mut lines = vec![format!("=== Run Replay: {} ===", self.run_id)];
        for ev in &self.events {
            lines.push(format!(
                "[{}] #{} {} — {}",
                ev.timestamp_secs, ev.sequence, ev.kind, ev.payload
            ));
        }
        lines.join("\n")
    }

    /// Write replay to a JSON file.
    pub fn persist(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, json)
    }
}

// ─── Incident Runbook ─────────────────────────────────────────────────────────

/// Incident severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// A single runbook entry mapping a failure scenario to remediation steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunbookEntry {
    pub scenario: String,
    pub severity: IncidentSeverity,
    pub detection: String,
    pub remediation_steps: Vec<String>,
}

/// Incident runbook covering top failure scenarios for autonomous_dev_ai.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentRunbook {
    pub entries: Vec<RunbookEntry>,
}

impl IncidentRunbook {
    /// Default runbook entries for top autonomous_dev_ai failure scenarios.
    pub fn default_runbook() -> Self {
        Self {
            entries: vec![
                RunbookEntry {
                    scenario: "Policy violation detected".to_string(),
                    severity: IncidentSeverity::High,
                    detection: "AuditEvent::SymbolicDecision with policy=deny, or PolicyViolation error".to_string(),
                    remediation_steps: vec![
                        "Inspect audit log for the violating action".to_string(),
                        "Confirm the PolicyEngine forbidden_patterns list is up-to-date".to_string(),
                        "If false positive, update policy pack version and re-sign".to_string(),
                        "Re-trigger the autonomous run after remediation".to_string(),
                    ],
                },
                RunbookEntry {
                    scenario: "Tool execution timeout".to_string(),
                    severity: IncidentSeverity::Medium,
                    detection: "ToolResult.success=false with timeout error in output".to_string(),
                    remediation_steps: vec![
                        "Check tool_timeout configuration (default 30 s)".to_string(),
                        "Investigate whether the underlying process is hung".to_string(),
                        "Increase timeout_seconds in agent config if workload is large".to_string(),
                        "Circuit-breaker will engage after 3 consecutive failures".to_string(),
                    ],
                },
                RunbookEntry {
                    scenario: "Circuit breaker open".to_string(),
                    severity: IncidentSeverity::High,
                    detection: "CircuitState::Open logged for a tool name in lifecycle metrics".to_string(),
                    remediation_steps: vec![
                        "Identify the tool whose circuit is open from the metrics log".to_string(),
                        "Fix or restart the underlying service/binary".to_string(),
                        "Wait for the circuit's timeout window to allow HalfOpen probe".to_string(),
                        "Monitor for successful HalfOpen → Closed transition".to_string(),
                    ],
                },
                RunbookEntry {
                    scenario: "Neural model drift detected".to_string(),
                    severity: IncidentSeverity::Medium,
                    detection: "DriftDetector::observe returns true; model auto-rolled-back".to_string(),
                    remediation_steps: vec![
                        "Inspect rolling confidence averages in the governance log".to_string(),
                        "Run offline evaluation on the affected model".to_string(),
                        "Re-promote model to canary only after evaluation passes".to_string(),
                        "Symbolic-only mode remains operational during model outage".to_string(),
                    ],
                },
                RunbookEntry {
                    scenario: "Agent stuck in non-terminal state".to_string(),
                    severity: IncidentSeverity::High,
                    detection: "Global timeout exceeded; state is not Done/Blocked/Failed".to_string(),
                    remediation_steps: vec![
                        "Check audit log for last state transition and tool execution".to_string(),
                        "Restore from latest checkpoint if available".to_string(),
                        "Manually set max_iterations lower to force termination".to_string(),
                        "File incident ticket with run replay artifact attached".to_string(),
                    ],
                },
            ],
        }
    }

    /// Look up runbook entries that match a keyword in the scenario description.
    pub fn lookup(&self, keyword: &str) -> Vec<&RunbookEntry> {
        let kw = keyword.to_ascii_lowercase();
        self.entries
            .iter()
            .filter(|e| e.scenario.to_ascii_lowercase().contains(&kw))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slo_evaluator_met() {
        let evaluator = SloEvaluator::new(SloEvaluator::default_slos());
        let mut obs = HashMap::new();
        obs.insert("run_success_rate".to_string(), 0.97);
        obs.insert("policy_violation_rate".to_string(), 1.0);
        obs.insert("iteration_latency_p95_secs".to_string(), 45.0);
        obs.insert("test_pass_rate".to_string(), 0.99);
        obs.insert("recovery_time_secs".to_string(), 90.0);

        let evals = evaluator.evaluate(&obs);
        assert!(evaluator.all_met(&evals), "all SLOs should be met");
    }

    #[test]
    fn test_slo_evaluator_breach() {
        let evaluator = SloEvaluator::new(SloEvaluator::default_slos());
        let mut obs = HashMap::new();
        obs.insert("run_success_rate".to_string(), 0.90); // below 0.95
        obs.insert("policy_violation_rate".to_string(), 1.0);
        obs.insert("iteration_latency_p95_secs".to_string(), 45.0);
        obs.insert("test_pass_rate".to_string(), 0.99);
        obs.insert("recovery_time_secs".to_string(), 90.0);

        let evals = evaluator.evaluate(&obs);
        assert!(!evaluator.all_met(&evals), "breach should be detected");
        let breached: Vec<_> = evals.iter().filter(|e| !e.met).collect();
        assert_eq!(breached.len(), 1);
        assert_eq!(breached[0].slo_name, "run_success_rate");
    }

    #[test]
    fn test_run_replay_reconstruct() {
        let mut replay = RunReplay::new("run-abc");
        replay.record("goal", "fix the bug");
        replay.record("plan", "explore -> patch -> verify");
        replay.record("action", "apply_patch");
        replay.record("result", "tests passed");

        let timeline = replay.reconstruct();
        assert!(timeline.contains("run-abc"));
        assert!(timeline.contains("goal"));
        assert!(timeline.contains("fix the bug"));
        assert_eq!(replay.events.len(), 4);
    }

    #[test]
    fn test_runbook_lookup() {
        let runbook = IncidentRunbook::default_runbook();
        let entries = runbook.lookup("policy");
        assert!(!entries.is_empty());
        assert!(entries[0].scenario.to_ascii_lowercase().contains("policy"));
    }
}
