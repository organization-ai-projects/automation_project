// projects/products/unstable/autonomy_orchestrator_ai/src/rollout_orchestrator.rs
use crate::domain::{RollbackDecision, RolloutPhase, RolloutStep};

const PHASES: &[RolloutPhase] = &[
    RolloutPhase::Canary,
    RolloutPhase::Partial,
    RolloutPhase::Full,
];

#[derive(Clone)]
pub struct RolloutConfig {
    pub enabled: bool,
    pub rollback_error_rate_threshold: f32,
    pub rollback_latency_threshold_ms: u64,
}

impl Default for RolloutConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            rollback_error_rate_threshold: 0.05,
            rollback_latency_threshold_ms: 5_000,
        }
    }
}

pub struct HealthSnapshot {
    pub error_rate: f32,
    pub latency_ms: u64,
}

pub struct RolloutOrchestrator {
    config: RolloutConfig,
    current_phase_index: usize,
}

impl RolloutOrchestrator {
    pub fn new(config: RolloutConfig) -> Self {
        Self {
            config,
            current_phase_index: 0,
        }
    }

    /// Advance through all phases, evaluating health at each one.
    ///
    /// Returns a pair of (rollout_steps, rollback_decision).
    /// If rollback is triggered the rollback_decision is `Some` and the steps
    /// up to (and including) the failing phase are returned.
    /// On healthy full progression rollback_decision is `None`.
    pub fn run(
        &mut self,
        snapshots: &[HealthSnapshot],
        timestamp_fn: &dyn Fn() -> u64,
    ) -> (Vec<RolloutStep>, Option<RollbackDecision>) {
        if !self.config.enabled {
            return (Vec::new(), None);
        }

        let mut steps: Vec<RolloutStep> = Vec::new();

        for (i, phase) in PHASES.iter().enumerate() {
            self.current_phase_index = i;

            // Evaluate health for this phase if a snapshot exists.
            if let Some(snapshot) = snapshots.get(i)
                && let Some(reason) = self.rollback_reason(snapshot)
            {
                let decision = RollbackDecision {
                    triggered_at_phase: *phase,
                    reason_code: reason,
                    timestamp_unix_secs: timestamp_fn(),
                };
                return (steps, Some(decision));
            }

            steps.push(RolloutStep {
                phase: *phase,
                reason_code: "ROLLOUT_PHASE_ADVANCED".to_string(),
                timestamp_unix_secs: timestamp_fn(),
            });
        }

        (steps, None)
    }

    fn rollback_reason(&self, snapshot: &HealthSnapshot) -> Option<String> {
        if snapshot.error_rate > self.config.rollback_error_rate_threshold {
            return Some("ROLLBACK_TRIGGER_ERROR_RATE".to_string());
        }
        if snapshot.latency_ms > self.config.rollback_latency_threshold_ms {
            return Some("ROLLBACK_TRIGGER_LATENCY".to_string());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ts() -> u64 {
        0
    }

    fn healthy() -> HealthSnapshot {
        HealthSnapshot {
            error_rate: 0.01,
            latency_ms: 100,
        }
    }

    #[test]
    fn disabled_rollout_returns_empty() {
        let mut orch = RolloutOrchestrator::new(RolloutConfig {
            enabled: false,
            ..Default::default()
        });
        let (steps, decision) = orch.run(&[], &ts);
        assert!(steps.is_empty());
        assert!(decision.is_none());
    }

    #[test]
    fn healthy_progression_advances_through_all_phases() {
        let mut orch = RolloutOrchestrator::new(RolloutConfig {
            enabled: true,
            rollback_error_rate_threshold: 0.05,
            rollback_latency_threshold_ms: 5_000,
        });
        let snapshots = vec![healthy(), healthy(), healthy()];
        let (steps, decision) = orch.run(&snapshots, &ts);

        assert!(decision.is_none());
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].phase, RolloutPhase::Canary);
        assert_eq!(steps[1].phase, RolloutPhase::Partial);
        assert_eq!(steps[2].phase, RolloutPhase::Full);
        assert!(
            steps
                .iter()
                .all(|s| s.reason_code == "ROLLOUT_PHASE_ADVANCED")
        );
    }

    #[test]
    fn error_rate_breach_at_canary_triggers_rollback() {
        let mut orch = RolloutOrchestrator::new(RolloutConfig {
            enabled: true,
            rollback_error_rate_threshold: 0.05,
            rollback_latency_threshold_ms: 5_000,
        });
        let snapshots = vec![
            HealthSnapshot {
                error_rate: 0.10,
                latency_ms: 100,
            },
            healthy(),
            healthy(),
        ];
        let (steps, decision) = orch.run(&snapshots, &ts);

        let decision = decision.expect("rollback should be triggered");
        assert_eq!(decision.reason_code, "ROLLBACK_TRIGGER_ERROR_RATE");
        assert_eq!(decision.triggered_at_phase, RolloutPhase::Canary);
        assert!(
            steps.is_empty(),
            "no phase should be advanced before rollback"
        );
    }

    #[test]
    fn latency_breach_at_partial_triggers_rollback() {
        let mut orch = RolloutOrchestrator::new(RolloutConfig {
            enabled: true,
            rollback_error_rate_threshold: 0.05,
            rollback_latency_threshold_ms: 5_000,
        });
        let snapshots = vec![
            healthy(),
            HealthSnapshot {
                error_rate: 0.01,
                latency_ms: 6_000,
            },
            healthy(),
        ];
        let (steps, decision) = orch.run(&snapshots, &ts);

        let decision = decision.expect("rollback should be triggered");
        assert_eq!(decision.reason_code, "ROLLBACK_TRIGGER_LATENCY");
        assert_eq!(decision.triggered_at_phase, RolloutPhase::Partial);
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].phase, RolloutPhase::Canary);
    }

    #[test]
    fn error_rate_exactly_at_threshold_does_not_trigger_rollback() {
        let mut orch = RolloutOrchestrator::new(RolloutConfig {
            enabled: true,
            rollback_error_rate_threshold: 0.05,
            rollback_latency_threshold_ms: 5_000,
        });
        let snapshots = vec![
            HealthSnapshot {
                error_rate: 0.05,
                latency_ms: 100,
            },
            healthy(),
            healthy(),
        ];
        let (steps, decision) = orch.run(&snapshots, &ts);
        assert!(decision.is_none());
        assert_eq!(steps.len(), 3);
    }

    #[test]
    fn latency_exactly_at_threshold_does_not_trigger_rollback() {
        let mut orch = RolloutOrchestrator::new(RolloutConfig {
            enabled: true,
            rollback_error_rate_threshold: 0.05,
            rollback_latency_threshold_ms: 5_000,
        });
        let snapshots = vec![
            HealthSnapshot {
                error_rate: 0.01,
                latency_ms: 5_000,
            },
            healthy(),
            healthy(),
        ];
        let (steps, decision) = orch.run(&snapshots, &ts);
        assert!(decision.is_none());
        assert_eq!(steps.len(), 3);
    }

    #[test]
    fn no_snapshots_advances_all_phases() {
        let mut orch = RolloutOrchestrator::new(RolloutConfig {
            enabled: true,
            rollback_error_rate_threshold: 0.05,
            rollback_latency_threshold_ms: 5_000,
        });
        let (steps, decision) = orch.run(&[], &ts);
        assert!(decision.is_none());
        assert_eq!(steps.len(), 3);
    }
}
