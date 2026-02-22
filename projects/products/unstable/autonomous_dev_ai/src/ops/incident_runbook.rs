// projects/products/unstable/autonomous_dev_ai/src/ops/incident_runbook.rs
use serde::{Deserialize, Serialize};

use crate::ops::{IncidentSeverity, run_book_entry::RunbookEntry};

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
                    detection:
                        "AuditEvent::SymbolicDecision with policy=deny, or PolicyViolation error"
                            .to_string(),
                    remediation_steps: vec![
                        "Inspect audit log for the violating action".to_string(),
                        "Confirm the PolicyEngine forbidden_patterns list is up-to-date"
                            .to_string(),
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
                    detection: "CircuitState::Open logged for a tool name in lifecycle metrics"
                        .to_string(),
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
                    detection: "DriftDetector::observe returns true; model auto-rolled-back"
                        .to_string(),
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
                    detection: "Global timeout exceeded; state is not Done/Blocked/Failed"
                        .to_string(),
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
