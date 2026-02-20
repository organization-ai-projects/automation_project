//projects/products/unstable/autonomous_dev_ai/src/ops/slo_evaluator.rs
use std::collections::HashMap;

use crate::ops::{Slo, SloEvaluation};

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
                let observed = observations.get(&slo.sli.name).copied().unwrap_or(0.0);
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
